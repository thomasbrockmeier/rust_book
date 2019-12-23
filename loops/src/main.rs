fn main() {
    return_from_loop();
    conditional_loop();
    loop_over_collection();
    loop_over_range();
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn conditional_loop() {
    let mut number = 3;

    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn loop_over_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}", element);
    }
}

fn loop_over_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");
}
