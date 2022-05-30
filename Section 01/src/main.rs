fn main()
{
    //This prints out the string "Hello, world!"

    /*
    This is
    a multiline
    comment
    */

    println!("Hello, world!");
    
    //Learning some basic output commands

    /*
    Ranging from
    -Basic printing
    -Printing with a format string
    -Printing with a format string and a variable
    -Printing over multiple lines
    -Printing with special characters
    -Printing new lines, tabs, and backspaces
    -Printing quotes and double quotes
    -Difference between println and print
    -Printing Ascii, hex, and binary
    */
    
    println!("The value of the constant is {}", 500);
    println!("My name is {} and my surname is {}", "Justin", "Dörner");

    print!("This line and the line below print on the same line! ");
    print!("This line and the line above print on the same line!");
    println!("");

    print!("This is going to be
                   printed on multiple
    lines!");

    println!("");
    println!("\n\nThis will be prionted after two lines! \tAnd there is a tab here >\t<. And this is how to print a backslack n \\n.");
    println!("This is how to print a single quote \' and this is how to print a double quote \"");
    println!("This is how to print a backslash \\");

    print!("This is some text which will be printed \rthis text will only appear.\n");

    println!("I am doing {2}, from {1}, and the temperature when writing this was {0}.", "23°C", "South Africa", "rust coding");

    println!("{langauge} is {value}", langauge = "Rust", value = "awesome");

    println!("The sum of 25 + 10 = {}", 25 + 10);

    println!("This is how to print out the ASCII of N => {}", 'N');
    println!("This is how to print out the ASCII of @ => {}", '@');
    println!("This is how to print out the binary of N => {:b}", 'N' as i8);
    println!("This is how to print out the binary of @ => {:b}", '@' as i8);
    println!("This is how to print out the hex of N => {:x}", 'N' as i8);
    println!("This is how to print out the hex of @ => {:x}", '@' as i8);
}
