fn main()
{
    //Stringss
    //String slices (&str)

    let some_string: &str = "Fixed Lentth String.";
    println!("The value of some_string is: \'{}\'", some_string);
 
    //Variable length strings
    let mut growable_string: String = String::from("This string grow");
    println!("The value of growable_string is: \'{}\'", growable_string);

    growable_string.push('s');
    println!("The value of growable_string is: \'{}\'", growable_string);

    growable_string.pop();
    println!("The value of growable_string is: \'{}\'", growable_string);

    growable_string.push_str("s, and keeps growing!");
    println!("The value of growable_string is: \'{}\'", growable_string);

    println!("Is the string empty? {}", growable_string.is_empty());
    println!("Is string length is {}", growable_string.len());
    println!("The string has {} bytes", growable_string.capacity());
    println!("Does the string contain the substring \'grow\'? {}", growable_string.contains("grow"));

    growable_string.push_str("       ");
    println!("Is string length with spaces is {}", growable_string.len());
    println!("Is string length after trim() is {}", growable_string.trim().len());

    let number = 6;
    let number_string = number.to_string();
    println!("The value of number_string is: \'{}\'", number_string);
    println!("Is the number really a string? {}", number.to_string() == "6");


    let my_name= "Justin Dörner".to_string();
    println!("My name is {}", my_name);

    let empty_string = String::new();
    println!("The length of the empty string is {}", empty_string.len());

    let s_1 = "Justin".to_string();
    let s_2 = "Dörner".to_string();
    let s_3 = format!("My first name is {} and my last name is {}", s_1, s_2);
    println!("{}", s_3);

    let string_1 = String::from("Justin");
    let string_2 = String::from("Dörner");

    let string_3 = format!("{} {}", string_1, string_2);
    println!("{}", string_3);
}
