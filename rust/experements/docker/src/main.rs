mod docker_file;

use std::collections::HashMap;
use docker_file::*;


fn main() {
    let mut compose = docker_compose::DockerCompose::new(String::from("3.6"));
    let mut services_list: Vec<service::Service> = Vec::new();
    
    let mut service: service::Service = service::Service::new("gitlab-server".to_string());
    service.restart = Some(restart::Restart::Always);
    service.image = Some("gitlab/gitlab-ce:latest".to_string());
    service.container_name = Some("gitlab".to_string());

    let mut env_map:HashMap<String, Vec<String>> = HashMap::new();
    let mut env_list = Vec::new();
    env_list.push("external_url 'http://localhost:8088'".to_string());
    env_list.push("nginx['listen_port']=8088".to_string());
    env_list.push("gitlab_rails['initial_root_password']='george4586533'".to_string());
    env_list.push("puma['worker_processe']=0".to_string());
    env_map.insert("GITLAB_OMNIBUS_CONFIG".to_string(), env_list);
    service.environment = Some(env_map);

    let mut volumes: Vec<String> = Vec::new();
    volumes.push("'./gitalab/config:/etc/gitlab'".to_string());
    volumes.push("'./gitalab/logs:/var/log/gitlab'".to_string());
    volumes.push("'./gitalab/data:/var/opt/gitlab'".to_string());
    service.volumes = Some(volumes);

    let mut ports = Vec::new();
    ports.push("8088:8088".to_string());
    ports.push("8443:443".to_string());
    ports.push("8022:22".to_string());
    service.ports = Some(ports);
    service.hostname = Some("'localhost'".to_string());

    services_list.push(service);

    
    let mut service1: service::Service = service::Service::new("gitlab-runner".to_string());
    // service1.restart = Restart::Always;
    service1.image = Some("gitlab/gitlab-runner".to_string());
    service1.container_name = Some("gitlab-runner".to_string());

    let mut vol2: Vec<String> = Vec::new();
    vol2.push("/var/run/docker.sock:/var/run/docker.sock".to_string());
    vol2.push("./gitlab-runner/config:/etc/gitlab-runner".to_string());
    service1.volumes = Some(vol2);

    services_list.push(service1);

    compose.services = Some(services_list);
    compose.generate_file(String::from("docker-compose.yml"));
}
