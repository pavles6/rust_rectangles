#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };

    let sq1 = Rectangle::square(15);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("can rect1 hold rect2? Answer: {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect1? Answer: {}", rect2.can_hold(&rect1));

    println!("square1: {:?}", sq1);
}
