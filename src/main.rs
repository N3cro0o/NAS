/*

To Do:
1. User can send messages in a server - done
2. Store messages in server - done
3. Main menu (loop) - done
4. Log in and log off - done
5. Place roles
6. Segregate modules and comment code
7. Saving server data
8. Desktop app
9. BANNING
10. More place functions (see members, see messages, see options)
11. User and place config
12. Default, empty current place, behaviour
13. Responsive text menu
14. PlaceMessage date formatting

*/


use nas::Simulation;
use nas::io;
use std::rc::Rc;

fn main() {
    // Start simulation
    let mut sim = Simulation::new();

    // Build first user
    let login_tuple = io::get_name_and_pass_from_console(true);
    sim.create_user(login_tuple.login(), login_tuple.password());
    let user = match sim.get_user_by_id(0){
        Ok(x) => x,
        Err(err) => panic!("{err}")
    };
    // Build first place
    let place_string = io::get_place_creation_data_from_console();
    sim.create_place(place_string, Rc::clone(&user));
    let _place = match sim.get_place_by_id(0){
        Ok(x) => x,
        Err(err) => panic!("{err}")
    };

    // Creation of some users
    sim.create_user("Madman".to_string(), "Jonni".to_string());
    sim.create_user("Femboy".to_string(), "Piofli".to_string());
    sim.create_user("The Forgotten One".to_string(), "Diat".to_string());

    // Create another server
    sim.create_place("Debug".to_string(), sim.get_user_by_id(2).unwrap());

    // App loop
    'app: loop {
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
        // Menu what to do
        let cur_user = match sim.return_current_user() {
            Some(x) => x,
            None => {continue 'app;}
        };
        println!("Hello {}! Please pick what you want to do:", cur_user.borrow().name);
        println!("1. Log off\n2. Change current place");
        println!("10. Send message in current place.");

        // User input
        let mut input = String::new();
        let user_option: usize;
        std::io::stdin().read_line(&mut input).expect("Something");
        user_option = input.trim().parse().expect("Should be an integer");
        input.clear();
        dbg!(&user_option);
        
        match user_option {
            1 => {
                sim.log_off();
            }
            2 => {
                println!("Give place id.");
                std::io::stdin().read_line(&mut input).expect("Something");
                let server_id: u64 = input.trim().parse().expect("Should be an integer");
                let server = match sim.get_place_by_id(server_id) {
                    Ok(x) => x,
                    Err(err) => {
                        println!("{err}");
                        continue 'app;
                    }
                };
                sim.change_place(&server);
            }
            10 => {
                sim.send_message("It's a test message");
            }

            _ => {
                println!("Wrong input.");
                break 'app;
            }
        };
        
    }
    dbg!(sim.get_place_by_id(0).unwrap());
}