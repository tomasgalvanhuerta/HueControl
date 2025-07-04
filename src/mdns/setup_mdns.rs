use mdns_sd::{Receiver, ServiceDaemon, ServiceEvent};
pub struct SetupMDNS {
    service_daemon: ServiceDaemon,
}

impl SetupMDNS {
    pub fn new() -> Self {
        let service_daemon = ServiceDaemon::new()
            .map(|service_daemon| SetupMDNS { service_daemon })
            .unwrap(); // bang!
        return service_daemon;
    }

    pub fn start(&self) -> Receiver<ServiceEvent> {
        self.disccover_mdns().expect("Welp") // BANG
    }

    pub fn disccover_mdns(&self) -> Result<Receiver<ServiceEvent>, SetupMDNSError> {
        let service_type = "_hue._tcp._local";
        let browse_result = self
            .service_daemon
            .browse(service_type)
            .map_err(|_| SetupMDNSError::NoDaemon);
        return browse_result;
    }
}

pub enum Event {
    /// Search has started for type
    SearchStarted(String),

    /// Found a Service (serive_type, fullname)
    Found(String, String),
}

impl Event {
    pub fn new_from(service_event: ServiceEvent) -> Self {
        match service_event {
            ServiceEvent::SearchStarted(str) => {
                println!("Searching started{str}");
            }
            ServiceEvent::ServiceFound(service_type, fullname) => {
                println!("{service_type} {fullname}, Service Found")
            }
            ServiceEvent::ServiceResolved(info) => println!("{:?}, Searching started", info),
            ServiceEvent::ServiceRemoved(service_type, fullname) => {
                println!("{service_type} {fullname}, Sclearearching started")
            }
            ServiceEvent::SearchStopped(reason) => println!("{reason}, Search Stopped"),
        }
        return Self::Found(String::new(), String::new());
    }
}

#[derive(Debug)]
pub enum SetupMDNSError {
    NoDaemon,
    DiscoverMDNS,
    ServiceError,
}
