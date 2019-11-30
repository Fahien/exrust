fn constant_declaration() {
    // Constant declaration
    const A_MILLION: u32 = 1_000_000;
    println!("A million = {}", A_MILLION);
}

fn shadowing() {
    // Shadowing
    let x = 1;
    println!("{}", x);
    let x = "one";
    println!("{}", x);
    let x = 1.0;
    println!("{}", x);
}

fn integers() {
    // 8-bit
    let x: i8 = 0;
    println!("{}", x);
    let x: u8 = 1;
    println!("{}", x);

    // 16-bit
    let x: i16 = 0;
    println!("{}", x);
    let x: u16 = 1;
    println!("{}", x);

    // 32-bit
    let x: i32 = 0;
    println!("{}", x);
    let x: u32 = 1;
    println!("{}", x);

    // 64-bit
    let x: i64 = 0;
    println!("{}", x);
    let x: u64 = 1;
    println!("{}", x);

    // 128-bit
    let x: i128 = 0;
    println!("{}", x);
    let x: u128 = 1;
    println!("{}", x);

    // arch
    let x: isize = 0;
    println!("{}", x);
    let x: usize = 1;
    println!("{}", x);
}

fn floating_points() {
    // Floating point
    let x = 0.0; // f64
    println!("{}", x);
    let x: f32 = 0.0;
    println!("{}", x);
}

fn literals() {
    // Decimal
    println!("{}", 98_222);
    // Hex
    println!("{}", 0xff);
    // Octal
    println!("{}", 0o77);
    // Binary
    println!("{}", 0b1111_0000);
    // Byte (u8 only)
    println!("{}", b'A');
}

fn tuples() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // Destructuring
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    // Array of 5 u32
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[1]);

    // Array of 2 elements initialized to 3
    let a = [3; 2];
    println!("{} {}", a[0], a[1]);
}

fn main() {
    constant_declaration();
    shadowing();
    integers();
    floating_points();
    literals();
    tuples();
    arrays();
}
