#[derive(Debug)]
pub struct User {
    name: String,
    age: u8,
    role: UserRole,
    gender: Gender,
}

#[derive(Debug)]
pub enum UserRole {
    Admin,
    Owner,
    Customer,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        match self {
            UserRole::Admin => return "Admin".to_owned(),
            UserRole::Customer => return "Customer".to_owned(),
            UserRole::Owner => return "Owner".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other(String),
}

impl Gender {
    pub fn to_string(&self) -> String {
        match self {
            Gender::Male => return "Male".to_owned(),
            Gender::Female => return "Female".to_owned(),
            Gender::Other(x) => return format!("Other ({})", x),
        }
    }
}

impl User {
    pub fn new(n: String, a: u8, g: Gender, r: Option<UserRole>) -> User {
        User {
            name: n,
            age: a,
            role: r.unwrap_or(UserRole::Customer),
            gender: g,
        }
    }

    pub fn hello(&self) {
        println!(
            "Hello {} {} {} {}",
            self.name,
            self.age,
            self.role.to_string(),
            self.gender.to_string()
        );
    }

    pub fn kill_me(&self) {
        panic!("AH! no");
    }
}
