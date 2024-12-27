use crate::docker_file::network::network::Network;

pub struct Networks {
    pub name: String,
    pub networks: Vec<Network>,
}

impl Networks {
    pub fn new(&self, name: String) -> Self {
        Networks { name, networks: vec![] }
    }
}
