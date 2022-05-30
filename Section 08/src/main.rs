fn main()
{
    //Functions
    basic_fn();
    fn_with_inputs("Justin Dörner", 18);
    println!("{:?}", fn_with_inputs_outputs(5, 5));

    let (multi, div, add, sub) = fn_with_inputs_outputs(5, 5);
    println!("multiplication is {}, division is {}, addition is {}, subtraction is {}", multi, div, add, sub);

    let result = fn_with_inputs_outputs(5, 5);
    println!("multiplication is {}, division is {}, addition is {}, subtraction is {}", result.0, result.1, result.2, result.3);

    let fullname = {
        let firstname = String::from("Justin");
        let lastname = String::from("Dörner");
        format!("{} {}", firstname, lastname)
    };

    println!("My full name is {}", fullname);

    //Inputs from user
    println!("Please enter a value....");
    let mut input_val: String = String::new();
    std::io::stdin().read_line(&mut input_val).expect("Failed to read line");
    let n:f64 = input_val.trim().parse().expect("Invalid input");
    println!("{:?}", n);

    println!("Please enter a string....");
    let mut input_val: String = String::new();
    std::io::stdin().read_line(&mut input_val).expect("Failed to read line");
    println!("{:?}", input_val);
}

fn basic_fn()
{
    println!("This is a basic function!");
}

fn fn_with_inputs(name: &str, age: u8)
{
    println!("Hello {}! You are {} years old.", name, age);
}

fn fn_with_inputs_outputs(num1: i32, num2: i32) -> (i32, i32, i32, i32)
{
    return (num1 * num2, num1 / num2, num1 + num2, num1 - num2);
}