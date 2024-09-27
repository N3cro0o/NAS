/*

To Do:
1. User can send messages in a server - done
2. Store messages in server - done
3. Main menu (loop) - done
4. Log in and log off - done
5. Place roles
6. Comment code
7. Saving server data 
8. Desktop app
9. BANNING - done
10. More place functions (see members, see messages, see options) - done done notDone
11. User and place config
12. Default, empty current place, behaviour
13. Responsive text menu
14. PlaceMessage date formatting

*/


use nas::place::roles;
use nas::place::roles::RoleTrait;
use nas::Simulation;
use nas::io;

fn main() {
    // Start simulation
    let mut sim = Simulation::new();

    // Build first user
    let login_tuple = io::get_name_and_pass_from_console(true);
    sim.create_user(login_tuple.login(), login_tuple.password());
    let user = match sim.get_user_by_id(1){
        Ok(x) => x,
        Err(err) => panic!("{err}")
    };
    // Build first place
    let place_string = io::get_place_creation_data_from_console();
    sim.create_place(place_string, user.borrow().id());
    let _place = match sim.get_place_by_id(0){
        Ok(x) => x,
        Err(err) => panic!("{err}")
    };

    // Creation of some users
    sim.create_user("Madman".to_string(), "Jonni".to_string());
    sim.create_user("Femboy".to_string(), "Piofli".to_string());
    sim.create_user("The Forgotten One".to_string(), "Diat".to_string());

    // Create another place
    sim.create_place("Debug".to_string(), 2);



    // App loop
    'app: loop {
        println!("-------------------------------------");
        // Check if logged
        let logged = sim.logged();
        if !logged {
            let login_tuple = io::get_name_and_pass_from_console(false);
            match sim.log_in(login_tuple.login(), login_tuple.password()) {
                Ok(s) => {println!("{s}")},
                Err(err) => {
                    println!("{err}\n\n");
                    continue 'app;
                }
            };
        }

        // Pre menu logic
        let cur_user = match sim.return_current_user() {
            Some(x) => x,
            None => {continue 'app;}
        };
        let cur_place = sim.return_current_place();

        let admin: bool = sim.is_admin(cur_user.borrow().id());

        // Load perms
        let perms = sim.return_current_user_perms();

        // Check if banned
        if cur_user.borrow().place.borrow().is_banned(cur_user.borrow().id()) {
            println!("You are banned, get lost.");
            sim.reset_place();
        }

        // Menu what to do
        println!("Hello {}! Please pick what you want to do:", cur_user.borrow().name());
        println!("1. Log off\n2. Change current place\n3. Print messages");
        println!("10. Send message in current place.\n11. Show users");

        // Menu for admin
        if admin {
            println!("It seems you are an admin of this place too! Nice :>\n60. Ban user\n61. Mute user")
        }
        // User input
        let mut input = String::new();
        let user_option: usize;
        std::io::stdin().read_line(&mut input).expect("Something");
        user_option = input.trim().parse().expect("Should be an integer");
        input.clear();
        
        match user_option {
            1 => {
                sim.log_off();
            }

            2 => {
                println!("Give place id.");
                std::io::stdin().read_line(&mut input).expect("Something");
                let server_id: u64 = input.trim().parse().expect("Should be an integer");
                if let Some(x) = sim.change_place(server_id) {
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

            10 => {
                if perms.can_talk {
                    sim.send_message("It's a test message");
                }
                else {
                    println!("You are muted. XDDD");
                }
            }

            11 => {
                println!("Member list of {}", cur_place.borrow().name);
                for members in cur_place.borrow().members.iter() {
                    if admin {
                        print!("Id: {}, ", members.user.upgrade().unwrap().borrow().id());
                    }
                    println!("{}", members.user.upgrade().unwrap().borrow());
                }
            }

            12 => {
                println!("Role list of {}", cur_place.borrow().name);
                let mut i = 0;
                for roles in cur_place.borrow().return_role_vec().iter() {
                    println!("{}. {}", i, roles.name);
                    i += 1;
                }
            }

            0 => {
                println!("Wrong input.");
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
                let mut role = roles::RoleTemplate::new("Muted".to_string(), 10);
                role.update_perms(roles::RolePerms::new(false, 10));
                cur_place.borrow_mut().add_role(role);
                println!("Mute role added");
            }

            62 => {
                if !admin {continue 'app;}
                println!("Give user id.");
                std::io::stdin().read_line(&mut input).expect("Something");
                let user_to_mute_id: u64 = input.trim().parse().expect("Should be an integer");
                let r = match cur_place.borrow().find_role_by_id(2){
                    Some(role) => role.clone(),
                    None => {continue 'app;}
                };
                // NEVER USE
                // cur_place.borrow_mut().grant_role(user_to_mute_id, cur_place.borrow().find_role_by_id(2));
                // You will fucking DIE!!!!!!!
                // really... trust me bro
                cur_place.borrow_mut().grant_role(user_to_mute_id, r);
            }
            _ => {
                continue 'app;
            }
        };
    }
}