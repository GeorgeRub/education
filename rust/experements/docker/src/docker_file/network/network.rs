use crate::docker_file::network::Driver;

pub struct Network {
    pub name: String,
    pub driver: Driver,
}

impl Network {
    pub fn new(name: String, driver: Driver) -> Self {
        Network { name, driver }
    }
}