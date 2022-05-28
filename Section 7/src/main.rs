fn main()
{
    //Vectors
    let mut num_vec: Vec<i32> = vec![4,5,6,8,9,10,11,12,15,16,12,10];
    println!("{:?}", num_vec);
    println!("length of vector is {}", num_vec.len());
    println!("The vector is occupying {} bytes", std::mem::size_of_val(&num_vec));
    println!("The vector is empty: {}", num_vec.is_empty());
    println!("Index 0 is {}", num_vec[0]);

    num_vec[4] = 5;
    println!("{:?}", num_vec);

    let array_with_some_elements = vec![0; 10];
    println!("{:?}", array_with_some_elements);

    let array_of_strings = vec!["Hello", "World", "!"];
    println!("{:?}", array_of_strings);

    let mut array_with_some_str_elem = vec!["Unknown"; 6];
    array_with_some_str_elem[0] = "Justin Dörner";
    println!("{:?}", array_with_some_str_elem);

    let char_array = vec!['a', 'p', 'p', 'l', 'e'];
    println!("{:?}", char_array);

    let subset_array = &char_array[0..3];
    println!("{:?}", subset_array);

    let check_index = num_vec.get(100);
    println!("{:?}", check_index);
    let check_index = num_vec.get(4);
    println!("check_index is {:?}", check_index);
    println!("check_index unwrapped is {:?}", check_index.unwrap());

    num_vec.push(30);
    num_vec.push(40);
    println!("{:?}", num_vec);

    num_vec.remove(5);
    println!("{:?}", num_vec);

    println!("Does the value of 10 exist in the vector: {}", num_vec.contains(&10));
}
