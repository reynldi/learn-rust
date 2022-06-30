fn main() {
    let s = "Hello"; // s is valid. It will be stored into memory stack
    // let b = s + "World"; we can't concat this variable since it's immutable
    println!("{}", s); // do the stuff with s

    // the s scope is now over. and s is no longer valid "pop-off" from the stack

    // -------------------------------------- //
    let mut b = String::from("Hello"); // b is valid. It will be stored into heap
    b.push_str(", World"); // it's mutable. b is still valid
    println!("{}", b); //  b is still valid

    // the b scope is now over. and b is no longer valid. Rust will calls a function
    // called `drop` to deallocating memory heap.
    // It's similar with Resource Acquisition Is Initialization (RAII) in C++

    let x = String::from("Halo"); // x is valid. It will be stored into heap
    let y = x.clone(); // copied x into y. Y stored into heap
    println!("{} Dunia", x); // x still valid

    // x out of scope

    println!("{}", y); // y still valid. But cannot refer to x since it's already dropped

    // y is out of scope, showing `double free error`
    // The condition when two variables conflicting to free-ing up them self
    // To solve the problem, use can use shallow copy
    // let y = x.clone();
}
