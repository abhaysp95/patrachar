#[derive(serde::Deserialize)]
pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        User {
            name, email
        }
    }

    pub fn get_name(&self) -> String {
        self.name
    }

    pub fn get_email(&self) -> String {
        self.email
    }

    pub fn set_name(&self, name: String) {
        self.name = name
    }

    pub fn set_email(&self, email: String) {
        self.email = email
    }
}
