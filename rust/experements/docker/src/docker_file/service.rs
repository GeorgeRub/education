use std::fs::File;
use std::io::Write;
use crate::docker_file::restart::Restart;

#[derive(Debug)]
pub struct Service {
    pub service_name: String,
    pub image: Option<String>,
    pub container_name: Option<String>,
    pub restart: Option<Restart>,
    pub hostname: Option<String>,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
}

impl Service {
    pub fn new(service_name: String) -> Service {
        Service {
            service_name,
            image: Option::None,
            container_name: Option::None,
            restart: Option::None,
            hostname: Option::None,
            ports: Option::None,
            volumes: Option::None,
        }
    }

    pub fn generate(&self, file: &mut File) {
        println!("Generating service {}", self.service_name);

        file.write_all(format!("\t{}:\r\n", &self.service_name).as_ref()).expect("TODO: panic message");

        file.write_all(format!("\t\timage: {}\r\n", &self.image.as_ref().unwrap()).as_ref()).expect("");
        file.write_all(format!("\t\tcontainer_name: {}\r\n", &self.container_name.as_ref().unwrap()).as_ref()).expect("");
        match &self.restart {
            Some(restart) => {
                file.write_all(format!("\t\trestart: {}\r\n", &self.restart.as_ref().unwrap().get_value()).as_ref()).expect("");
            }
            _ => {}
        }
    }
}