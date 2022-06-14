fn main() {
    let mut x = 0;
    'counting_up: loop {
        println!("{}", x);
        x += 1;
        if x == 10 {
            break 'counting_up;
        }
    }

    println!("Done counting up!");

    just_a_while(25);
}

fn just_a_while(stop_at: i32) {
    let mut x: i32 = 0;
    while x < stop_at {
        println!("{}", x);
        x += 1;
    }

    println!("Done counting up with while!");

    just_a_for();
}

fn just_a_for() {
    let arr = [1, 2, 3, 4, 5];
    let mut i = 0;
    for el in arr {
        println!("{}", el);
        i += 1;
    }

    println!("Done counting up with for. The result is {}", i);

    just_a_rev();
}

fn just_a_rev() {
    for num in (1..4).rev() {
        println!("{}", num);
    }
}
