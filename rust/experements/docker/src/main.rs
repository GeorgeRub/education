mod docker_file;

use docker_file::*;


fn main() {
    let mut compose = docker_compose::DockerCompose::new(String::from("3.6"));
    let mut services_list: Vec<service::Service> = Vec::new();
    
    let mut service: service::Service = service::Service::new("gitlab-server".to_string());
    service.restart = Some(restart::Restart::Always);
    service.image = Some("gitlab/gitlab-ce:latest".to_string());
    service.container_name = Some("gitlab".to_string());
    services_list.push(service);
    
    let mut service1: service::Service = service::Service::new("gitlab-runner".to_string());
    // service1.restart = Restart::Always;
    service1.image = Some("gitlab/gitlab-runner".to_string());
    service1.container_name = Some("gitlab-runner".to_string());
    services_list.push(service1);

    compose.services = Some(services_list);
    compose.generate_file(String::from("docker-compose.yml"));
}
