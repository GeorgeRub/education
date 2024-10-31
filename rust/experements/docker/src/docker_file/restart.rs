#[derive(Debug)]
pub enum Restart {
    Always,
    No,
    OnFailure,
    UnlessStopped
}

impl Restart {
    pub fn get_value(&self) -> &'static str {
        match self {
            Restart::Always => "always",
            Restart::No => "no",
            Restart::OnFailure => "on-failure",
            Restart::UnlessStopped => "unless-stopped"
        }
    }
}