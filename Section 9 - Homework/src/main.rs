////////////////////////////////PART 1////////////////////////////////
fn main(){
    // Step 1: Write a print statement for asking user to input the width of a rectangle
    println!("Please enter the width of a rectangle....");

    // Step 2: Write code for taking the input from the user of type u32 and store it in the variable of width
    let mut width_str: String = String::new();
    std::io::stdin().read_line(&mut width_str).expect("Failed to read line");
    let width:u32 = width_str.trim().parse().expect("Invalid input");

    // Step 3: Write a print statement for asking the user to input the length of a rectangle
    println!("Please enter the length of a rectangle....");

    // Step 4: Write code for taking the input from the user of type u32 and store it in the variable of length
    let mut length_str: String = String::new();
    std::io::stdin().read_line(&mut length_str).expect("Failed to read line");
    let length:u32 = length_str.trim().parse().expect("Invalid input");

    let resultant_area = {
        // Step 5: call a function area() inside here with inputs of width and length which will return the area
        area(width, length)
    };

    // Step 6: write code to print the resultant_area variable to the terminal
    println!("The resultant area of the rectangle is {}", resultant_area);
 }

 fn area(length:u32, width:u32) -> u32 {
    // Step 7: write the definition of the area here which is length * width and return the result
    return length * width;
 }
////////////////////////////////PART 1////////////////////////////////


////////////////////////////////PART 2////////////////////////////////
// fn main(){
//     println!("{}", elapsed_time_in_minutes(3, 20));
// }

// fn expected_minutes_in_oven() -> u32 {
//     // Step 1: write the definition of the function here which will return the expected minutes in oven
//     return 40;
// }

// fn remaining_minutes_in_oven(minutes_in_oven:u32) -> u32 {
//     // Step 2: write the definition of the function here which will return the remaining minutes in oven
//     return expected_minutes_in_oven() - minutes_in_oven;
// }

// fn preparation_time_in_minutes(number_of_toppings:u32) -> u32 {
//     // Step 3: write the definition of the function here which will return the preparation time in minutes
//     return number_of_toppings * 2;
// }

// fn elapsed_time_in_minutes(number_of_toppings:u32, minutes_in_oven:u32) -> u32 {
//     return preparation_time_in_minutes(number_of_toppings) + remaining_minutes_in_oven(minutes_in_oven);
// }
////////////////////////////////PART 2////////////////////////////////


////////////////////////////////PART 3////////////////////////////////
// fn main() {
//     println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));
// }

// fn print_distance(point: (f32, f32)) -> f32 {
//     //√(x − 0)2 + (y − 0)2
//     let dx = ((0.0 - point.0) as f32).powf(2.0);
//     let dy = ((0.0 - point.1) as f32).powf(2.0);

//     //Here we calculate p2.x − p1.x and p2.y − p1.y
//     //We also use the square root function to finally calculate the distance between the 2 points.
//     let distance = (dx + dy).sqrt() as f32;

//     //We ensure that the distance is absolute. As we can't have a negative distance.
//     let abs_distance = distance.abs();
//     return abs_distance;
// }
////////////////////////////////PART 3////////////////////////////////