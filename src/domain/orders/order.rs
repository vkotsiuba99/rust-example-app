use domain::products::{Product, ProductData};

pub type OrderError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: i32,
}

pub struct Order {
    data: OrderData
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderItemData {
    pub id: i32,
    pub product_id: i32,
    pub price: f32
}

pub struct OrderItem {
    data: OrderItemData
}

pub struct OrderAggregate {
    order: Order,
    order_items: Vec<OrderItem>,
}

impl Order {
    pub(in domain::orders) fn new(data: OrderData) -> Self {
        Order {
            data: data
        }
    }

    pub fn split(self) -> OrderData {
        self.data
    }
}

impl OrderItem {
    pub(in domain::orders) fn new(data: OrderItemData) -> Self {
        OrderItem {
            data: data
        }
    }

    pub fn split(self) -> OrderItemData {
        self.data
    }
}

impl OrderAggregate {
    pub fn contains_product(&self, product_id: i32) -> bool {
        self.order_items.iter().any(|item| item.data.product_id == product_id)
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), OrderError> {        
        let ProductData { id, .. } = product.into_data();

        if !self.contains_product(id) {
            let order_item = OrderItem::new(OrderItemData {
                id: 1,
                product_id: id,
                price: 1f32
            });

            self.order_items.push(order_item);

            Ok(())
        }
        else {
            Err("product is already in order")?
        }
    }
}