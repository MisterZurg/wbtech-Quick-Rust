struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f32 {
        // d= √ ((x2 – x1)² + (y2 – y1)²)
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

fn main() {
    let first = Point::new(1.0, 1.0);
    let second = Point::new(4.0, 5.0);

    println!("{}", first.distance(&second))
}
