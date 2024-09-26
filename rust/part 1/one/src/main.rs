mod customer;
mod product;
mod order;
mod category;

use crate::{customer::Customer, product::Product, order::Order, category::Category};
use array_tool::vec::*;

fn main() {
    let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let product2 = Product::new(2, String::from("T-Shirt"), 799.99, Category::Clothing);
    let product3 = Product::new(3, String::from("Book"), 799.99, Category::Books);

    let set1: Vec<&Product> = vec![&product1, &product2];
    let set2: Vec<&Product> = vec![&product2, &product3];

    let intersection = set1.intersect(set2);
    println!("Intersection: {:?}", intersection);

}
