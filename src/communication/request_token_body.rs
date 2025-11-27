pub enum RequestTokenBody {
    DeviceType(String),
    GenerateClientKey(bool),
}

impl RequestTokenBody {
    pub fn as_string(&self) -> String {
        match self {
            RequestTokenBody::DeviceType(name) => format!("devicetype:{name}"),
            RequestTokenBody::GenerateClientKey(flag) => format!("generateclientkey:{flag}"),
        }
    }
}
