fn main() {
    // ************ if expressions ************
    println!("************ if expressions ************" );
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

// ************ Error in if conditions ************
    //  let number = 3;

    //     if number {
    //         println!("number was three");
    //     }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

// ************ if-else block  ************
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // ************ using if in let statements ************
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

}
