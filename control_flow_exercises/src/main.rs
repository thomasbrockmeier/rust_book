fn main() {
    temperatures();
    fibonaccis();
}

fn temperatures() {
    let temperatures = [32.0, 82.5, 100.0];

    for temperature in temperatures.iter() {
        println!("{}F is {}C", temperature, f_to_c(*temperature));
    }
}

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn fibonaccis() {
    let a = [2, 5, 10, 20];

    for element in a.iter() {
        println!("The {}th fibonacci number is {}", element, fib(*element));
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

