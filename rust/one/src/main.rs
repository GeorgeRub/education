mod models;


fn main() {
    println!("Hello world!");
    let car = models::Car {
        model: String::from("AUDI"),
        color: String::from("red"),
    };
    let car2 = models::Car {
        model: String::from("BMW"),
        color: String::from("red"),
    };
    car.string();
    car2.string();
    assert_eq!(car.color, car2.color)
}
