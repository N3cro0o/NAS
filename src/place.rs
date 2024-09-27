use std::{cell::RefCell, fmt::Display, rc::{Rc, Weak}, time::SystemTime};
use chrono::{DateTime, Local};
use roles::RoleTrait;
use super::user::User;

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
                perms: RolePerms::new(true, priority)
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
        pub priority: u8
    }

    impl RolePerms {
        pub fn new(message: bool, priority: u8) -> RolePerms {
            RolePerms {
                can_talk: message,
                priority
            }
        }

        pub fn new_admin() -> RolePerms {
            RolePerms {
                can_talk: true,
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

#[derive(Debug)]
pub struct Place {
    pub name: String,
    id: u64,
    pub members: Vec<PlaceUser>,
    admin: Vec<Weak<RefCell<User>>>,
    pub messages: Vec<PlaceMessage>,
    roles: Vec<roles::RoleTemplate>
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

    pub fn add_user(&mut self, user: Rc<RefCell<User>>){
        let x = PlaceUser::new(Rc::downgrade(&user));
        self.members.push(x);
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

    pub fn find_user_by_id(&self, id: u64) -> Option<&PlaceUser> {
        for user in self.members.iter(){
            if user.user.upgrade().unwrap().borrow().id() == id {return Some(user);}
        }
        return None;
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
        write!(f, "{} at {}:\n{}", self.user.upgrade().unwrap().borrow().name(), DateTime::<Local>::from(self.time), self.message)
    }
}

#[derive(Debug)]
pub struct PlaceUser {
    pub user: Weak<RefCell<User>>,
    pub roles: Vec<roles::RoleTemplate>,
    pub banned: bool
}  

impl PlaceUser {
    pub fn new(user: Weak<RefCell<User>>) -> PlaceUser{
        let role = roles::RoleTemplate::new_basic();
        PlaceUser {
            user,
            roles: vec![role],
            banned: false
        }
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