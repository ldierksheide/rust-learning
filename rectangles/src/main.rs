#[derive(Debug)] //allows printing the struct usin the {:?} or {:#?} in println!
struct Rectangle {
    width: u32,
    height: u32,
}

//add a method associated w Rectangle struct (methods must always have self as 1st parameter)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
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
    let square1 = Rectangle::square(22);
    
    println!(
        "The area of the rectangle ({:?}) is {} square pixels",
        rect1,
        rect1.area()
    );

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    println!("a square! {:#?}", square1);
}

/*before adding the area method, used an area function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */