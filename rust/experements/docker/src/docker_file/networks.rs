pub enum Driver {
    Bridge
}

impl Driver {
    pub fn get_value(&self) -> &'static str {
        match self {
            Driver::Bridge => "bridge",
        }
    }
}

pub struct Networks {
    pub name: String,
    pub networks: Vec<Network>,
}

impl Networks {
    pub fn new(&self, name: String) -> Self {
        Networks { name, networks: vec![] }
    }
}

pub struct Network {
    pub name: String,
    pub driver: Driver,
}

impl Network {
    pub fn new(name: String, driver: Driver) -> Self {
        Network { name, driver }
    }
}