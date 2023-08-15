trait Shape {
    fn area(&self) -> f32;
    fn as_string(&self) -> String;
}

struct Rectangle {
    width: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn as_string(&self) -> String {
        format!("Rectangle width={}, height={}", self.width, self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
    fn as_string(&self) -> String {
        format!("Circle radius={}", self.radius)
    }
}

pub fn main() {
    let mut shapes: Vec<&dyn Shape> = vec![];

    let r1 = Rectangle {
        width: 3f32,
        height: 4f32,
    };
    shapes.push(&r1);

    let r2 = Circle { radius: 10f32 };
    shapes.push(&r2);

    for &s in shapes.iter() {
        println!("{}", s.as_string());
        println!("Area: {}", s.area());
    }
}
