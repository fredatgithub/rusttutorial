fn main() {
    println!("Let's learn Rust language");
    let x = 5;
    let name = "John";
    let age: i32 = 42;
    let name: String = String::from("Jean");
    println!("Name: {}, Age: {}", name, age);
    println!("x = {}", x);
    let mut y = 10; // mutable variable
    y = 15;
    println!("y = {}", y);
    let z = 20; // immutable variable
    let varInt32: i32 = 30;
    println!("z = {}", z);
    println!("varInt32 = {}", varInt32);

    let varInt64: i64 = 40;
    println!("varInt64 = {}", varInt64);

    let varFloat32: f32 = 3.14;
    println!("varFloat32 = {}", varFloat32);

    let varFloat64: f64 = 2.71828;
    println!("varFloat64 = {}", varFloat64);

    let varBool: bool = true;
    println!("varBool = {}", varBool);

    let varChar: char = 'A';
    println!("varChar = {}", varChar);

    let varString: String = String::from("Hello, Rust!");
    println!("varString = {}", varString);

    let age = 20;

    let status = if age >= 18 {
            "majeur"
        } else {
            "mineur"
    };

    for i in 0..10 {
        println!("{i}");
    }

    let mut count = 0;
    while count < 10 {
        count += 1;
    }

    println!("Count: {}", count);

}
