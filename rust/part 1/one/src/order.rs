use crate::product::Product;
use crate::customer::Customer;

#[derive(Debug)]
pub struct Order {
    id: u64,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    fn calculate_discount(&self) -> f64 {
        if self.quantity > 5 {
            0.1
        } else {
            0.0
        }
    }

    pub fn total_bill(&self) -> f64 {
        let discount: f64 = self.calculate_discount();
        let total_before_discount = self.product.product_price() * self.quantity as f64;
        total_before_discount - (total_before_discount * discount)
    }

    pub fn new(id: u64,
               product: Product,
               customer: Customer,
               quantity: u32, ) -> Order {
        Order {
            id,
            product,
            customer,
            quantity,
        }
    }
}