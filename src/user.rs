use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::place::Place;


#[derive(Debug)]
pub struct User{
    pub name: String,
    pass: String,
    id: u64,
    pub place: Weak<RefCell<Place>>,
    pub data: UserData
}

impl User {
    pub fn new(name: String, pass: String, id: u64) -> User{
        User {
            name,
            pass,
            id,
            place: Weak::new(),
            data: UserData::new(String::new(), String::new(), String::new())
        }
    }

    pub fn pass(&self) -> String{
        self.pass.clone()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        if self.id == other.id {
            true
        }
        else {
            false
        }
    }
}

#[derive(Debug)]
pub struct UserData {
    pass: String,
    login: String,
    email: String,
    friends: Vec<Rc<RefCell<User>>>
}

impl UserData {
    pub fn new(pass: String, login: String, email: String) -> UserData {
        UserData {
            pass,
            login,
            email,
            friends: vec![]
        }
    }
}