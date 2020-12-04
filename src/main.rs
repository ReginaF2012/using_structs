#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 44,
    };

    println!(
        "The area of the first rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "Rectangle 1 can hold rectangle 2: {}", rect1.can_hold(&rect2)
    );

    println!(
        "Rectangle 1 can hold rectangle 3: {}", rect1.can_hold(&rect3)
    );
}