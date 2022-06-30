## Function

- Rust code uses *snake case* as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words
- Statement don’t return a value. In Rust, statement is does not bind to anything

```jsx
fn main() {
	let x = (let y=1) // it won't work tho. Not like C++
}
```

- Expression ofc will return a value ( i know )

```jsx
fn main() {
	let x = {
		let y = 1;
		y + 1 //don't put semicolon here. Expressions do not include ending semicolons
 }
}
```

- Expressions do not include ending semicolons

## Control Flow

- In rust we can use `if-else` `loop` `while` or `for`
- We can define the label for `loop` expression if we have sub-loop inside a loop. To tell to the program to break in outer loop instead inner loop

```jsx
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

- Put a variable after `break` to returning a value: `break counter * 2;`

# Terms

- Control Flow = The ability to run some code depending on if a condition is true. In rust we can use `if-else` expression or `loop` expression
- Mutable is **a type of variable that can be changed**
- Immutable is the opposite