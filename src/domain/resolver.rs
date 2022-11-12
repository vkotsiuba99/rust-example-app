use std::sync::Arc;

use domain::products::model::store as product_store;
use domain::orders::model::store as order_store;

pub struct Resolver {
    product_store: Arc<product_store::InMemoryStore>,
    order_store: Arc<order_store::InMemoryStore>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            product_store: Arc::new(product_store::in_memory_store()),
            order_store: Arc::new(order_store::in_memory_store())
        }
    }
}

impl Resolver {
    pub fn product_store(&self) -> impl product_store::ProductStore {
        self.product_store.clone()
    }

    pub fn order_store(&self) -> impl order_store::OrderStore {
        self.order_store.clone()
    }

    pub fn order_with_items_store(&self) -> impl order_store::OrderLineItemsAggregateStore {
        self.order_store.clone()
    }
}