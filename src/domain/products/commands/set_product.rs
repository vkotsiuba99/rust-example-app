use auto_impl::auto_impl;

use domain::products::Product;
use domain::products::infra::{Resolver, Store};

pub struct SetProduct {
    pub id: i32,
    pub title: String,
}

pub type CommandError = String;

#[auto_impl(FnMut)]
pub trait SetProductCommand {
    fn set_product(&mut self, command: SetProduct) -> Result<(), CommandError>;
}

pub fn set_product_command<TStore>(store: TStore) -> impl SetProductCommand 
    where TStore: Store
{
    move |command: SetProduct| {
        let product = {
            if let Some(mut product) = store.get(command.id)? {
                product.set_title(command.title)?;

                product
            }
            else {
                Product::new(command.id, command.title)?
            }
        };

        store.set(product)
    }
}

impl Resolver {
    pub fn set_product_command(&self) -> impl SetProductCommand {
        let store = self.store();

        set_product_command(store)
    }
}