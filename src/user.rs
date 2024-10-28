use std::{cell::RefCell, fmt::Display, rc::Rc};


#[derive(Debug)]
pub struct User{
    name: String,
    id: u64,
    pub place: u64,
    pub group: u64,
    pub data: UserData
}

impl User {
    pub fn new(login: String, email: String, pass: String, id: u64, place: u64) -> User{
        User {
            name: String::new(),
            id,
            place,
            data: UserData::new(login, pass, email, 0),
            group: 0,

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

    pub fn email(&self) -> String {
        self.data.email.clone()
    }

    pub fn change_nickname(&mut self, nickname: &str) {
        self.name = nickname.to_string();
    }

    pub fn update_def_group(&mut self, id: u64) {
        self.data.def_group_id = id;
    }

    pub fn return_def_group(&self) -> u64 {
        self.data.def_group_id
    }

    pub fn show_user_data(&self) {
        println!("User: {}\nEmail: {}\nDefault group id: {}\nMore to come!", self.name(), self.email(), self.return_def_group());
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
        write!(f, "User {}, login {}", self.name(), self.data.login)
    }
}

#[derive(Debug)]
pub struct UserData {
    pass: String,
    login: String,
    pub email: String,
    pub friends: Vec<Rc<RefCell<User>>>,
    pub places: Vec<u64>, // ID? or Weak<>?
    groups: Vec<u64>,
    def_group_id: u64
}

impl UserData {
    pub fn new(login: String, pass: String, email: String, group_id: u64) -> UserData {
        UserData {
            pass,
            login,
            email,
            friends: vec![],
            places: vec![],
            groups: vec![],
            def_group_id: group_id
        }
    }

    pub fn add_friend(&mut self, friend: Rc<RefCell<User>>) {
        let friend_clone = Rc::clone(&friend);
        self.friends.push(friend_clone);
    }

    pub fn return_friends_list_iter(&self) -> Vec<Rc<RefCell<User>>> {
        self.friends.clone()
    }

    pub fn check_group_id(&self, group_id: u64) -> bool {
        match self.groups.binary_search(&group_id) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn check_place_id(&self, place_id: u64) -> bool {
        match self.places.binary_search(&place_id) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn add_place(&mut self, place_id: u64) {
        if !self.check_place_id(place_id) {
            self.places.push(place_id);
        }
    }

    pub fn add_group(&mut self, group_id: u64) {
        if !self.check_group_id(group_id) {
            self.groups.push(group_id);
        }
    }

    pub fn return_groups(&self) -> &Vec<u64> {
        &self.groups
    }

    pub fn return_places(&self) -> &Vec<u64> {
        &self.places
    }

    pub fn login(&self) -> String {
        self.login.clone()
    }
}