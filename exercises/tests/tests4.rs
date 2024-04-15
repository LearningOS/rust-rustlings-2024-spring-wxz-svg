// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Result<Self, String> {
        if width <= 0 || height <= 0 {
            Err(format!("Rectangle width and height cannot be negative! Given: ({}, {})", width, height))
        } else {
            Ok(Rectangle {width, height})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20).unwrap();
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    fn negative_width_returns_error() {
        // This test should check if creating a rectangle with negative width returns an error
        let result = Rectangle::new(-10, 10);
        assert!(result.is_err());
    }

    #[test]
    fn negative_height_returns_error() {
        // This test should check if creating a rectangle with negative height returns an error
        let result = Rectangle::new(10, -10);
        assert!(result.is_err());
    }
}
