use std::{cell::RefCell, fmt::Display, rc::Rc};


#[derive(Debug)]
pub struct User{
    name: String,
    id: u64,
    pub place: u64,
    pub data: UserData
}

impl User {
    pub fn new(name: String, pass: String, id: u64, place: u64) -> User{
        User {
            name: String::new(),
            id,
            place,
            data: UserData::new(name, pass, String::new())
        }
    }

    pub fn name(&self) -> String {
        if !self.name.is_empty() {self.name.clone()}
        else {self.data.login.clone()}
    }

    pub fn login(&self) -> String {
        self.data.login.clone()
    }

    pub fn pass(&self) -> String{
        self.data.pass.clone()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn change_nickname(&mut self, nickname: &str) {
        self.name = nickname.to_string();
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

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {}, login {}", self.name, self.data.login)
    }
}

#[derive(Debug)]
pub struct UserData {
    pass: String,
    login: String,
    pub email: String,
    pub friends: Vec<Rc<RefCell<User>>>
}

impl UserData {
    pub fn new(login: String, pass: String, email: String) -> UserData {
        UserData {
            pass,
            login,
            email,
            friends: vec![]
        }
    }

}