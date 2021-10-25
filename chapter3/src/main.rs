/*
 *
 *
 *
 *
 *
 *
 *
 */

use std::io;
use std::io::Write;

mod homework;

fn main() {
    let c_temp = 20.0; // degrees celsius
    println!("Temp in C: {}", c_temp);
    println!("Temp in converted F: {}", homework::degrees::c_to_f(c_temp));
    println!(
        "Temp in double converted C: {}",
        homework::degrees::f_to_c(homework::degrees::c_to_f(c_temp))
    );

    let fib_num = 12;
    println!("\nFirst {} numbers of fibonacci sequence:", fib_num);

    for i in 0..fib_num {
        print!("{} ", homework::fibonacci::fib_gen(i));
        io::stdout().flush()
            .expect("Something went wrong!");
    }
    println!("");
}
