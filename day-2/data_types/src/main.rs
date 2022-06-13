fn main() {

    // u8 is unsigned 8-bit integer, using value 256 will cause error
    let int: u8 = 255;
    println!("The value is {}", int);

    let float: f32 = 3.14;
    println!("The value is {}", float);

    // MATH OPERATIONS

    // default price
    let price = 200.0;
    let disc: f32 = 0.1;

    // subtraction
    let price_after_disc = price - disc;
    println!("The price after discount is {}", price_after_disc);

    // addition
    let tax: f32 = 1.0;
    let price_after_tax = price + tax;
    println!("The price after tax is {}", price_after_tax);


    // remainder
    let price = price_after_tax as i32; // convert from f32 to i32
    let final_price = price - (price * 2 / 100);
    println!("The final price after service fee is {}", final_price);

    // boolean
    let min_price_add_disc = 150;

    if final_price > min_price_add_disc {
        let final_price_after_disc = final_price - (final_price * 5 / 100);
        let cat_char = '😻';  // rust support character value btw
        println!("You got extra 5% discount {}, you final price is {}", cat_char, final_price_after_disc);
    } else {
        println!("The final price is {}", final_price);
    }

}
