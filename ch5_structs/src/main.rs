#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block
// Implement the method area on the Rect struct
// we pass &self as a parameter which represents
// the instance of Rect the method is being called on
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method with another parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.area() >= other.area()
        self.width > other.width && self.height > other.height
    }

    // Associated functions (not methods since they don't have an instance of the struct to work with)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let w = 10;
    // let h = 20;

    // println!("Area of Rect is {} sq pixels", area(w, h));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of Rect is {} sq pixels", rect1.area());
    println!("rect1 is: {:#?}", rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Making a square: {:#?}", Rectangle::square(20));
}

// fn area(rectangle: &Rect) -> u32 {
//     rectangle.width * rectangle.height
// }
