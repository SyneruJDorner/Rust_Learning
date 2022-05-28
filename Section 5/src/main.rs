fn main()
{
    //Tuples
    let my_info = ("Salary", 45_000);
    println!("My {} is {}", my_info.0, my_info.1);

    let (salary, salary_value) = my_info;
    println!("My {} is {}", salary, salary_value);

    let salary = my_info.0;
    let salary_value = my_info.1;
    println!("My {} is {}", salary, salary_value);

    let nested_tuple = ("Justin", "Dörner", ("salary", 45_000), "Programmer");
    let element = nested_tuple.2.0;
    println!("{}", element);
    println!("My name is {} {} and I am a {} and my {} is {}", nested_tuple.0, nested_tuple.1, nested_tuple.3, nested_tuple.2.0, nested_tuple.2.1);

    //Arays
    let mut num_array: [i32; 5] = [4, 5, 6, 7, 8];
    println!("Index 0 is {}", num_array[0]);
    println!("{:?}", num_array);
    println!("length of array is {}", num_array.len());
    println!("The array is occupying {} bytes", std::mem::size_of_val(&num_array));

    let check_index = num_array.get(100);
    println!("{:?}", check_index);
    let check_index = num_array.get(4);
    println!("check_index is {:?}", check_index);
    println!("check_index unwrapped is {:?}", check_index.unwrap());

    num_array[4] = 5;
    println!("{:?}", num_array);

    let array_with_some_elements = [0; 10];
    println!("{:?}", array_with_some_elements);

    let array_of_strings = ["Hello", "World", "!"];
    println!("{:?}", array_of_strings);

    let mut array_with_some_str_elem = ["Unknown"; 6];
    array_with_some_str_elem[0] = "Justin Dörner";
    println!("{:?}", array_with_some_str_elem);

    let char_array = ['a', 'p', 'p', 'l', 'e'];
    println!("{:?}", char_array);
    let subset_array = &char_array[0..3];
    println!("{:?}", subset_array);
    //subset_array[0] = 'b'; -> Cannot do this `subset_array` is a `&` reference, so the data it refers to cannot be written
}
