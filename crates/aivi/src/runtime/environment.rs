use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::values::Value;

#[derive(Clone)]
pub(super) struct Env {
    inner: Arc<EnvInner>,
}

struct EnvInner {
    parent: Option<Env>,
    values: Mutex<HashMap<String, Value>>,
}

impl Env {
    pub(super) fn new(parent: Option<Env>) -> Self {
        Self {
            inner: Arc::new(EnvInner {
                parent,
                values: Mutex::new(HashMap::new()),
            }),
        }
    }

    pub(super) fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.inner.values.lock().expect("env lock").get(name) {
            return Some(value.clone());
        }
        self.inner
            .parent
            .as_ref()
            .and_then(|parent| parent.get(name))
    }

    pub(super) fn set(&self, name: String, value: Value) {
        self.inner
            .values
            .lock()
            .expect("env lock")
            .insert(name, value);
    }
}

pub(super) struct RuntimeContext {
    pub(super) globals: Env,
}
