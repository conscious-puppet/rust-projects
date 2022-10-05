fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}"); // x: 2
    println!("y: {y}"); // y: 3

    if x < y {
        println!("x < y");
    } else {
        println!("x > y");
    }

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}");

    let a = [1, 2, 3, 4, 5];
    let _b = [3; 5];

    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");
}
