pub mod io;
pub mod user;
pub mod place;

use user::User;
use place::{roles::{RolePerms}, Place};
use std::{cell::RefCell, rc::{Rc, Weak}, time::SystemTime};

#[derive(Debug)]
pub struct Simulation{
    current_user: Weak<RefCell<User>>,
    members: Vec<Rc<RefCell<User>>>,
    places: Vec<Rc<RefCell<Place>>>
}

impl Simulation{
    /*
    
    Add checking for exising id numbers in methods 'get_user_next_id' and 'get_place_next_id'
    While creating user check if login/e-mail is unique

    */


    // Constructor
    pub fn new() -> Simulation{
        let mut s = Simulation{
            members: vec![],
            places: vec![],
            current_user: Weak::new(),
        };
        // create default place
        let place = Place::new("Default".to_string(), None, 0);
        s.places.push(Rc::from(RefCell::new(place)));

        s.create_user("Admin".to_string(), "***".to_string());
        s
    }
    
    // User methods
    fn get_next_user_id(&self) -> u64 {
        self.members.len() as u64
    }

    pub fn create_user(&mut self, name: String, pass: String) -> u64 {
        let num = self.get_next_user_id();
        let user = User::new(name, pass, num, self.get_default_place());
        let user = Rc::from(RefCell::new(user));
        self.members.push(Rc::clone(&user));
        self.add_to_default_place(user);
        num
    }

    pub fn get_user_by_id(&self, id: u64) -> Result<Rc<RefCell<User>>, &'static str> {
        for x in self.members.iter(){
            if x.borrow().id() == id{
                return Ok(Rc::clone(&x))
            }
        }
        Err("Cannot find the user. Make sure you have the correct id.")
    }

    // Place methods
    fn get_default_place(&self) -> Rc<RefCell<Place>> {
        self.get_place_by_id(0).unwrap()
    }

    fn add_to_default_place(&mut self, user: Rc<RefCell<User>>) {
        let place = self.get_place_by_id(0).unwrap();
        place.borrow_mut().add_user(user);
    }

    fn get_next_place_id(&self) -> u64 {
        self.places.len() as u64
    }

    pub fn get_place_by_id(&self, id: u64) -> Result<Rc<RefCell<Place>>, &'static str> {
        for x in self.places.iter(){
            if x.borrow().id() == id{
                return Ok(Rc::clone(x))
            }
        }
        Err("Cannot find the place. Make sure you have the correct id.")
    }

    pub fn return_current_place(&self) -> Rc<RefCell<Place>> {
        let user = match self.current_user.upgrade() {
            Some(user) => user,
            None => {return Rc::clone(&self.get_default_place());}
        };
        let place = Rc::clone(&user.borrow().place);
        place
    }

    pub fn create_place(&mut self, name: String, admin_id: u64) -> u64 {
        let admin = match self.get_user_by_id(admin_id) {
            Ok(x) => Some(x),
            Err(_) => None
        };
        let num = self.get_next_place_id();
        let place = Place::new(name, admin, num);
        self.places.push(Rc::from(RefCell::new(place)));
        num
    }

    pub fn return_current_user_perms(&self) -> RolePerms {
        let place = self.return_current_place();
        let x = place.borrow().return_perms(self.return_current_user().unwrap().borrow().id());
        x
    }

    // Loggin system
    pub fn logged(&self) -> bool {
        match self.current_user.upgrade() {
            Some(_) => true,
            None => false
        }
    }

    pub fn log_in(&mut self, login: String, password: String) -> Result<&'static str, &'static str>{
        for user in self.members.iter() {
            if user.borrow().login() == login
            {
                if user.borrow().pass() == password {
                    self.current_user = Rc::downgrade(user);
                    return Ok("Logged in successfuly.")
                }
                else {
                    return Err("Logging failed. Wrong login or password.")
                }
            }
        }
        Err("Logging failed. Can't find the user")
    }

    pub fn log_off(&mut self){
        self.current_user = Weak::new();
        println!("Successfuly log off!");
    }

    pub fn return_current_user(&self) -> Option<Rc<RefCell<User>>> {
        self.current_user.upgrade()
    }

    // Functions
    pub fn change_place(&mut self, place_id: u64) -> Option<&'static str>{
        let user =  self.current_user.upgrade().unwrap();
        let place = match self.get_place_by_id(place_id) {
            Ok(x) => x,
            Err(err) => {return Some(err);}
        };
        // Add user if it's first time
        user.borrow_mut().place = Rc::clone(&place);
        if place.borrow().find_user_by_id(user.borrow().id()).is_none() {
            place.borrow_mut().add_user(Rc::clone(&user));
        }
        None
        
    }

    pub fn reset_place(&mut self){
        self.change_place(0);
    }

    pub fn send_message(&mut self, message: &str){
        let user = match self.current_user.upgrade() {
            Some(x) => x,
            None => {return;}
        };
        let place = Rc::clone(&self.current_user.upgrade().unwrap().borrow().place);
        io::sent_message(&user.borrow(), &place.borrow(), message);
        let mess = place::PlaceMessage::new(&user, String::from(message), SystemTime::now());
        place.borrow_mut().add_message(mess);
    }

    pub fn return_current_place_messages(&self) -> Vec<String> {
        let place = self.return_current_place();
        let mut vec: Vec<String> = vec![];
        for mesg in place.borrow().messages.iter() {
            vec.push(format!("{mesg}"));
        }
        vec
    }

    // Admin functions
    pub fn is_admin(&self, user_id: u64) -> bool {
        match self.return_current_place().borrow().find_admin(user_id) {
            Some(_) => true,
            None => false,
        }
        
    }

    pub fn is_admin_in_server(&self, user_id: u64, place_id: u64) -> bool {
        let place = match self.get_place_by_id(place_id) {
            Ok(place) => {
                match place.borrow().find_admin(user_id) {
                    Some(_) => true,
                    None => false,
                }
            },
            Err(_) => false,
        };
        place
    }

    pub fn ban_user(&mut self, user_id: u64) -> Result<bool, &'static str> {
        let curr_user = Rc::clone(&self.current_user.upgrade().unwrap());
        let curr_place = Rc::clone(&curr_user.borrow().place);
        if self.is_admin(curr_user.borrow().id()) {
            return Ok(curr_place.borrow_mut().ban_user(user_id));
        }
        Err("User not an admin")
    } 

}

#[cfg(test)]
mod testing{
    use super::*;
    
    #[test]
    fn check_logged_start(){
        let sim = Simulation::new();
        //println!("{:#?}", sim);
        assert_eq!(false, sim.logged());
    }

    #[test]
    fn check_logged(){
        let mut sim = Simulation::new();
        sim.create_user("test".to_string(), "1234".to_string());
        match sim.log_in("test".to_string(), "1234".to_string()) {
            Ok(x) => x,
            Err(x) => {panic!("{x}")}
        };
    }

    #[test]
    fn check_user_partialeq(){
        let place = Rc::new(RefCell::new(Place::new("test_place".to_string(), None, 1)));
        let user1 = User::new("test1".to_string(), "123".to_string(), 1, Rc::clone(&place));
        let user2 = User::new("test2".to_string(), "321".to_string(), 1, Rc::clone(&place));

        assert_eq!(user1, user2);
    }

    #[test]
    fn check_admin() {
        let mut sim = Simulation::new();
        let id = sim.create_user("test".to_string(), "1234".to_string());
        let place_id = sim.create_place("Debug".to_string(), id);
        assert!(sim.is_admin_in_server(id, place_id));
    }
}