fn main() {
    println!("Hello, world!");

    another_function(666, 420);
    scope_function();
    println!("x = {}", return_type());
}

fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

fn scope_function() {
    let x = 5;
    let y = {
        let x = 5;
        x + 1
    };

    println!("x = {}", x);
    println!("y = {}", y);
}

fn return_type() -> i32 {
    5
}
