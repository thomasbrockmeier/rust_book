const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);

    // Arithmetic between different types of scalars is not possible
    // They must be cast to compatible types first
    let x = 40 as f64 - 30.5;
    println!("x = {}", x);

    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]);
    println!("a[4] = {}", a[4]);
}
