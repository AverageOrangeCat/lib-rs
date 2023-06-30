use std::sync::Arc;
use tokio::sync::RwLock;

pub type ShareType<T> = Arc<RwLock<T>>;

pub struct Share {  }

impl Share {
    pub fn new<T>(value: T) -> ShareType<T> {
        Arc::new(RwLock::new(value))
    }
}
