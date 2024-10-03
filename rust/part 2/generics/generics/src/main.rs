// -------------------------------------------
// 			Traits
// -------------------------------------------

// mod test_one;
//
// // struct drawing_info {
// //     line_width: u8,
// //     color: String,
// // }
// struct Square {
//     side: f32,
//     line_width: u8,
//     color: String,
//     //info: drawing_info,
// }
//
// struct Rectangle {
//     length: f32,
//     width: f32,
//     line_width: u8,
//     color: String,
//     // info: drawing_info,
// }
//
// // impl Square {
// //     fn calculate_area(&self) {
// //         println!("The area is: {}", self.side * self.side);
// //     }
// // }
//
// // impl Rectangle {
// //     fn area(&self) -> f32 {
// //         self.length * self.width
// //     }
// // }
//
// trait Draw {
//     fn draw_object(&self);
// }
//
// trait Shape: Draw {
//     fn area(&self) -> f32;
//     fn perimeter(&self) -> f32 {
//         println!("Perimeter not implemented, returning dummy value");
//         0.0
//     }
// }
//
// fn shape_properties<T>(object: impl Shape) {
//     object.area();
//     object.perimeter();
// }
//
// fn shape_properties_dynamic(object: Box<dyn Shape>) {
//     object.area();
//     object.perimeter();
// }
//
// fn shape_properties_where<T>(object: T)
// where
//     T: Shape,
// {
//     object.area();
//     object.perimeter();
// }
//
// fn return_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
//     if dimension.len() == 1 {
//         let sq = Square {
//             side: dimension[0],
//             line_width: 25,
//             color: "Red".to_string(),
//         };
//         Box::new(sq)
//     } else {
//         let rect = Rectangle {
//             length: dimension[0],
//             width: dimension[1],
//             line_width: 5,
//             color: "Red".to_string(),
//         };
//         Box::new(rect)
//     }
// }
//
// impl Draw for Rectangle {
//     fn draw_object(&self) {
//         println!("Drawing Rectangle");
//     }
// }
//
// impl Shape for Rectangle {
//     fn area(&self) -> f32 {
//         let area_of_rect = self.length * self.width;
//         println!("Rectangle area: {}", area_of_rect);
//         area_of_rect
//     }
//
//     fn perimeter(&self) -> f32 {
//         let perimeter_of_rect = 2.0 * (self.length + self.width);
//         println!("Rectangle Perimeter: {}", perimeter_of_rect);
//         perimeter_of_rect
//     }
// }
//
// impl Draw for Square {
//     fn draw_object(&self) {
//         println!("Drawing Square");
//     }
// }
//
// impl Shape for Square {
//     fn area(&self) -> f32 {
//         let area_of_square = self.side * self.side;
//         println!("Square area: {}", area_of_square);
//         area_of_square
//     }
// }
// fn main() {
//     let r1 = Rectangle {
//         width: 5.0,
//         length: 4.0,
//         line_width: 1,
//         color: String::from("Red"),
//     };
//
//     let s1 = Square {
//         side: 3.2,
//         line_width: 1,
//         color: String::from("Red"),
//     };
//
//     shape_properties_dynamic(Box::new(r1));
//     shape_properties_dynamic(Box::new(s1));
//
//     let shape = return_shape(vec![1.0, 1.0]);
//     println!("{:?}", shape.area());
// }


//
// // Problem 2: Try identifying the error in the code
// // Hint: The error is related to the concept of supertrait
//
// mod test_one;
//
// trait Size {
//     fn compute_size(&self) -> u16;
//     fn size_to_str(&self) -> String;
// }
//
// trait Printable : Size {
//     fn size_to_str(&self) -> String;
// }
//
// trait Comparable : Size {
//     fn print_greater(a: &Self, b: &Self) {
//         // Please note that Self on the line above means, that type which will be implementing the trait
//
//         let item1 = a.compute_size();
//         let item2 = b.compute_size();
//         if item1 > item2 {
//             println!("{} is greater than {}", a.size_to_str(), b.size_to_str());
//         } else if item2 > item1 {
//             println!("{} is greater than {}", b.size_to_str(), a.size_to_str());
//         } else {
//             println!("Both sizes are {}", a.size_to_str());
//         }
//     }
// }
//
// struct Book {
//     page: u16,
// }
//
// impl Size for Book {
//     fn compute_size(&self) -> u16 {
//         self.page
//     }
//
//     fn size_to_str(&self) -> String {
//         self.page.to_string()
//     }
// }
//
// impl Printable for Book {
//     fn size_to_str(&self) -> String {
//         format!("Book having {} pages", self.page)
//     }
// }
//
// impl Comparable for Book {
//
// }
//
// fn main() {
//     let book_1 = Book { page: 50 };
//     let book_2 = Book { page: 450 };
//     Comparable::print_greater(&book_1, &book_2);
// }


// Problem 2: Fix the code by making suitable changes to the signatures
// of the functions 'get_vehicle' and 'operate_vehicle'

