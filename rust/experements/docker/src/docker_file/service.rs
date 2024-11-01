use crate::docker_file::restart::Restart;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

#[derive(Debug)]
pub struct Service {
    pub service_name: String,
    pub image: Option<String>,
    pub container_name: Option<String>,
    pub restart: Option<Restart>,
    pub hostname: Option<String>,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub environment: Option<HashMap<String, Vec<String>>>,
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
            environment: Option::None,
        }
    }

    fn generate_restart(&self, file: &mut File) {
        match &self.restart {
            Some(restart) => {
                file.write_all(format!("\t\trestart: {}\r\n", restart.get_value()).as_ref()).expect("");
            }
            _ => {}
        }
    }

    fn generate_hostname(&self, file: &mut File) {
        match &self.hostname {
            Some(hostname) => {
                file.write_all(format!("\t\thostname: {}\r\n", hostname).as_ref()).expect("");
            }
            _ => {}
        }
    }

    fn generate_ports(&self, file: &mut File) {
        match &self.ports {
            Some(ports) => {
                file.write_all("\t\tports:\r\n".to_string().as_ref()).expect("");
                for port in ports.iter() {
                    file.write_all(format!("\t\t\t- '{}'\r\n", port).as_ref()).expect("");
                }
            }
            _ => {}
        }
    }

    fn generate_environments(&self, file: &mut File) {
        match &self.environment {
            Some(environment) => {
                file.write_all("\t\tenvironment:\r\n".to_string().as_ref()).expect("Error writing the environment key!!!");
                for (key, value) in environment.iter() {
                    let mut env_val: String = String::new();
                    for val in value.iter() {
                        env_val.push_str(format!(" {}", val).as_str());
                    }
                    file.write_all(format!("\t\t\t{}:{}\r\n", key, env_val).as_ref()).expect("Error writing the environment value!!!");
                }
            }
            _ => {}
        }
    }

    fn generate_volumes(&self, file: &mut File) {
        match &self.volumes {
            Some(volumes) => {
                file.write_all("\t\tvolumes:\r\n".to_string().as_ref()).expect("Error writing the volumes key!!!");
                for volume in volumes.iter() {
                    file.write_all(format!("\t\t\t- {}\r\n", volume).as_ref()).expect("Error writing the volume!!!");
                }
            }
            _ => {}
        }
    }

    pub fn generate(&self, file: &mut File) {
        file.write_all(format!("\t{}:\r\n", &self.service_name).as_ref()).expect("Error writing the service name!!!");
        file.write_all(format!("\t\timage: {}\r\n", &self.image.as_ref().unwrap()).as_ref()).expect("Error writing the image!!!");
        file.write_all(format!("\t\tcontainer_name: {}\r\n", &self.container_name.as_ref().unwrap()).as_ref()).expect("Error writing the container name!!!");
        self.generate_restart(file);
        self.generate_hostname(file);
        self.generate_ports(file);
        self.generate_volumes(file);
        self.generate_environments(file);
    }
}