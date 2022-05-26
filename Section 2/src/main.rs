fn main()
{
    //By default all variables are immutable in rust and cannot be changed.
    //Use the keyword 'mut' to make a variable mutable and capable of being changed.
    let mut x:i32 = 15; // i32 by default
    println!("The value of x is {}", x);
    
    x = 60;
    println!("The new value of x is {}", x);

    let y:i64 = 5 * 5;
    println!("The value of y is {}", y);

    let z:bool = true;
    println!("The value of z is {}", z);

    let a:f64 = 1.5;
    println!("The value of a is {}", a);

    let b:char = 'a';
    let c:char = '3';
    let d:char = '+';
    let e:char = '\u{288A}';
    println!("The value of b, c, dd, e is {:?}", (b, c, d, e));
    
    let f:&str = "Testing a string";
    println!("{}", f);
    println!("\n");

    println!("===================Data types and sizes for 'i'===================");
    println!("The minimum value of i8 is \t{} and the maximum size is \t\t\t\t\t\t{}", i8::MIN, i8::MAX);
    println!("The minimum value of i16 is \t{} and the maximum size is \t\t\t\t\t\t{}", i16::MIN, i16::MAX);
    println!("The minimum value of i32 is \t{} and the maximum size is \t\t\t\t\t{}", i32::MIN, i32::MAX);
    println!("The minimum value of i64 is \t{} and the maximum size is \t\t\t\t{}", i64::MIN, i64::MAX);
    println!("The minimum value of i128 is \t{} and the maximum size is \t{}", i128::MIN, i128::MAX);
    println!("===================================================================");
    println!("\n");

    println!("===================Data types and sizes for 'u'===================");
    println!("The minimum value of u8 is \t{} and the maximum size is \t{}", u8::MIN, u8::MAX);
    println!("The minimum value of u16 is \t{} and the maximum size is \t{}", u16::MIN, u16::MAX);
    println!("The minimum value of u32 is \t{} and the maximum size is \t{}", u32::MIN, u32::MAX);
    println!("The minimum value of u64 is \t{} and the maximum size is \t{}", u64::MIN, u64::MAX);
    println!("The minimum value of u128 is \t{} and the maximum size is \t{}", u128::MIN, u128::MAX);
    println!("===================================================================");
    println!("\n");

    println!("===================Data types and sizes for 'f'===================");
    println!("The minimum value of f32 is \t{} and the maximum size is \t{}", f32::MIN, f32::MAX);
    println!("The minimum value of f64 is \t{} and the maximum size is \t{}", f64::MIN, f64::MAX);
    println!("===================================================================");
    println!("\n");

    println!("The values of the following variables are: {:?}", (x, y, z, a, b));

    //Interger types in Rust: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    //Floating point types in Rust: f32, f64
    //Boolean types in Rust: bool
    //Character types in Rust: char
    //String types in Rust: String
    //Tuple types in Rust: (i32, i32, i32)
    //Array types in Rust: [i32; 5]
    //Slices in Rust: &[i32]
    fn type_of<T>(_: &T)
    {
        println!("{}", std::any::type_name::<T>())
    }

    type_of(&a);
    type_of(&b);
    type_of(&c);
    type_of(&d);
    type_of(&e);
    type_of(&f);

    type_of(&x);
    type_of(&y);
    type_of(&z);
}
