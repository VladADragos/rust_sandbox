mod fib;
mod vec;

fn main() {
    println!("Hello, world!");

    let t = String::from("132");
    t.capacity();
    let mut point1 = vec::Vec2d::new(4f64, 3f64);
    let mut point2 = vec::Vec2d::new(3f64, -2f64);
    let dist = point1.distance(point2);

    println!("{}", dist);
}
