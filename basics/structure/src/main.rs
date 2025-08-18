fn main () {
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40
    };
    let sq = Rectangle::square(21);
    println!("The area of the reactangle is {}", rect1.area());
    println!("Reactangle 3 can hold reactangle 1 {}", rect3.can_hold(&rect1));
    println!("Reactangle 1 can hold reactangle 2 {}", rect1.can_hold(&rect2));
    println!("Square can hold rectangle 1 {}", sq.can_hold(&rect1));
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
   fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    } 
    
}

