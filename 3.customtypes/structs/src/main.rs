#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let Rectangle { p1, p2 } = r;

    let Point { x: p1x, y: p1y } = p1;
    let Point { x: p2x, y: p2y } = p2;
    (p2x - p1x).abs() * (p2y - p1y).abs()
}

fn square(p: Point, r: f32) -> Rectangle {
    let Point { x: my_x, y: my_y } = p;
    Rectangle {
        p1: p,
        p2: Point {
            x: my_x + r,
            y: my_y + r,
        },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({},{})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({},{})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!(
        "rectangle area {}",
        rect_area(Rectangle {
            p2: Point { x: 0.7, y: 0.9 },
            .._rectangle
        })
    );

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let r1 = square(Point { x: 0.3, y: 0.4 }, 4f32);

    println!("square {:?}, {:?}", r1.p1, r1.p2);
}
