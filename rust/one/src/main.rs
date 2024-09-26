mod customer;
mod product;
mod order;
mod category;
use crate::{customer::Customer, product::Product, order::Order, category::Category};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, "George".to_string(), "george@gmail.com".to_string());
    let order = Order::new(1, product, customer, 6);
    println!("Total coast {}:", order.total_bill());
}
