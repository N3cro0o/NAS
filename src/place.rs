use std::{cell::RefCell, fmt::Display, rc::{Rc, Weak}, time::SystemTime};
use chrono::{DateTime, Local};
use roles::{RoleTemplate, RoleTrait};
use super::user::User;

/*
    Change how admin fields work
        Admin search in members field or just change data type

*/

pub mod roles {
    use std::u8;

    pub trait RoleTrait {
        fn return_perms(&self) -> RolePerms;
        fn update_perms(&mut self, source: RolePerms) -> RolePerms;
    }

    #[derive(Debug, Clone)]
    pub struct RoleTemplate {
        pub name: String,
        perms: RolePerms
    }

    impl RoleTemplate {
        pub fn new(name: String, priority: u8) -> RoleTemplate {
            RoleTemplate {
                name,
                perms: RolePerms::new(true, false, priority)
            }
        }

        pub fn new_basic() -> RoleTemplate {
            Self::new("Basic".to_string(), 0)
        }

        pub fn new_admin() -> RoleTemplate {
            RoleTemplate {
                name: String::from("Admin"),
                perms: RolePerms::new_admin()
            }
        }
    }

    impl RoleTrait for RoleTemplate {
        fn return_perms(&self) -> RolePerms {
            self.perms.clone()
        } 

        fn update_perms(&mut self, source: RolePerms) -> RolePerms {
            self.perms.clone_from(&source);
            self.perms.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct RolePerms {
        pub can_talk: bool,
        pub priority: u8,
        pub change_nickname: bool
    }

    impl RolePerms {
        pub fn new(message: bool, nickname: bool, priority: u8) -> RolePerms {
            RolePerms {
                can_talk: message,
                change_nickname: nickname,
                priority
            }
        }

        pub fn new_admin() -> RolePerms {
            RolePerms {
                can_talk: true,
                change_nickname:  true,
                priority: u8::MAX
            }
        }
    }

    #[cfg(test)]
    mod testing {
        use super::*;

        #[test]
        fn check_clone(){
            let mut role1 = RoleTemplate::new("test".to_string(), 0);
            role1.perms.can_talk = false;
            let perms = role1.return_perms();
            assert_eq!(role1.perms.can_talk, perms.can_talk);
        }
    }
}

pub trait PlaceTrait {
    fn add_user(&mut self, user: Rc<RefCell<User>>);
    fn find_user_by_id(&self, id: u64) -> Option<&PlaceUser>;
}

#[derive(Debug)]
pub struct DefaultPlace {
    pub members: Vec<PlaceUser>,
    admin: Vec<Rc<RefCell<User>>>,
    messages: Vec<DefaultPlaceMessage>,
    groups: Vec<DefaultPlaceGroup>,
}

#[derive(Debug)]
pub struct PlaceMessage {
    user: Weak<RefCell<User>>,
    message: String,
    time: SystemTime
}

#[derive(Debug)]
pub struct DefaultPlaceMessage {
    sender: Rc<RefCell<User>>,
    message: String,
    time: SystemTime
}
#[derive(Debug)]
pub struct DefaultPlaceGroup {
    messages: Vec<DefaultPlaceMessage>,
    members: Vec<Rc<RefCell<User>>>,
    name: String
}

#[derive(Debug)]
pub struct Place {
    pub name: String,
    id: u64,
    pub members: Vec<PlaceUser>,
    admin: Vec<Weak<RefCell<User>>>,
    pub messages: Vec<PlaceMessage>,
    roles: Vec<roles::RoleTemplate>
}

impl PlaceTrait for Place {
    fn add_user(&mut self, user: Rc<RefCell<User>>){
        let x = PlaceUser::new(Rc::downgrade(&user));
        self.members.push(x);
    }
    fn find_user_by_id(&self, id: u64) -> Option<&PlaceUser> {
        for user in self.members.iter(){
            if user.user.upgrade().unwrap().borrow().id() == id {return Some(user);}
        }
        return None;   
    }
}

impl Place {
    pub fn new(name: String, admin: Option<Rc<RefCell<User>>>, id: u64) -> Place { 
        // Default roles
        // Basic
        // Admin

        let mut p = Place{
            name,
            members: vec![],
            admin: vec![],
            id,
            messages: vec![],
            roles: vec![
                roles::RoleTemplate::new_basic(),
                roles::RoleTemplate::new_admin()
            ]
        };
        if let Some(user) = admin {
            p.admin.push(Rc::downgrade(&user));
            p.add_admin(user);
        };
        p
    }

    pub fn add_admin(&mut self, user: Rc<RefCell<User>>) {
        let mut x = PlaceUser::new(Rc::downgrade(&user));
        x.add_admin_role();
        self.members.push(x);
    }

    pub fn add_message(&mut self, message: PlaceMessage) {
        self.messages.push(message);
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }

    fn find_user_by_id_mut(&mut self, id: u64) -> Option<&mut PlaceUser> {
        for user in self.members.iter_mut(){
            if user.user.upgrade().unwrap().borrow().id() == id {return Some(user);}
        }
        return None;
    }

    pub fn find_admin(&self, id: u64) -> Option<Rc<RefCell<User>>> {
        for user in self.admin.iter() {
            if user.upgrade().unwrap().borrow().id() == id {return Some(user.upgrade().unwrap());}
        }
        return None
    }

