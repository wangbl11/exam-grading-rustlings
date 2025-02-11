struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn get_xy(re: &Rectangle, mode: i8) -> i32{
  if mode==1{
    return re.width
  }else{
    return re.height
  }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: This test should check if the rectangle has the size that we
        // pass to its constructor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(get_xy(&rect,1), 10); // Check width
        assert_eq!(get_xy(&rect,0), 20); // Check height
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative width.
    #[test]
    #[should_panic(expected="Rectangle width and height must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: This test should check if the program panics when we try to create
    // a rectangle with negative height.
    #[test]
    #[should_panic(expected="Rectangle width and height must be positive")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
