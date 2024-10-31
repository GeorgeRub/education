use std::fs::File;
use std::io::Write;
use crate::service::Service;

pub struct DockerCompose {
    version: String,
    pub(crate) services: Option<Vec<Service>>,
}

impl DockerCompose {
    pub fn new(version: String) -> DockerCompose {
        DockerCompose { version, services: None }
    }

    pub fn generate_file(&self, file_name: String) {
        // println!("Generating file: {}", file_name);
        let mut f = File::create(file_name).expect("Unable to create file");
        let version_string = format!("version: '{}'\r\n", &self.version);
        f.write_all(version_string.as_ref()).expect("Error write version of a file");
        f.write_all("services:\r\n".as_ref()).expect("TODO: panic message");
        match &self.services {
            Some(services) => {
                for service in services {
                    service.generate(&mut f);
                }
            },
            _ => {}
        }
    }

}