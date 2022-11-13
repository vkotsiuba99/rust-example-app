/*!
Entities for orders and line items.

The separation between `Order` and `OrderLineItem` is kind of arbitrary, and may end up being a bit of a nuisance.
If this becomes the case then rather than coupling the two together even more, we should make sure they're separated.
*/

use std::convert::{TryFrom, TryInto};

pub mod store;

#[cfg(test)]
pub mod test_data;

use domain::entity::Entity;
use domain::id::{Id, IdProvider, NextId};
use domain::version::Version;
use domain::products::{Product, ProductData, ProductId};
use domain::customers::{Customer, CustomerData, CustomerId};

pub type OrderError = String;

pub type OrderId = Id<OrderData>;
pub type NextOrderId = NextId<OrderData>;
pub type OrderVersion = Version<OrderData>;
pub type LineItemId = Id<LineItemData>;
pub type NextLineItemId = NextId<LineItemData>;
pub type LineItemVersion = Version<LineItemData>;

/// An order item quantity.
pub struct Quantity(u32);

impl TryFrom<u32> for Quantity {
    type Error = OrderError;

    fn try_from(quantity: u32) -> Result<Self, Self::Error> {
        if quantity < 1 {
            Err("quantity must be greater than 0")?
        }

        Ok(Quantity(quantity))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: OrderId,
    pub version: OrderVersion,
    pub customer_id: CustomerId,
    _private: (),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemData {
    pub id: LineItemId,
    pub version: LineItemVersion,
    pub product_id: ProductId,
    pub price: f32,
    pub quantity: u32,
    _private: (),
}

/// An order and its line items.
///
/// Products can be added to an order as a line item, so long as it isn't already there.
pub struct Order {
    order: OrderData,
    line_items: Vec<LineItemData>,
}

/// An order and one of its line items.
///
/// Properties on the line item can be updated.
pub struct OrderLineItem {
    order: OrderData,
    line_item: LineItemData,
}

pub enum IntoLineItem {
    InOrder(OrderLineItem),
    NotInOrder(Order),
}

impl OrderLineItem {
    pub(self) fn from_data(order: OrderData, line_item: LineItemData) -> Self {
        OrderLineItem {
            order: order,
            line_item: line_item,
        }
    }

    pub fn into_data(self) -> (OrderId, LineItemData) {
        (self.order.id, self.line_item)
    }

    pub fn to_data(&self) -> (OrderId, &LineItemData) {
        (self.order.id, &self.line_item)
    }

    pub fn set_quantity<TQuantity>(&mut self, quantity: TQuantity) -> Result<(), OrderError>
    where
        TQuantity: TryInto<Quantity, Error = OrderError>,
    {
        self.line_item.quantity = quantity.try_into()?.0;

        Ok(())
    }
}

impl Order {
    pub(self) fn from_data<TItems>(order: OrderData, line_items: TItems) -> Self
    where
        TItems: IntoIterator<Item = LineItemData>,
    {
        let line_items = line_items.into_iter().collect();

        Order {
            order: order,
            line_items: line_items,
        }
    }

    pub fn into_data(self) -> (OrderData, Vec<LineItemData>) {
        (self.order, self.line_items)
    }

    pub fn to_data(&self) -> (&OrderData, &[LineItemData]) {
        (&self.order, &self.line_items)
    }

    pub fn into_line_item_for_product(self, product_id: ProductId) -> IntoLineItem {
        if !self.contains_product(product_id) {
            IntoLineItem::NotInOrder(self)
        } else {
            let Order {
                order, line_items, ..
            } = self;

            let item = line_items
                .into_iter()
                .find(|item| item.product_id == product_id)
                .unwrap();

            IntoLineItem::InOrder(OrderLineItem::from_data(order, item))
        }
    }

    pub fn new<TId>(id_provider: TId, customer: &Customer) -> Result<Self, OrderError>
    where
        TId: IdProvider<OrderData>,
    {
        let id = id_provider.id()?;
        let &CustomerData {
            id: customer_id, ..
        } = customer.to_data();

        let order_data = OrderData {
            id: id,
            version: OrderVersion::default(),
            customer_id: customer_id,
            _private: (),
        };

        Ok(Order::from_data(order_data, vec![]))
    }

    pub fn contains_product(&self, product_id: ProductId) -> bool {
        self.line_items
            .iter()
            .any(|item| item.product_id == product_id)
    }

    pub fn add_product<TId, TQuantity>(&mut self, id_provider: TId, product: &Product, quantity: TQuantity) -> Result<(), OrderError>
    where
        TId: IdProvider<LineItemData>,
        TQuantity: TryInto<Quantity, Error = OrderError>,
    {
        let &ProductData {
            id: product_id,
            price,
            ..
        } = product.to_data();

        if self.contains_product(product_id) {
            Err("product is already in order")?
        }

        let id = id_provider.id()?;
        let line_item = LineItemData {
            id: id,
            version: LineItemVersion::default(),
            product_id: product_id,
            price: price,
            quantity: quantity.try_into()?.0,
            _private: (),
        };

        self.line_items.push(line_item);

        Ok(())
    }
}

impl Entity for Order {
    type Id = OrderId;
    type Version = OrderVersion;
    type Data = OrderData;
    type Error = OrderError;
}

impl Entity for OrderLineItem {
    type Id = LineItemId;
    type Version = LineItemVersion;
    type Data = LineItemData;
    type Error = OrderError;
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::customers::model::test_data::default_customer;

    #[test]
    fn add_item_to_order() {
        let product_id = ProductId::new();
        let product = Product::new(product_id, "A title", 1f32).unwrap();

        let customer = default_customer();

        let order_id = OrderId::new();
        let mut order = Order::new(order_id, &customer).unwrap();

        let order_item_id = LineItemId::new();
        order.add_product(order_item_id, &product, 1).unwrap();

        assert_eq!(1, order.line_items.len());
        assert!(order.contains_product(product_id));
    }

    #[test]
    fn quantity_must_be_greater_than_0() {
        let mut order = Order::new(OrderId::new(), &default_customer()).unwrap();

        let product = Product::new(ProductId::new(), "A title", 1f32).unwrap();

        assert!(order.add_product(LineItemId::new(), &product, 0).is_err());

        order.add_product(LineItemId::new(), &product, 1).unwrap();
        let (order_data, mut line_item_data) = order.into_data();
        let mut order = OrderLineItem::from_data(order_data, line_item_data.pop().unwrap());

        assert!(order.set_quantity(0).is_err());
    }

    #[test]
    fn product_must_not_be_in_order_when_adding() {
        let mut order = Order::new(OrderId::new(), &default_customer()).unwrap();

        let product = Product::new(ProductId::new(), "A title", 1f32).unwrap();

        order.add_product(LineItemId::new(), &product, 1).unwrap();

        assert!(order.add_product(LineItemId::new(), &product, 1).is_err());
    }
}