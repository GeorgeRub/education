mod shapes {
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: f64) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius cannot be negative"))
            }
        }

        pub fn new_2(radius: f64) -> Circle {
            match radius {
                ..=0.0 => panic!("radius cannot be negative"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Circle;
    // use super::*;
    #[test]
    fn larger_circle_should_contain_smaller() {
        let large_circle = Circle::new(5.0);
        let small_circle = Circle::new(2.0);

        assert_eq!(large_circle.contains(&small_circle), true, "Custom failure message");

        assert_ne!(large_circle.contains(&small_circle), false, "Custom failure message");

        assert!(large_circle.contains(&small_circle), "Custom failure message");
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let large_circle = Circle::new(5.0);
        let small_circle = Circle::new(2.0);
        assert_eq!(!small_circle.contains(&large_circle), true);
    }

    // #[test]
    // fn should_not_create_circle() -> Result<(), String> {
    //     let circle = Circle::new_1(-5.0)?;
    //     Ok(())
    // }

    #[test]
    #[should_panic(expected = "radius cannot be negative")]
    fn should_not_create_and_panic() {
        let circle = Circle::new_2(-11.0);
    }
}
