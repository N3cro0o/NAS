use std::{fmt::Display, cell::RefCell, rc::{Rc, Weak}, time::SystemTime};
use chrono::{DateTime, Local};
use super::user::User;

#[derive(Debug)]
pub struct Place {
    pub name: String,
    id: u64,
    pub members: Vec<Rc<RefCell<User>>>,
    pub admin: Rc<RefCell<User>>,
    pub messages: Vec<PlaceMessage>
}

impl Place {
    pub fn new(name: String, admin: Rc<RefCell<User>>, id: u64) -> Place {
        Place{
            name,
            members: vec![],
            admin,
            id,
            messages: vec![]
        }
    }

    pub fn add_user(&mut self, user: Rc<RefCell<User>>){
        self.members.push(user);
    }

    pub fn add_message(&mut self, message: PlaceMessage) {
        self.messages.push(message);
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub struct PlaceMessage {
    user: Weak<RefCell<User>>,
    message: String,
    time: SystemTime
}

impl PlaceMessage {
    pub fn new(user: &Rc<RefCell<User>>, message: String, time: SystemTime) -> PlaceMessage {
        PlaceMessage {
            user: Rc::downgrade(user),
            message,
            time
        }
    }
}

impl Display for PlaceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:\n{}", self.user.upgrade().unwrap().borrow().name, DateTime::<Local>::from(self.time), self.message)
    }
}