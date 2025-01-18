use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Store {
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn init() -> Self {
        Store {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let inner = self.inner.lock().await;
        inner.get(key).cloned()
    }

    pub async fn set(&self, key: &str, value: &str) {
        let mut inner = self.inner.lock().await;
        inner.insert(key.to_string(), value.to_string());
    }

    pub async fn delete(&self, key: &str) -> Option<String> {
        let mut inner = self.inner.lock().await;
        inner.remove(key)
    }
}
