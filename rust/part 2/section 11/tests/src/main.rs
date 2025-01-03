// struct ArrayProcessor<'a> {
//     data: &'a [i32],
// }
//
// impl<'a> ArrayProcessor<'a> {
//     fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
//         let previous_data = self.data;
//         self.data = new_data;
//         previous_data
//     }
// }
//
// fn main() {
//     let mut some_data = ArrayProcessor { data: &[4, 5, 6, 7] };
//     let previous_data = some_data.update_data(&[10,11,12,13]);
//     println!("{:?}", previous_data);
//     println!("{:?}", some_data.data);
// }

// ----------------------
// RefCell
// ----------------------

// use std::cell::RefCell;
// use std::rc::Rc;
//
// fn main() {
//
//     // this code will work with error because of 'a' was borrowed two times as not mutable
//     // let a = RefCell::new(10);
//     // let b = a.borrow();
//     // let c = a.borrow();
//     // let d = a.borrow_mut();
//     // println!("{} {}", b, c);
//
//     // dropping variables 'b' and 'c' before mut borrow and it will OK
//     // let a = RefCell::new(10);
//     // let b = a.borrow();
//     // let c = a.borrow();
//     // drop(b);
//     // drop(c);
//     // let mut d = a.borrow_mut();
//     // *d = 15;
//     // drop(d);
//     // println!("a => {:?} ", a.take());
//
//     //multiple owners of data
//     let a = Rc::new(RefCell::new(String::from("C++")));
//     let b = Rc::clone(&a);
//     *b.borrow_mut() = String::from("Rust");
//     println!("{:?}", a.take());
//
//
// }

// // Problem 1: fill in the code for TODO's
//
// use std::cell::RefCell;
// fn main() {
//     let data: RefCell<Option<i32>> = RefCell::new(Some(42));
//
//     // TODO: Use borrow_mut to safely modify the value inside the RefCell to None.
//
//     if !data.borrow().is_none() {
//         println!("Final value: {:?}", data.borrow());
//     } else {
//         println!("No value present.");
//     }
// }

// Problem 2: Fix the lines indicated in the code so that it compiles

// use std::cell::RefCell;
// struct Car {
//     model: String,
//     price: u32,
//     status: RefCell<&'static str>,
// }
//
// impl Car {
//     fn new(model: &str, price: u32) -> Self {
//         Car {
//             model: model.to_owned(),
//             price,
//             status: RefCell::new("Available"),
//         }
//     }
//
//     fn sold(&self) {
//         let new_status = match self.price {
//             0..=50000 => "Sold - Economy",
//             50001..=100000 => "Sold - Mid Range",
//             _ => "Sold - Luxury",
//         };
//         *self.status.borrow_mut() = new_status; // fix this line
//     }
// }
//
// fn main() {
//     let mut car = Car::new("Sedan", 75000);
//     car.sold();
//     println!("Car Status: {}", car.status.borrow()); // Fix this line
// }

// Problem 3: The code below will compile. However, it will panic at execution time.
// Task 1: Your first task is to add some code so that it does not panic at execution time
// Task 2: The value at the last print line will not be displayed.
// Instead of value, <Borrowed> will be displayed.
// Add appropriate code so that the value of x is being displayed.

// use std::cell::RefCell;
//
// fn main() {
//     let x = RefCell::new(5);
//     let x_ref1 = x.borrow();
//     let x_ref2 = x.borrow();
//     println!("x_ref1: {}, x_ref2: {}", x_ref1, x_ref2);
//     drop(x_ref1);
//     drop(x_ref2);
//     /* Code for Task 1 */
//
//     let mut x_ref3 = x.borrow_mut();
//     *x_ref3 = 6;
//     drop(x_ref3);
//     /* Code for Task 2 */
//
//     println!("Stored value: {:?}", x);
// }

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct File{
    active_user: u32
}

#[derive(Debug)]
struct User{
    file: Rc<RefCell<File>>
}

fn main() {
    let text_file = Rc::new(RefCell::new(File{active_user:0}));
    println!("Active users are {:?}",text_file.borrow().active_user);
    let user_1 = User{file: Rc::clone(&text_file)};
    user_1.file.borrow_mut().active_user+=1;
    println!("Active users are {:?}",text_file.borrow().active_user);
    let user_2 = User{file: Rc::clone(&text_file)};
    user_2.file.borrow_mut().active_user+=1;
    println!("Active users are {:?}",text_file.borrow().active_user);
}