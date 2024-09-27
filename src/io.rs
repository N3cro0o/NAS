use std::io;
use crate::user::User;
use super::place::Place;


pub struct LoginData(String, String);

impl LoginData {
    pub fn login(&self) -> String {
            self.0.clone()
    }

    pub fn password(&self) -> String {
        self.1.clone()
    }
}

pub fn get_name_and_pass_from_console(repeat_password: bool) -> LoginData
    // Write error handling
{
    let mut input = String::new();
    let login;
    let pass;
    println!("Please enter login and password:\nLogin: ");
    io::stdin().read_line(&mut input).expect("Wrong data input");
    login = String::from(input.trim());
    input.clear();
    println!("Password: ");
    io::stdin().read_line(&mut input).expect("Wrong data input");
    pass = String::from(input.trim());
    input.clear();
    if repeat_password {
        println!("Repeat password: ");
        io::stdin().read_line(&mut input).expect("Wrong data input");
    }
    LoginData(login, pass)
}

pub fn get_place_creation_data_from_console() -> String 
    // After adding more stuff to place struct and shit add them here too
{
    let mut input = String::new();
    let name;
    println!("Please enter place creation data:\nPlace name: ");
    io::stdin().read_line(&mut input).expect("Wrong data input");
    name = String::from(input.trim());
    name
    }

pub fn sent_message(user: &User, place: &Place, message: &str){
    println!("User {} said in {}: {}", user.name(), place.name, message);
}