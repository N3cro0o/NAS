pub mod io;
pub mod user;
pub mod place;

use user::User;
use place::{roles::RolePerms, Place, PlaceTrait};
use std::{cell::RefCell, rc::{Rc, Weak}, time::SystemTime};
use eframe::App;
use eframe::egui as gui;

pub struct Simulation{
    pub options: eframe::NativeOptions,
    current_user: Weak<RefCell<User>>,
    members: Vec<Rc<RefCell<User>>>,
    pub default_place: place::DefaultPlace,
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
            default_place: place::DefaultPlace::new(),
            current_user: Weak::new(),
            options: eframe::NativeOptions {
                viewport: gui::ViewportBuilder::default()
                    .with_inner_size([400.0, 500.0])
                    .with_resizable(false),
                    ..Default::default()
            }
        };
        // Create app admin
        let id = s.create_user("Admin".to_string(), "***".to_string());
        s.default_place.add_admin(s.get_user_by_id(id).unwrap());
        s
    }

    // Id methods
    fn get_next_user_id(&self) -> u64 {
        self.members.len() as u64
    }

    fn get_next_place_id(&self) -> u64 {
        // +1 because default place
        self.places.len() as u64 + 1
    }

    // App methods
    pub fn return_options(&self) -> eframe::NativeOptions {
        let x1 = &self.options;
        let x = eframe::NativeOptions{
            ..x1.clone()
        };
        x
    }

    // User methods
    pub fn create_user(&mut self, name: String, pass: String) -> u64 {
        let num = self.get_next_user_id();
        let user = User::new(name, pass, num, 0);
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
    
    pub fn return_current_user(&self) -> Option<Rc<RefCell<User>>> {
        self.current_user.upgrade()
    }

    // Place methods
    fn get_default_place(&self) -> &place::DefaultPlace {
        &self.default_place
    }

    fn get_default_place_mut(&mut self) -> &mut place::DefaultPlace {
        &mut self.default_place
    }

    fn add_to_default_place(&mut self, user: Rc<RefCell<User>>) {
        let place = self.get_default_place_mut();
        place.add_user(user);
    }

    pub fn get_place_by_id(&self, id: u64) -> Result<Rc<RefCell<Place>>, &place::DefaultPlace> {
        for x in self.places.iter(){
            if x.borrow().id() == id{
                return Ok(Rc::clone(x))
            }
        }
        Err(&self.default_place)
    }

    pub fn return_current_place(&self) -> Result<Rc<RefCell<Place>>, &place::DefaultPlace> {
        let user = self.current_user.upgrade().unwrap();
        let id = user.borrow().place;
        if id != 0 {
            if let Ok(x) =  self.get_place_by_id(id) {
                return Ok(x);
            }
        }
        Err(&self.default_place)
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
        match self.return_current_place() {
            Ok(place) => {
                return place.borrow().return_perms(self.return_current_user().unwrap().borrow().id());
            }
            Err(def) =>{
                if self.default_place.is_admin(self.current_user.upgrade().unwrap().borrow().id()) {
                    return RolePerms::new_admin()
                }
                else {
                    return RolePerms::new(true, false, 0);
                }
            }
        };
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

    // Functions
    pub fn return_curr_user_name(&self) -> String {
        let user_id = self.current_user.upgrade().unwrap().borrow().id();
        let place_name = match self.return_current_place() {
            Ok(place) => place.borrow().find_user_by_id(user_id).unwrap().place_nickname.clone(),
            Err(def) => def.find_user_by_id(user_id).unwrap().place_nickname.clone()
        };
        if place_name.is_empty() {
            self.current_user.upgrade().unwrap().borrow().name()
        }
        else{
            place_name
        }
    }

    pub fn change_place(&mut self, place_id: u64) -> Result<(), &'static str>{
        let user =  self.current_user.upgrade().unwrap();
        if place_id == 0{
            user.borrow_mut().place = 0;
            return Ok(())
        }
        let place = match self.get_place_by_id(place_id) {
            Ok(x) => x,
            Err(_) => {
                user.borrow_mut().place = 0;
                return Err("Cannot find place. Setting default")
            }
        };
        user.borrow_mut().place = place.borrow().id();
        Ok(())
    }

    pub fn reset_place(&mut self){
        self.change_place(0).unwrap();
    }

    pub fn send_message(&mut self, message: &str){
        let user = match self.current_user.upgrade() {
            Some(x) => x,
            None => {return;}
        };
        let id = self.current_user.upgrade().unwrap().borrow().place;
        let place = Rc::clone(&self.get_place_by_id(id).unwrap());
        io::sent_message(&user.borrow(), &place.borrow(), message);
        let mess = place::PlaceMessage::new(&user, String::from(message), SystemTime::now());
        place.borrow_mut().add_message(mess);
    }

    pub fn return_current_place_user_roles(&self, user_id: u64) -> Result<Vec<place::roles::RoleTemplate>, &'static str> {
        let place = self.return_current_place().unwrap();
        let roles = match place.borrow().return_user_roles(user_id) {
            Ok(x) => x.clone(),
            Err(err) => {return Err(err);}
        };
        Ok(roles)
    }

    pub fn change_nick(&mut self, user_id: u64, new_nick: &str) -> Result<(), &'static str> {
        let user = match self.get_user_by_id(user_id) {
            Ok(x) => x,
            Err(_) => {return Err("Cannot find user");}
        };
        user.borrow_mut().change_nickname(new_nick);
        Ok(())
    }

    pub fn change_nick_place(&mut self, user_id: u64, place_id: u64, new_nick: &str) -> Result<(), &'static str> {
        let place = match self.get_place_by_id(place_id){
            Ok(x) => x,
            Err(_) => {return Err("Cannot find place")}
        } ;
        place.borrow_mut().change_user_nickname(user_id, new_nick.to_string())?;
        Ok(())
    }

    pub fn return_current_place_messages(&self) -> Vec<String> {
        let place = self.return_current_place().unwrap();
        let mut vec: Vec<String> = vec![];
        for mesg in place.borrow().messages.iter() {
            vec.push(format!("{mesg}"));
        }
        vec
    }

    // Admin functions
    pub fn is_admin(&self, user_id: u64) -> bool {
        match self.return_current_place() {
            Ok(x) => {
                match x.borrow().find_admin(user_id) {
                    Some(_) => true,
                    None => false,
                }
            }
            Err(y) => {
                y.is_admin(user_id)
            } 
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
        let place_id = curr_user.borrow().place;
        let curr_place = Rc::clone(&self.get_place_by_id(place_id).unwrap());
        if self.is_admin(curr_user.borrow().id()) {
            return Ok(curr_place.borrow_mut().ban_user(user_id));
        }
        Err("User not an admin")
    } 

}

impl App for Simulation {
    // Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &gui::Context, _frame: &mut eframe::Frame) {
        gui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("NAS ver. 0.1.0");
        });
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
        let user1 = User::new("test1".to_string(), "123".to_string(), 1, place.borrow().id());
        let user2 = User::new("test2".to_string(), "321".to_string(), 1, place.borrow().id());

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