use std::io;
//Fibonacci Series (User Input)
fn main() {

    println!("Enter the number of terms (n):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap_or(0);

    if n == 0 {
        println!("No terms to display.");
        return;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    print!("Fibonacci Series: ");
    for i in 0..n {
        if i == 0 {
            print!("{}", a);
        } else if i == 1 {
            print!(" {}", b);
        } else {
            let c = a + b;
            print!(" {}", c);
            a = b;
            b = c;
        }
    }
    println!();
}
