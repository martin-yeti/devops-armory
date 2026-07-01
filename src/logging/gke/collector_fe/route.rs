use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Route {
    Dashboard,
    NotFound(String),
}


impl Route {
    pub fn new(path: &str) -> Route {
        vertigo::log::info!("path = {path}");
        match path {
            "" | "/" => Self::Dashboard,
            path => Self::NotFound(path.to_string()),
        }
    }
}

impl From<String> for Route {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dashboard => write!(f, "/"),
            Self::NotFound(path) => write!(f, "{path}"),
        }
    }
}
