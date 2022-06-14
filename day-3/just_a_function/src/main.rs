fn main() {
    just_a_function("World");
}

fn just_a_function(world: &str) {
    println!("Hello, {}", world);
    another_function(-6.1309453, 106.6833565);
}

fn another_function(lat: f32, long: f32) {
    println!("Your coordinate is {} , {}", lat, long);
    this_is_statement(12);
}

fn this_is_statement(age: i8) {
    println!("Your age is {}", age);
    this_is_expression();
}

fn this_is_expression() {
    let x = {
        let y = 1;
        y + 1 // don't put semicolon here. Expressions do not include ending semicolons
    };

    println!("Your queue is {}", x);

    let current_que: i8 = func_return_value();
    println!("Current queue is {}", current_que);
}

fn func_return_value() -> i8 {
    let x = 2;
    x - 1
}
