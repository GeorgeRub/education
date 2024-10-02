// -------------------------------------------
// 			Traits
// -------------------------------------------

mod test_one;

// struct drawing_info {
//     line_width: u8,
//     color: String,
// }
struct Square {
    side: f32,
    line_width: u8,
    color: String,
    //info: drawing_info,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
    // info: drawing_info,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

trait Draw {
    fn draw_object(&self);
}

trait Shape : Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

fn shape_properties<T>(object: impl Shape){
    object.area();
    object.perimeter();
}

fn shape_properties_where<T>(object: T) where T: Shape{
    object.area();
    object.perimeter();
}

fn return_shape() -> impl Shape {
    let sq = Square{
        side: 5.5,
        line_width: 25,
        color: "Red".to_string(),
    };
    sq
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}
fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: String::from("Red"),
    };

    let s1 = Square {
        side: 3.2,
        line_width: 1,
        color: String::from("Red"),
    };

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();
}


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
