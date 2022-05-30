fn main()
{
    ////////////////////////////////PART 1////////////////////////////////
    let p1 = (5, 6);    //Position 1 on graph (x-axis, y-axis)
    let p2 = (-7, 11);  //Position 2 on graph (x-axis, y-axis)

    //Code must break down to this
    // dx = (x₂ - x₁)
    // dy = (y₂ - y₁)

    //Calculate difference between 2 points.
    //We need to cast the values of dx and dy to f32 as there are potential decimal values in the future.
    let dx = (p2.0 - p1.0) as f32;
    let dy = (p2.1 - p1.1) as f32;
    println!("absolute difference of x is {}", dx.abs()); //We just must not forget to make X absolute
    println!("absolute difference of y is {}", dy.abs()); //We just must not forget to make Y absolute
    ////////////////////////////////PART 1////////////////////////////////


    ////////////////////////////////PART 2////////////////////////////////
    let p1: [f64; 2] = [5.0, 6.0];      //Position 1 on graph (x-axis, y-axis)
    let p2: [f64; 2] = [-7.0, 11.0];    //Position 2 on graph (x-axis, y-axis)

    //Code must break down to this
    // dx = (x₂ - x₁)
    // dy = (y₂ - y₁)

    //Calculate difference between 2 points.
    //We need to cast the values of dx and dy to f32 as there are potential decimal values in the future.
    let dx = (p2[0] - p1[0]) as f32;
    let dy = (p2[1] - p1[1]) as f32;

    println!("absolute difference of x is {}", dx.abs()); //We just must not forget to make X absolute
    println!("absolute difference of y is {}", dy.abs()); //We just must not forget to make Y absolute
    ////////////////////////////////PART 2////////////////////////////////


    ////////////////////////////////PART 3////////////////////////////////
    let p1 = (4.0, 3.0);    //Position 1 on graph (x-axis, y-axis)
    let p2 = (5.0, 4.5);  //Position 2 on graph (x-axis, y-axis)

    //Code must break down to this
    // √[(x₂ - x₁)² + (y₂ - y₁)²]

    //Calculate distance between 2 points.
    //We need to cast the values of dx and dy to f32 as there are potential decimal values in the future.
    //We also Calculate the power of the values of dx and dy.
    let dx = ((p2.0 - p1.0) as f32).powf(2.0);
    let dy = ((p2.1 - p1.1) as f32).powf(2.0);

    //Here we calculate p2.x − p1.x and p2.y − p1.y
    //We also use the square root function to finally calculate the distance between the 2 points.
    let distance = (dx + dy).sqrt() as f32;

    //We ensure that the distance is absolute. As we can't have a negative distance.
    let abs_distance = distance.abs();
    println!("The absolute distance between the two points is {}", abs_distance);
    ////////////////////////////////PART 3////////////////////////////////
}

