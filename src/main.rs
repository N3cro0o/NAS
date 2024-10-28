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
15. Implement Debug for Simulation struct ??????????
16. Change everywhere to Rc::clone() in methods ???
17. Create new places - done
18. Create new groups - done
19. Add friends - done
20. Find users by login
21. Places 'logins' - done
22. Save already joined places and groups
23. Check if joining default groups
24. Let change group and place names
25. Email verification
*/
use nas::place::roles::{self, RoleTrait};
use nas::user::User;
use nas::Simulation;
use nas::io;
use std::{rc::Rc, cell::RefCell};

fn main() {
    // Start simulation
    let mut sim = Simulation::new();

    // uncomment for app
    //eframe::run_native("NAS", sim.return_options(), Box::new(|cc| Ok(Box::new(sim))));

    let mut input = String::new();
    let mut user_option: usize;

    // App loop ----------------------------------------------------------------------------------------------------------------------------------------------------
    'app: loop {
        input.clear();
        // Check if logged
        let logged = sim.logged();
        if !logged {
            println!("Please log in! Do this by pressing 1.\nIf you want to register, press 2.\nAny not described input closes app.");
            std::io::stdin().read_line(&mut input).expect("Something");
            user_option = input.trim().parse().expect("Should be an integer");
            input.clear();
            match user_option {
                1 => {
                    let login_tuple = io::get_logging_data_from_console();
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
                        if let Err(str) = sim.create_user(login_tuple.login(), login_tuple.email(), login_tuple.password()) {
                            println!("{str}");
                            continue 'register;
                        }
                        match sim.log_in(login_tuple.login(), login_tuple.password()) {
                            Ok(s) => {println!("{s}")},
                            Err(err) => {
                                println!("{err}\n");
                            }
                        };
                        continue 'app;
                    }
                }

                0 =>{
                    break 'app;
                }

                _ => {}
            }
        }

        // Pre menu logic
        let curr_user = match sim.return_current_user() {
            Some(x) => x,
            None => {continue 'app;}
        };
        let curr_place_id = curr_user.borrow().place;
        let curr_place_name = match sim.get_place_by_id(curr_place_id) {
            Ok(place) => place.borrow().name(),
            Err(_def) => String::from("Default place")
        };

        let curr_group_id = curr_user.borrow().return_def_group();

        let admin: bool = sim.is_admin(curr_user.borrow().id());

        // Load perms
        let perms = sim.return_current_user_perms();

        // Check if banned
        {
            let place_id = curr_user.borrow().place;
            match sim.get_place_by_id(place_id) {
                Ok(place) => {
                    if place.borrow().is_banned(curr_user.borrow().id()) {
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
        
        println!("---------------------------------------------------------------------------------------------------------------");
        println!("Hello {} in {}! Please pick what you want to do:", sim.return_curr_user_name(), curr_place_name);
        println!("1. Log off\n2. Change current place\n3. Change nickname\n4. Show users\n5. Show groups\n6. Show places\n7. Add friend");
        println!("8. Return friends list\n9. Create new place");
        if curr_place_id != 0 {
            println!("10. Send message in current place.\n11. Print messages\n12. Check your roles\n13. Print all roles");
            if perms.change_nickname {
                println!("14. Change server nickname");
            }
            println!("15. Get invite code");
            // Menu for admin
            if admin {
                println!("It seems you are an admin of this place too! Nice :>\n60. Change place name\n61. Ban user\n62. Create role\n63. Add role to user");
            }
        }
        else {
            println!("10. Send message in current group.\n11. Print messages\n12. Change groups\n13. Add members to current group\n14. Create new group")

        }
        
        // User input
        std::io::stdin().read_line(&mut input).expect("Something");
        user_option = input.trim().parse().expect("Should be an integer");
        input.clear();
        
        // Debug prints
        // dbg!(&curr_user.borrow().name());
        // dbg!(&curr_user.borrow().data);

        // Describe each arm, just few words will be enough
        match user_option {
            0 => {
                println!("Sayonara, nerd.");
                break 'app;
            }

            1 => { // Log off
                sim.log_off();
            }

            2 => { // Change place
                println!("Id or code? Please type");
                std::io::stdin().read_line(&mut input).expect("Something");
                if input.trim().to_lowercase() == "id" {
                    input.clear();
                    println!("Give place id.");
                    std::io::stdin().read_line(&mut input).expect("Something");
                    let server_id: u64 = input.trim().parse().expect("Should be an integer");
                    if let Err(x) = sim.change_place(server_id) {
                        println!("{x}");
                        continue 'app;
                    }
                }
                else if input.trim().to_lowercase() == "code" {
                    input.clear();
                    println!("Give place code.");
                    std::io::stdin().read_line(&mut input).expect("Something");
                    if let Err(x) = sim.change_place_code(input.clone()) {
                        println!("{x}");
                        continue 'app;
                    }
                }
            }

            3 => { // Change user nickname
                println!("Give new nickname:");
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Invalid input value");
                let user_id = curr_user.borrow().id();
                if let Err(err) = sim.change_nick(user_id, input.trim()) {
                    println!("{err}");
                }
            }

            4 => { // Return members of place / group
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) => {
                        println!("Member list of {}", place.borrow().name());
                        for member in place.borrow().members.iter() {
                            if admin {
                                print!("Id: {}, ", member.user().upgrade().unwrap().borrow().id());
                            }
                            println!("{}", member.user().upgrade().unwrap().borrow());
                        }
                    }
                    Err(def) => {
                        for member in def.find_group(curr_group_id).unwrap().return_members_vec().iter() {
                            println!("{}", member.borrow());
                        }
                    }
                }
            }

            5 => { // Show joined groups' ids
                let vec = curr_user.borrow().data.return_groups().clone();
                let mut invalid_groups = 0;

                // Print user def group
                let def_id = curr_user.borrow().return_def_group();
                println!("Default group {} with id: {}", sim.get_default_place().find_group(def_id).unwrap().name(), def_id);

                for group_id in vec.iter() {
                    let group = match sim.get_default_place().find_group(*group_id) {
                        Some(g) => g,
                        None => {
                            invalid_groups += 1;
                            continue;
                        }
                    };
                    // Skip default groups
                    if group.is_default() {
                        invalid_groups += 1;
                        continue;
                    }
                    println!("Group {} with id: {}", group.name(), group_id)
                }
                println!("Invalid groups found: {}", invalid_groups - 1); // - 1 groups because one default belongs to user!
            }

            6 => {  // Show joined places' ids
            let vec = curr_user.borrow().data.return_places().clone();
            let mut invalid_places = 0;

            for place_id in vec.iter() {
                let place = match sim.get_place_by_id(*place_id) {
                    Ok(g) => g,
                    Err(_) => {
                        invalid_places += 1;
                        continue;
                    }
                };
                println!("Place {} with id: {}", place.borrow().name(), place_id)
            }
            println!("Invalid places found: {}", invalid_places - 1); // - 1 groups because one default belongs to user!
            }

            7 => { // Add new friend
                println!("Give friend id");
                std::io::stdin().read_line(&mut input).expect("Something");
                let friend_id = match input.trim().parse::<u64>() {
                    Ok(x) => x,
                    Err(_) => {println!{"Cancel operation"}; continue 'app;}
                };
                let friend = sim.get_user_by_id(friend_id).unwrap();
                curr_user.borrow_mut().data.add_friend(friend);
            }
            
            8 => { // Return friends list
                let friend_list = curr_user.borrow().data.return_friends_list_iter();
                for friend in friend_list.iter() {
                    println!("{}", friend.borrow());
                }
                
            }

            9 => { // Create new place
                let place_string = io::get_place_creation_data_from_console();
                let new_place_id = match sim.create_place(place_string, curr_user.borrow().id()) {
                    Ok(x) => x,
                    Err(err) => {
                        println!("{err}"); continue 'app;
                    }
                };
                if let Err(_) = sim.change_place(new_place_id) {} // we don't have to check, it's freshly created
            }

            10 => { // Send message in place
                input.clear();
                println!("Type your message:");
                std::io::stdin().read_line(&mut input).expect("Invalid input value");
                if !perms.can_talk {
                    println!("You are muted. XDDD");
                    continue 'app;
                }
                sim.send_message(input.trim());
            }

            11 => { // Return messages in place / group
                let mesg_vec = sim.return_current_place_messages();
                for s in mesg_vec{
                    println!("{s}");
                }
            }

            12 => { // Return user place roles / Change groups
                let role_vec = sim.return_current_place_user_roles(curr_user.borrow().id()).unwrap();
                for role in role_vec.iter() {
                    println!("{}", role.name);
                }
            }

            13 => { // Return all place roles / Add members to groups
                if curr_place_id != 0 {
                    if let Ok(place) = sim.get_place_by_id(curr_place_id) {
                        println!("Role list of {}", place.borrow().name());
                        let mut i = 0;
                        for roles in place.borrow().return_role_vec().iter() {
                            println!("{}. {}", i, roles.name);
                            i += 1;
                        }
                    }
                }
                else {
                    println!("User login or id? Please type login or id.");
                    input.clear();
                    std::io::stdin().read_line(&mut input).expect("Random message");
                    if input.trim().to_lowercase() == "id" {
                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Tracę włosy, tracę głos");
                        let id = input.trim().parse::<u64>().unwrap();
                        let user = sim.get_user_by_id(id).unwrap_or_else(|_|
                             -> std::rc::Rc<std::cell::RefCell<nas::user::User>> { // fuck me
                                return std::rc::Rc::clone(&curr_user)
                        });
                        add_to_group(&mut sim, curr_user, user);
                    }
                    else if input.trim().to_lowercase() == "login" {
                        input.clear();
                        std::io::stdin().read_line(&mut input).expect("Starość mnie nie dotknie!");
                        let user = sim.get_user_by_login(input.trim()).unwrap_or_else(|_|
                            -> std::rc::Rc<std::cell::RefCell<nas::user::User>> { // fuck me
                               return std::rc::Rc::clone(&curr_user)
                        });
                        add_to_group(&mut sim, curr_user, user);
                    }
                }
            }

            14 => { // Change server nickname / Create new group
                if curr_place_id == 0 {
                    println!("Please give new group's name:");
                    input.clear();
                    std::io::stdin().read_line(&mut input).expect("Something");
                    sim.create_new_group(vec![curr_user], input.trim().to_string());

                    continue 'app;
                }
                println!("Give new nickname:");
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Invalid input value");
                let user_id = curr_user.borrow().id();
                match sim.get_place_by_id(curr_place_id) {
                    Ok(place) => {
                        if let Err(err) = place.borrow_mut().change_user_nickname(user_id, input.trim().to_string()) {
                            println!("{err}");
                        }
                    }
                    Err(_) => {println!("Cannot change nickname in default place")}
                }
            }

            15 => { // Get invite code
                if curr_place_id == 0 {continue 'app;}
                if let Ok(place) = sim.get_place_by_id(curr_place_id) {
                    println!("Code: {}", place.borrow().return_invite_infinite_code());
                }
            }
            
            // Admin stuff
            60 => {
                if !admin {continue 'app;}
                println!("Give new place name:");
                input.clear();
                std::io::stdin().read_line(&mut input).expect("Something");
                input = input.trim().to_string();
                sim.get_place_by_id(curr_place_id).unwrap().borrow_mut().change_name(input.clone());
            }

            61 => {
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
            62 => {
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

            63 => {
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

            // funni
            2137 => {
                match open::that("https://www.youtube.com/watch?v=wP8OA3Qdlhw"){
                    Ok(_) => (),
                    Err(err) => {println!("{err}");}
                };
            }
            _ => {}
        };
    }
}

fn add_to_group(sim: &mut Simulation, curr: Rc<RefCell<User>>, target: Rc<RefCell<User>>) {
    if let Ok(()) = sim.add_user_to_current_group(Rc::clone(&target)) {
        return
    }
    let vec = vec![curr, target];
    println!("Creating new group with: {:?}", vec);
    let mut s = String::new();
    for us in vec.iter() {
        let name = us.borrow().name();
        s.push_str(&name);
        s.push_str(", ");
    }
    s.remove(s.len() - 1);
    s.remove(s.len() - 1);
    sim.create_new_group(vec, s);
}