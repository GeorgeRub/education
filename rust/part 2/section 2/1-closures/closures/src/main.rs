// //---------
// // Closures
// //---------
//
// struct User {
//     name: String,
//     age: u8,
//     salary: u32,
// }
//
// fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advanced_validator: V2) -> bool
// where
//     V1: Fn(&str) -> bool,
//     V2: Fn(u8) -> bool,
// {
//     simple_validator(name) && advanced_validator(age)
// }
//
// fn main() {
//     let user_1 = User {
//         name: "George".to_string(),
//         age: 36,
//         salary: 250000,
//     };
//
//     let validate_user_simple = |name: &str| -> bool {
//         name.len() != 0
//     };
//     let validate_user_advance = |age: u8| -> bool {
//         age >= 30
//     };
//     println!("Is valid Name: {}", validate_user_simple(&user_1.name));
//     println!("Is valid age: {}", validate_user_advance(user_1.age));
//     println!("Is user valid: {}",
//              is_valid_user(
//                  &user_1.name,
//                  user_1.age,
//                  validate_user_simple,
//                  validate_user_advance));
// }


// Problem 4: Fix the struct definition to allow closures with event handling logic.

struct EventHandler<T>
where
    T: Fn() -> () , // Something wrong here.
//Hint: Check the code in main and see how the closure is using
// the values from its enviroment
{
    on_event: T,
}

impl<T> EventHandler<T>
where
    T: Fn() -> (), // Something wrong here
{
    fn handle_event(&mut self) {
        (self.on_event)()
    }
}

fn main() {
    let mut lights_on = false;
    let mut temperature = 25;

    let mut lights_handler = EventHandler {
        on_event: || {
            lights_on = !lights_on;
            println!("Lights are now {}", if lights_on { "on" } else { "off" });
        },
    };

    let mut temperature_handler = EventHandler {
        on_event: || {
            temperature += 5;
            println!("Temperature increased to {}°C", temperature);
        },
    };

    lights_handler.handle_event();
    temperature_handler.handle_event();
    temperature_handler.handle_event();
    lights_handler.handle_event();

    assert_eq!(temperature, 35);
    assert_eq!(lights_on, true);
}
