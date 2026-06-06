use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Upstreams {
    pub addrs: Arc<Vec<String>>,
    pub idx: Arc<AtomicUsize>,
}

impl Upstreams {
    pub fn new(addrs: Vec<String>) -> Self {
        Self {
            addrs: Arc::new(addrs),
            idx: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next(&self) -> String {
        let i = self.idx.fetch_add(1, Ordering::Relaxed);
        let n = self.addrs.len();
        self.addrs[i % n].clone()
    }

}

pub struct ForbiddenPath(pub actix_web::web::Data<std::string::String>);
pub struct SudoExecutor(pub String);
pub struct ScriptLocation(pub String);
