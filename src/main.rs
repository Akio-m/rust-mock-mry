#[derive(Debug, Default)]
struct Point(i32, i32);

fn main() {
    let point = Point(0, Default::default());
    println!("Hello, world! {:?}", point);
}
