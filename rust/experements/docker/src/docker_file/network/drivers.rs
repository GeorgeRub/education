pub enum Driver {
    Bridge, Host,Overlay,IpvLan,MacvLan, None
}

impl Driver {
    pub fn get_value(&self) -> &'static str {
        match self {
            Driver::Bridge => "bridge",
            Driver::Host => "host",
            Driver::Overlay => "overlay",
            Driver::IpvLan => "ipvlan",
            Driver::MacvLan => "macvlan",
            Driver::None => "none"
        }
    }
}