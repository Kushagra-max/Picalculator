use std::io;


fn main() {
    println!("This will calcualte pi up to n digits using the Greogry-Lebinz series.");
    let mut n = String::new();
    let mut a:f64;
    let mut b:f64 = 1.0;
    let mut sum:f64 = 0.0;
    let mut count:u128 = 1;

    io::stdin()
        .read_line(&mut n)
        .expect("failed to read");
    let n:u128 = n.trim().parse().expect("wrong");



    while n > count {
        a = 4.0/b + sum;
        sum = a;
        if b.signum() == 1.0 {
            b += 2.0;
            b = -b;
    }
        else if b.signum() == -1.0 {
            b -= 2.0;
            b = b.abs();
    }
        println!("{a}");
        count+= 1;
    }






}
