use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::ProductStore;

#[derive(Deserialize)]
pub struct SetProductTitle {
    pub id: i32,
    pub title: String,
}

pub type SetProductTitleError = String;

#[auto_impl(FnMut)]
pub trait SetProductTitleCommand {
    fn set_product_title(&mut self, command: SetProductTitle) -> Result<(), SetProductTitleError>;
}

pub fn set_product_title_command<TStore>(store: TStore) -> impl SetProductTitleCommand 
    where TStore: ProductStore
{
    move |command: SetProductTitle| {
        let product = {
            if let Some(mut product) = store.get(command.id)? {
                product.set_title(command.title)?;

                product
            }
            else {
                Err("not found")?
            }
        };

        store.set(product)
    }
}

impl Resolver {
    pub fn set_product_title_command(&self) -> impl SetProductTitleCommand {
        let store = self.product_store();

        set_product_title_command(store)
    }
}