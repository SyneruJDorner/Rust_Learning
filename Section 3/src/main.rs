fn main()
{
    //Initializing multiple variables
    let (fist_num, second_num, third_num, fourth_num, fifth_num) = (1, 2, 3, 4, 5);
    println!("The fisrt number is {}", fist_num);
    println!("The second number is {}", second_num);
    println!("The third number is {}", third_num);
    println!("The fourth number is {}", fourth_num);
    println!("The fifth number is {}", fifth_num);

    //Readability of large number
    let large_number:u64 = 1_234_567_890;
    println!("The large number is {}", large_number);

    //Interger overflow
    //u8 may not exceed 255
    //let mut overflow_number:u8 = 256;
    let overflow_number:u8 = 255;
    println!("The overflow number is {}", overflow_number);

    //Decimal numbers in other formats
    let x = 255;
    println!("The value of the variable x in hexidecimal is {:o} and in octal is {:x} and in binary is {:b}", x, x, x);

    //Operations on numbers in different formats
    let n: i32 = 14;
    let m: f64 = 15.6;
    let o = n as f64 + m;
    println!("The value of n + m is {}", o);

    //Shadowing
    // let s: i32 = 5;
    // let s: i32 = 5 * 5;
    // println!("The value of s is {}", s);

    // let mut s: i32 = 5;
    // let s: i32 = 5 * 5;
    // println!("The value of s is {}", s);

    let s: i32 = 32;
    println!("The value of s is {}", s);

    let s: char = 'A';
    println!("The value of s is {}", s);

    let s: f64 = 64.5;
    println!("The value of s is {}", s);

    let s: bool = true;
    println!("The value of s is {}", s);

    //How to have a value return to its original value after the shadowing
    let s: i32 = 65;
    {
        let s: i32 = 60;
        println!("The value of s within the inner scope is {}", s);
    }
    println!("The value of s outside the inner scope is {}", s);

    //How to change a value in a scope
    let mut t = 31;
    println!("The value of s starts with {}", t);
    {
        t = 101;
        println!("The value of s within the inner scope is {}", t);
    }
    println!("The value of s outside the inner scope is {}", t);

    //Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
}
