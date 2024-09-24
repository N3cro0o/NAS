pub mod io;
pub mod user;
pub mod place;

use user::User;
use place::Place;
use std::{cell::RefCell, rc::{Rc, Weak}, time::SystemTime};

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
        Simulation{
            members: vec![],
            places: vec![],
            current_user: Weak::new(),
        }
    }
    
    // User methods
    fn get_next_user_id(&self) -> u64 {
        self.members.len() as u64
    }

    pub fn create_user(&mut self, name: String, pass: String) -> u64 {
        let num = self.get_next_user_id();
        let user = User::new(name, pass, num);
        self.members.push(Rc::from(RefCell::new(user)));
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

    pub fn create_place(&mut self, name: String, admin: Rc<RefCell<User>>) -> u64 {
        let num = self.get_next_place_id();
        let place = Place::new(name, admin, num);
        self.places.push(Rc::from(RefCell::new(place)));
        num
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
            if user.borrow().name == login
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
    pub fn change_place(&mut self, place: &Rc<RefCell<Place>>){
        let user =  self.current_user.upgrade().unwrap();
        user.borrow_mut().place = Rc::downgrade(place);
        if !place.borrow().members.contains(&user) {
            place.borrow_mut().add_user(Rc::clone(&user));
        }
        
    }

    pub fn send_message(&mut self, message: &str){
        let user = match self.current_user.upgrade() {
            Some(x) => x,
            None => {return;}
        };
        let place = match self.current_user.upgrade().unwrap().borrow().place.upgrade() {
            Some(x) => x,
            None => {return;}
        };
        io::sent_message(&user.borrow(), &place.borrow(), message);
        let mess = place::PlaceMessage::new(&user, String::from(message), SystemTime::now());
        place.borrow_mut().add_message(mess);
    }

}

#[cfg(test)]
mod testing{
    use super::*;
    #[test]
    fn check_logged_start(){
        let sim = Simulation::new();
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
        let user1 = User::new("test1".to_string(), "123".to_string(), 1);
        let user2 = User::new("test2".to_string(), "321".to_string(), 1);

        assert_eq!(user1, user2);
    }
}