    // Roles and perms and shit
    pub fn return_perms(&self, user_id: u64) -> roles::RolePerms {
        let user = self.find_user_by_id(user_id).unwrap();
        let mut target: &roles::RoleTemplate = &roles::RoleTemplate::new("0".to_string(), 0);
        // Check len
        if user.return_roles().len() == 0 {panic!("There should be default role");}
        for t in user.return_roles().iter() {
            if target.return_perms().priority < t.return_perms().priority {
                target = t;
            }
        }
        target.return_perms()
    }

    pub fn return_user_roles(&self, user_id: u64) -> Result<&Vec<RoleTemplate>, &'static str> {
        let user = match self.find_user_by_id(user_id) {
            Some(x) => x,
            None => {return Err("Error, can't find the user")}
        };
        Ok(user.return_roles())
    }

    pub fn update_roles(&mut self, user_id: u64, new_role: roles::RoleTemplate) -> Result<roles::RolePerms, &'static str> {
        let user = match self.find_user_by_id_mut(user_id) {
            Some(x) => x,
            None => {return Err("Error, can't find the user")}
        };
        for p in user.return_roles_mut().iter_mut() {
            if p.return_perms().priority == new_role.return_perms().priority {
                p.update_perms(new_role.return_perms());
                return Ok(new_role.return_perms().clone());
            }
        }
        user.add_role(new_role);
        Ok(self.return_perms(user_id)) // We know user is here so unwrap bois
    }

    pub fn return_role_vec(&self) -> &Vec<roles::RoleTemplate> {
        &self.roles
    }

    pub fn add_role(&mut self, new_role: roles::RoleTemplate) {
        self.roles.push(new_role);
    }

    pub fn find_role_by_id(&self, role_id: usize) -> Option<&roles::RoleTemplate> {
        self.roles.get(role_id)
    }

    pub fn find_role_by_name(&self, role_name: String) -> Option<&roles::RoleTemplate> {
        let new_name = role_name.to_lowercase();
        for r in self.roles.iter() {
            let r_name = r.name.to_lowercase();
            if new_name == r_name {
                return Some(r)
            }
        }
        None
    }

    pub fn grant_role(&mut self, user_id: u64, new_role: roles::RoleTemplate) {
        let user = match self.find_user_by_id_mut(user_id) {
            Some(user) => user,
            None => {return;}
        };
        user.add_role(new_role);
    }

    pub fn change_user_nickname(&mut self, user_id: u64, new_nick: String) -> Result<(), &'static str> {
        let user = self.find_user_by_id_mut(user_id);
        if let None = user {
            return Err("Cannot find user")
        }
        user.unwrap().place_nickname = new_nick;
        Ok(())
    }

    // Banicja methods
    pub fn ban_user(&mut self, user_id: u64) -> bool {
        let mut target_user: Option<&mut PlaceUser> = None;
        // find place user data
        for user in self.members.iter_mut(){
            if user.user.upgrade().unwrap().borrow().id() == user_id {target_user = Some(user)}
        }
        // if empty return
        if target_user.is_none() {return false;}
        
        // ban moron
        let target_user = target_user.unwrap();
        target_user.banned = true;
        return target_user.banned;
    }

    pub fn is_banned(&self, user: u64) -> bool {
        match self.find_user_by_id(user){
            Some(user) => user.banned,
            None => false,
        }
    }
}

impl DefaultPlace {
    pub fn new() -> Self {
        DefaultPlace{
            members: vec![],
            admin: vec![],
            messages: vec![],
            groups: vec![]
        }
    }

    pub fn add_admin(&mut self, admin: Rc<RefCell<User>>) {
        let mut admin_user = PlaceUser::new(Rc::downgrade(&admin));
        admin_user.add_admin_role();
        self.admin.push(Rc::clone(&admin));
        self.members.push(admin_user);
    }

    pub fn is_admin(&self, user_id: u64) -> bool {
        for ad in self.admin.iter() {
            if ad.borrow().id() == user_id {return true}
        }
        false
    }
}

impl PlaceTrait for DefaultPlace {
    fn add_user(&mut self, user: Rc<RefCell<User>>) {
        let x = PlaceUser::new(Rc::downgrade(&user));
        self.members.push(x);
    }

    fn find_user_by_id(&self, id: u64) -> Option<&PlaceUser> {
        for user in self.members.iter() {
            if user.user().upgrade().unwrap().borrow().id() == id {
                return Some(user)
            }
        }
        None
    }
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
        write!(f, "{} at {}:\n{}", self.user.upgrade().unwrap().borrow().name(), DateTime::<Local>::from(self.time), self.message)
    }
}

#[derive(Debug)]
pub struct PlaceUser {
    user: Weak<RefCell<User>>,
    pub place_nickname: String,
    pub roles: Vec<roles::RoleTemplate>,
    pub banned: bool
}  

impl PlaceUser {
    pub fn new(user: Weak<RefCell<User>>) -> PlaceUser{
        let role = roles::RoleTemplate::new_basic();
        PlaceUser {
            user,
            place_nickname: String::new(),
            roles: vec![role],
            banned: false
        }
    }

    pub fn user(&self) -> Weak<RefCell<User>>{
        self.user.clone()
    }

    pub fn add_admin_role(&mut self) {
        let admin = roles::RoleTemplate::new_admin();
        self.roles.push(admin);
    }

    pub fn return_roles(&self) -> &Vec<roles::RoleTemplate> {
        &self.roles
    }

    fn return_roles_mut(&mut self) -> &mut Vec<roles::RoleTemplate>{
        &mut self.roles
    }

    pub fn add_role(&mut self, new_role: roles::RoleTemplate){
        self.roles.push(new_role);
    }
}