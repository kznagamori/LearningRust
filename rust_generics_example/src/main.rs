// ジェネリック型を持つ構造体
struct Point<T> {
    x: T,
    y: T,
}

// ジェネリック型を持つメソッド
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// ジェネリック型を持つ関数
fn print_point<T: std::fmt::Display>(point: &Point<T>) {
    println!("Point: ({}, {})", point.x(), point.y());
}

fn main() {
    let point_int = Point::new(5, 10);
    let point_float = Point::new(1.2, 3.4);

    print_point(&point_int);
    print_point(&point_float);
}
