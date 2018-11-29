extern crate rand;

use rand::Rng;

fn main() {
    let xs = [1, 2, 3, 4, 5];
    for x in &xs {
        print!("{} ", x);
    }
    println!("Hello, world!");

    println!("\nRandom numbers:");
    for x in 1..11 {
        let random_number = rand::thread_rng()
            .gen_range(1, 101);
        println!("{} -> {}", x, random_number)
    }
}