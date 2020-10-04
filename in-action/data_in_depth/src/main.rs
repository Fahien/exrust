fn reinterpret() {
    // Nice reference
    let a: f32 = 42.42;

    // The same thing in C++ could be achieved with a reinterpret_cast
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    // Type u32 implements fmt::Binary, therefore we can print its bytes
    println!("u32 binary f32");
    print!("{} {:032b} ", frankentype, frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };
    println!("{}", b);
}

fn integer_overflow() {
    let mut i: u8 = 255;
    print!("{} == ", i);
    // This could be written using a binary literal
    i = 0b1111_1111;
    println!("{}", i);
    // This triggers a panic, cause the add will overflow
    // i += 1;
}

fn endianness() {
    // CPU architectures can order differently bytes making up integers

    // Some prefer sequences from left-to-right (Arm)
    // This stores the most significant byte at the smallest memory address
    // Here 0 is stored at position 0.
    let big_endian: [u8; 2] = [0x00, 0x01];

    // Other prefer sequences from right-to-left (Intel)
    // Here the least significant byte is at the smallest memory address
    // Indeed 1 is stored at position 0.
    let little_endian: [u8; 2] = [0x01, 0x00];

    let a: i16 = unsafe { std::mem::transmute(big_endian) };
    let b: i16 = unsafe { std::mem::transmute(little_endian) };

    println!("big-endian little-endian");
    println!("{} {}", a, b);

    if a == 1 {
        println!("This is a big-endian architecture");
    } else {
        println!("This is a little-endian architecture");
    }

    // How about bit-endianness?
    let big_endian: u8 = 0b0000_0001;
    if big_endian == 1 {
        println!("Big bit-endianness");
    } else {
        println!("Little bit-endianness");
    }
}

/// A 32 bit floating point number is made of 3 fields ((± signbit) mantissa ^ (2 ^ exponent)):
/// A signbit, which determines wheter the number is positive or negative
/// The exponent, biased 8 bits integer. Special cases: 0x00 (subnormal number); 0xFF (infinity)
/// The fraction, 23 bit fixed point number
fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
    let bits: u32 = unsafe { std::mem::transmute(n) };

    let signbit = bits >> 31;
    let exponent = (bits << 1) >> 24;
    let fraction = (bits << 9) >> 9;

    (signbit, exponent, fraction)
}

fn decode_f32_parts(signbit: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_one = (-1.0f32).powf(signbit as f32);

    let exponent = (exponent as i32) - 127; // bias
    let radix: f32 = 2.0;
    let exponent_part = radix.powf(exponent as f32);

    let mut mantissa_part: f32 = 1.0;
    for i in 0..23u32 {
        let bit = (1 << i) & fraction;
        if bit != 0 {
            mantissa_part += 2.0f32.powf((i as f32) - 23.0);
        }
    }

    (signed_one, exponent_part, mantissa_part)
}

fn floating_point() {
    let n: f32 = 42.42;

    let (signbit, exponent, fraction) = deconstruct_f32(n);
    println!(
        "{:1b}({}) - {:8b}({}) - {:23b}({})",
        signbit, signbit as f32, exponent, exponent as f32, fraction, fraction as f32
    );

    let (sign, exponent, mantissa) = decode_f32_parts(signbit, exponent, fraction);
    println!(
        "{} * {} * {} = {}",
        sign,
        exponent,
        mantissa,
        sign * exponent * mantissa
    );
}

/// Q format is a fixed-point number, developed by Texas Instrument for embedded computing devices.
/// Q7 means there are 1 sign bit and 7 bits for the number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127) // max?
        } else if n <= -1.0 {
            Q7(-128) // min?
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        n.0 as f64 / 128.0
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        n.0 as f32 / 128.0
    }
}

fn q_format() {
    // Out of bounds, we get the same value
    assert_eq!(Q7::from(10.0), Q7::from(1.0));
    assert_eq!(Q7::from(-10.0), Q7::from(-1.0));

    // f32
    let qf32 = Q7::from(0.5f32);
    assert_eq!(qf32, Q7(64));
    println!("Q::from(f32) = {:?}", qf32);

    // f64
    let qf64 = Q7::from(0.5);
    assert_eq!(qf64, Q7(64));
    println!("Q::from(f64) = {:?}", qf64);
}

fn main() {
    reinterpret();
    integer_overflow();
    endianness();
    floating_point();
    q_format();
}
