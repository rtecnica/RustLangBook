/*
 *
 *
 *
 *
 */

pub fn fib_gen(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fib_gen(num - 2) + fib_gen(num - 1)
    }
}
