pub use one::{product::Product, customer::Customer, order::Order, category::Category};

#[test]
fn test_total_bill_without_discount() {
    let product1 = Product::new(1, String::from("Laptop"), 19.99, Category::Electronics);
    let customer = Customer::new(1, "George".to_string(), "george@gmail.com".to_string());
    let order1 = Order::new(2, product1, customer, 3);
    assert_eq!(format!("{:.2}", order1.total_bill()), "65.97");
}
#[test]
fn test_total_bill_with_discount() {
    let product1 = Product::new(1, String::from("Laptop"), 19.99, Category::Electronics);
    let customer = Customer::new(1, "George".to_string(), "george@gmail.com".to_string());
    let order1 = Order::new(2, product1, customer, 10);
    assert_eq!(format!("{:.2}", order1.total_bill()), "197.90");
}