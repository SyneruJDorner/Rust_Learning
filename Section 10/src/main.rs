fn main()
{
    //Rust ownership
    //There is a copy ownership that occurs here with values due to it being a primitive type
    //Primitive are of fixed size and cannot grow
    let x = 32;
    let y = x;
    println!("The value for x = {} and the value for y = {}", x, y);

    //However there is no ownership transfer here with strings due to it being a non-primitive type
    //Non-primitive types are of variable size and can grow
    // let s1 = String::from("ABC");
    // let s2 = s1;
    // println!("The value for s1 = {} and the value for s2 = {}", s1, s2);

    let s1 = String::from("ABC");
    let s2 = &s1;   //Borrowing the value and creates a reference to point back to the s1 value without taking ownership
    println!("The value for s1 = {} and the value for s2 = {}", s1, s2);

    let num_vec1: Vec<i32> = vec![5, 6, 9, 8, 7];
    let num_vec2 = &num_vec1;
    println!("The value for num_vec1 = {:?} and the value for num_vec2 = {:?}", num_vec1, num_vec2);

    //How to clone value into a new variable
    let num_vec2 = num_vec1.clone();
    println!("The value for num_vec1 = {:?} and the value for num_vec2 = {:?}", num_vec1, num_vec2);

    //Ownership exists wwithin the scope of the variable, the variable outside the scope it will be dropped and cannot be used
    /*
    {
        let my_name = "Justin Dörner";
    }

    println!("The value for my_name = {}", my_name);
    */
}