// trait Vehicle {
//     fn start_engine(&self) -> String;
//     fn drive(&self) -> String;
// }
//
// struct Car;
//
// struct Bicycle;
//
// impl Vehicle for Car {
//     fn start_engine(&self) -> String {
//         "🚗 Engine started".to_string()
//     }
//
//     fn drive(&self) -> String {
//         "🚗 Driving the car".to_string()
//     }
// }
//
// impl Vehicle for Bicycle {
//     fn start_engine(&self) -> String {
//         "🚲 No engine to start for a bicycle".to_string()
//     }
//
//     fn drive(&self) -> String {
//         "🚲 Pedaling the bicycle".to_string()
//     }
// }
//
// fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle>  { // This line needs a fix
//     match vehicle_type {
//         "car" => Box::new(Car),
//         "bicycle" => Box::new(Bicycle),
//         _ => panic!("No vehicle of that type available"),
//     }
// }
//
// fn operate_vehicle(driver: &dyn Vehicle) { // This line needs a fix
//     println!("{}", driver.start_engine());
//     println!("{}", driver.drive());
// }
//
// fn main() {
//     // Do not change code in main
//     let my_vehicle = get_vehicle("car");
//     operate_vehicle(my_vehicle.as_ref());
//
//     let my_vehicle = get_vehicle("bicycle");
//     operate_vehicle(my_vehicle.as_ref());
// }


// Problem 3: Fix the code by adding a proper type annotation for the vector in main
//
// trait Vehicle {
//     fn start_engine(&self) -> String;
//     fn drive(&self) -> String;
// }
//
// struct Car;
//
// struct Bicycle;
//
// impl Vehicle for Car {
//     fn start_engine(&self) -> String {
//         "🚗 Engine started".to_string()
//     }
//
//     fn drive(&self) -> String {
//         "🚗 Driving the car".to_string()
//     }
// }
//
// impl Vehicle for Bicycle {
//     fn start_engine(&self) -> String {
//         "🚲 No engine to start for a bicycle".to_string()
//     }
//
//     fn drive(&self) -> String {
//         "🚲 Pedaling the bicycle".to_string()
//     }
// }
//
// fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
//     match vehicle_type {
//         "car" => Box::new(Car),
//         "bicycle" => Box::new(Bicycle),
//         _ => panic!("No vehicle of that type available"),
//     }
// }
//
// fn operate_vehicle(driver: &dyn Vehicle) {
//     println!("{}", driver.start_engine());
//     println!("{}", driver.drive());
// }
//
// fn main() {
//     let vehicle_1 = Car;
//     let vehicle_2 = Car;
//     let vehicle_3 = Bicycle;
//
//     let vehicles: Vec<&dyn Vehicle> = vec![&vehicle_1, &vehicle_2, &vehicle_3]; // Fix this line
//
//     for v in vehicles {
//         operate_vehicle(v);
//     }
// }


//----
// Derive Traits
// Marker Traits
//-----

// use std::cmp::PartialEq;
//
// trait Properties: PartialEq + Default + Clone {}
//
// #[derive(Debug, PartialEq, Default, Clone)]
// struct Student {
//     name: String,
//     age: u8,
//     sex: char
// }
//
// impl Properties for Student {}
//
// fn main() {
//     let s1 = Student{
//         name: "ABC".to_string(),
//         age: 35,
//         sex: 'M',
//     };
//     let s2 = Student{
//         name: "XYZ".to_string(),
//         age: 40,
//         sex: 'M',
//     };
//
//     println!("{:?}", s1);
//     println!("s1 equal to s2 {:?}", s1 == s2);
// }

//----
// Associated Types in Traits
//----
//
// #[derive(Debug)]
// struct Km {
//     value: u32,
// }
//
// #[derive(Debug)]
// struct Kmh {
//     value: u32,
// }
//
// #[derive(Debug)]
// struct Miles {
//     value: u32,
// }
//
// #[derive(Debug)]
// struct Mph {
//     value: u32,
// }
//
// trait DistanceThreeHours{
//     type Distance;
//     fn distance_in_three_hours(&self) -> Self::Distance;
// }
//
// impl DistanceThreeHours for Mph {
//     type Distance = Miles;
//     fn distance_in_three_hours(&self) -> Self::Distance{
//         Self::Distance{
//             value: self.value * 3,
//         }
//     }
// }
// impl DistanceThreeHours for Kmh {
//     type Distance = Km;
//     fn distance_in_three_hours(&self) -> Self::Distance{
//         Self::Distance{
//             value: self.value * 3,
//         }
//     }
// }
//
//
// fn main() {
//
//     let speed_Kmh = Kmh { value: 90 };
//     let distance_Kmh = speed_Kmh.distance_in_three_hours();
//     println!("The distance in three hours is {}", distance_Kmh.value);
//
//     let speed_Mph = Mph { value: 90 };
//     let distance_Mph = speed_Mph.distance_in_three_hours();
//     println!("The distance in three hours is {}", distance_Mph.value);
// }

//------
// Choosing Associated vs Generic Types
//------

trait Addition<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    println!("{:?}", p3);
}