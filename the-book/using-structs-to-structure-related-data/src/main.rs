// Structs are easy
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // Even though email and username are reordered
    // The assignment works fine as variable and field names are exactly the same
    User {
        email,
        username,
        sign_in_count: 0,
        active: false,
    }
}

// Like tuples, but with a different type
fn tuple_structs() {
    struct Vec3(f32, f32, f32);
    struct Point(f32, f32, f32);

    // These are different types
    let x = Vec3(1.0, 1.0, 1.0);
    let one = Point(1.0, 0.0, 0.0);

    println!("Vec3 {} {} {}", x.0, x.1, x.2);
    println!("Color {} {} {}", one.0, one.1, one.2);
}

fn define_and_instantiate_structs() {
    // Easy to instantiate
    let mut me = User {
        username: String::from("fahien"),
        email: String::from("fake@mail.com"),
        sign_in_count: 0,
        active: true,
    };

    println!(
        "{} <{}> {} {}",
        me.username, me.email, me.sign_in_count, me.active
    );

    // Dot notation
    me.email = String::from("another@email.com");

    let mut _other = build_user(String::from("other@email"), String::from("other"));

    // Fill the remaining field with those from me
    _other = User {
        username: String::from("zaphod"),
        email: String::from("zaph@magrathea.planet"),
        ..me
    };

    tuple_structs();
}

fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

fn rectangle_area_calculator() {
    // These are related
    let width = 30;
    let height = 50;
    println!(
        "Rectangle area is {} square pixels",
        area_simple(width, height)
    );

    // How about a tuple?
    let rectuple = (width, height);
    println!("Rectangle area is {} square pixels", area_tuple(rectuple));

    // Structs add more meaning
    let rect = Rect { width, height };
    println!("Rectangle area is {} square pixels", area(&rect));

    // Nice printing when Rect implements std::fmt::Display
    //println!("My rect is {}", rect);

    // Debug printing when Rect implements stf::fmt::Debug or has annotation #[derive(Debug)]
    println!("My rect is {:?}", rect);
    // Different style
    println!("My rect styled {:#?}", rect);
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions are cool
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn method_syntax() {
    let rect = Rect {
        width: 20,
        height: 30,
    };

    let small = Rect {
        width: 10,
        height: 5,
    };

    let big = Rect {
        width: 30,
        height: 40,
    };

    // We can do better. Instead of a free function, we can use methods!
    println!("Rectangle area is {} square pixels", rect.area());

    println!("Rect can hold the smaller one? {}", rect.can_hold(&small));
    println!("Rect can hold the bigger one? {}", rect.can_hold(&big));

    // Let's create a square with an associated function
    let square = Rect::square(4);
    println!("We have made a square: {:?}", square);
}

fn main() {
    define_and_instantiate_structs();
    rectangle_area_calculator();
    method_syntax();
}
