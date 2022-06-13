fn main() {

    // Mutable variables
    let mut x = 10;
    println!("The value is {}", x);

    x = 20;
    println!("The value is {}", x);

    // Constant variables
    const MAX_POINT: u32 = 100_000;
    println!("The value is {}", MAX_POINT);

    // Shadowing
    let y = 10;
    let y = y + 1;
    println!("The value is {}", y);

    { // this is inner scope
        let y = y - 1;
        let y = y * 2;
        println!("The value is {}", y); //should be 20
    } // this is outer scope

    let x = "0"; // override value from int to str
    println!("The value is {}", x);
}
