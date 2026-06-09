fn main() {
    let x = 2.0;

    let y: f32 = 3.0;

    // char
    let c = 'z';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    println!("The value of x is: {0}, {1}", tup.0, tup.1);

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {0:?}", a)
}
