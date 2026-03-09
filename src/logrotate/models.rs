#[derive(Debug, Clone)]
pub struct CreateSpec {
    pub mode: u32,
    pub owner: String,
    pub group: String,
}

#[derive(Debug, Clone)]
pub struct LogrotateConfig {
    pub frequency: Option<String>,
    pub rotate: Option<u32>,
    pub compress: bool,
    pub delaycompress: bool,
    pub missingok: bool,
    pub notifempty: bool,
    pub create: Option<CreateSpec>,
    pub sharedscripts: bool,
    pub postrotate: Option<String>,
}
