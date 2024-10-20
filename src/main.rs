/*
To Do:
1. User can send messages in a server - done
2. Store messages in server - done
3. Main menu (loop) - done
4. Log in and log off - done
5. Place roles - done
6. Comment code
7. Saving server data 
8. Desktop app
9. BANNING - done
10. More place functions (see members, see messages, see options) - done done notDone
11. User and place config
12. Default, empty current place, behaviour
13. Responsive text menu
14. PlaceMessage date formatting
15. Implement Debug for Simulation struct
*/
use nas::place::roles;
use nas::place::roles::RoleTrait;
use nas::Simulation;
use nas::io;


fn main() {
    // Start simulation
    let mut sim = Simulation::new();

    // uncomment for app
    //eframe::run_native("NAS", sim.return_options(), Box::new(|cc| Ok(Box::new(sim))));

    // Build first user
    let login_tuple = io::get_name_and_pass_from_console(true).unwrap();
    sim.create_user(login_tuple.login(), login_tuple.password());
    let user = match sim.get_user_by_id(1){
        Ok(x) => x,
        Err(err) => panic!("{err}")
    };
    // Build first place
    let place_string = io::get_place_creation_data_from_console();
    sim.create_place(place_string, user.borrow().id());
    let _place = match sim.get_place_by_id(1){
        Ok(x) => x,
        Err(_) => {panic!("Cannot find place")}
    };

    // Creation of some users
    sim.create_user("Madman".to_string(), "Jonni".to_string());
    sim.create_user("Femboy".to_string(), "Piofli".to_string());
    sim.create_user("The Forgotten One".to_string(), "Diat".to_string());

    // Create another place
    sim.create_place("Debug".to_string(), 2);
    let mut input = String::new();
    let mut user_option: usize;

    // App loop
    'app: loop {
        dbg!(&sim.logged());
        input.clear();
        println!("-------------------------------------");
        // Check if logged
        let logged = sim.logged();
        if !logged {
            println!("Please log in! Do this by pressing 1.\nIf you want to register, press 2.\nAny not described input closes app.");
            std::io::stdin().read_line(&mut input).expect("Something");
            user_option = input.trim().parse().expect("Should be an integer");
            input.clear();
            match user_option {
                1 => {
                    let login_tuple = io::get_name_and_pass_from_console(false).unwrap();
                    match sim.log_in(login_tuple.login(), login_tuple.password()) {
                        Ok(s) => {println!("{s}")},
                        Err(err) => {
                            println!("{err}\n");
                            continue 'app;
                        }
                    };
                }

                2 => {
                    'register: loop {
                        let login_tuple = match io::get_name_and_pass_from_console(true) {
                            Ok(x) => x,
                            Err(_) =>{
                                println!("You buffoon, do it again");
                                continue 'register;
                            }
                        };
                        sim.create_user(login_tuple.login(), login_tuple.password());
                        match sim.log_in(login_tuple.login(), login_tuple.password()) {
                            Ok(s) => {println!("{s}")},
                            Err(err) => {
                                println!("{err}\n");
                            }
                        };
                        continue 'app;
                    }
                }

                _ =>{
                    break 'app;
                }
            }
        }

        // Pre menu logic
        let cur_user = match sim.return_current_user() {
            Some(x) => x,
            None => {continue 'app;}
        };
        let curr_place_id = cur_user.borrow().place;
        let admin: bool = sim.is_admin(cur_user.borrow().id());

        // Load perms
        let perms = sim.return_current_user_perms();

        // Check if banned
        {
            let place_id = cur_user.borrow().place;
            match sim.get_place_by_id(place_id) {
                Ok(place) => {
                    if place.borrow().is_banned(cur_user.borrow().id()) {
                        println!("You are banned, get lost.");
                        sim.reset_place();
                    }
                }
                Err(_def) => {
                    // Add ban in default place 
                }
            }
        }
        // Menu what to do
        println!("Hello {}! Please pick what you want to do:", sim.return_curr_user_name());
        println!("1. Log off\n2. Change current place\n3. Print messages\n4. Check your roles");
        println!("5. Change nickname");
        if perms.change_nickname {
            println!("6. Change server nickname");
        }
        println!("10. Send message in current place.\n11. Show users");

        // Menu for admin
        if admin {
            println!("It seems you are an admin of this place too! Nice :>\n60. Ban user\n61. Create role\n62. Add role to user");
            println!()
        }
        // User input
        std::io::stdin().read_line(&mut input).expect("Something");
        user_option = input.trim().parse().expect("Should be an integer");
        input.clear();
        
        // Describe each arm, just few words will be enough
        match user_option {
            1 => {
                sim.log_off();
            }

            2 => {
                println!("Give place id.");
                std::io::stdin().read_line(&mut input).expect("Something");
                let server_id: u64 = input.trim().parse().expect("Should be an integer");
                if let Err(x) = sim.change_place(server_id) {
                    println!("{x}");
                    continue 'app;
                }
            }
            
            3 => {
                let mesg_vec = sim.return_current_place_messages();
                for s in mesg_vec{
                    println!("{s}");
                }
            }

            4 => {
                let role_vec = sim.return_current_place_user_roles(user.borrow().id()).unwrap();
                for role in role_vec.iter() {
                    println!("{}", role.name);
                }
            }

            5 => {
                println!("Give new nickname:");
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Invalid input value");
                let user_id = user.borrow().id();
                if let Err(err) = sim.change_nick(user_id, input.trim()) {
                    println!("{err}");
                }
            }

            6 => {
                println!("Give new nickname:");
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Invalid input value");
                let user_id = user.borrow().id();
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) => {
                        if let Err(err) = place.borrow_mut().change_user_nickname(user_id, input.trim().to_string()) {
                            println!("{err}");
                        }
                    }
                    Err(_) => {println!("Cannot change nickname in default place")}
                }
            }
            
            10 => {
                if perms.can_talk {
                    sim.send_message("It's a test message");
                }
                else {
                    println!("You are muted. XDDD");
                }
            }

            11 => {
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) => {
                        println!("Member list of {}", place.borrow().name);
                        for members in place.borrow().members.iter() {
                            if admin {
                                print!("Id: {}, ", members.user().upgrade().unwrap().borrow().id());
                            }
                            println!("{}", members.user().upgrade().unwrap().borrow());
                        }
                    }
                    Err(_) => {
                        println!("Make group logic");
                    }
                }
            }

            12 => {
                if let Ok(place) = sim.get_place_by_id(curr_place_id) {
                    println!("Role list of {}", place.borrow().name);
                    let mut i = 0;
                    for roles in place.borrow().return_role_vec().iter() {
                        println!("{}. {}", i, roles.name);
                        i += 1;
                    }
                }
            }

            0 => {
                println!("Sayonara, nerd.");
                break 'app;
            }
            
            // Admin stuff
            60 => {
                if !admin {continue 'app;}
                println!("Give user id.");
                std::io::stdin().read_line(&mut input).expect("Something");
                let user_to_ban_id: u64 = input.trim().parse().expect("Should be an integer");
                match sim.ban_user(user_to_ban_id) {
                    Ok(b) => {
                        if b {
                            println!("User banned! debil.");
                        }
                        else {
                            println!("User isn't banned! strange...");
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                };
            }
            61 => {
                if !admin {continue 'app;}
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) => {
                        println!("Give new role name and priority");

                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Not valid input");
                        let role_name = input.trim().to_string();

                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Non valid input");
                        let priority = input.trim().parse::<u8>().unwrap();
                        let mut role = roles::RoleTemplate::new(role_name, priority);

                        println!("Now, PERMS!\nCan they send messages? Please use yes/y or no/n");
                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Non valid input");
                        let mut message = false;
                        if input.trim().to_lowercase() == "yes" || input.trim().to_lowercase() == "y" {
                            message = true;
                        }

                        println!("Can they change their nickname? Please use yes/y or no/n");
                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Non valid input");
                        let mut nick = false;
                        if input.trim().to_lowercase() == "yes" || input.trim().to_lowercase() == "y" {
                            nick = true;
                        }

                        role.update_perms(roles::RolePerms::new(message, nick, priority));
                        place.borrow_mut().add_role(role);
                        println!("Mute role added");
                    }

                    Err(_) => {println!("Cannot do it in default place")}
                }
            }

            62 => {
                if !admin {continue 'app;}
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) =>{
                        println!("Give user id.");
                        std::io::stdin().read_line(&mut input).expect("Something");
                        let user_to_mute_id: u64 = input.trim().parse().expect("Should be an integer");
                        let r = match place.borrow().find_role_by_id(2){
                            Some(role) => role.clone(),
                            None => {continue 'app;}
                        };
                        // NEVER USE
                        // place.borrow_mut().grant_role(user_to_mute_id, place.borrow().find_role_by_id(2));
                        // You will fucking DIE!!!!!!!
                        // really... trust me bro
                        place.borrow_mut().grant_role(user_to_mute_id, r);
                    }

                    Err(_) => {println!("Cannot do it in default place")}
                }
            }

            2137 => {
                match open::that("https://www.youtube.com/watch?v=wP8OA3Qdlhw"){
                    Ok(_) => (),
                    Err(err) => {println!("{err}");}
                };
            }
            _ => {
                continue 'app;
            }
        };
    }
}