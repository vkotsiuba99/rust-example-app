/*! Contains the `GetOrderQuery` type. */

use crate::domain::{
    infra::*,
    orders::*,
    Error,
};

pub type Result = ::std::result::Result<Option<Order>, Error>;

/** Input for a `GetOrderQuery`. */
#[derive(Deserialize)]
pub struct GetOrder {
    pub id: OrderId,
}

/** Get an order entity. */
#[auto_impl(Fn)]
pub trait GetOrderQuery {
    fn get_order(&self, query: GetOrder) -> Result;
}

/** Default implementation for a `GetOrderQuery`. */
pub(in crate::domain) fn get_order_query(store: impl OrderStore) -> impl GetOrderQuery {
    move |query: GetOrder| {
        let order = store.get_order(query.id)?;

        Ok(order)
    }
}

impl Resolver {
    /** Get an order. */
    pub fn get_order_query(&self) -> impl GetOrderQuery {
        let store = self.order_store();

        get_order_query(store)
    }
}