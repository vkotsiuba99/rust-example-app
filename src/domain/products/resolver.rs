use std::sync::Arc;

use super::model::store::*;

pub struct Resolver {
    store: Arc<InMemoryStore>
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            store: Arc::new(in_memory_store())
        }
    }
}

impl Resolver {
    pub fn product_store(&self) -> impl ProductStore {
        self.store.clone()
    }
}