/*
 * Given a list of integers, use a vector and return the mean (the average
 * value), median (when sorted, the value in the middle position), and
 * mode (the value that occurs most often; a hash map will be helpful
 * here) of the list.
 */

mod stats;

fn main() {
    let list2 = vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 8, 8];
    let list = vec![4, 4, 4, 4, 5, 5, 5, 6];
    
    println!("{}, {}, {}", stats::mean(&list2), stats::mode(&list2), stats::median(&list2));
    println!("{}, {}, {}", stats::mean(&list), stats::mode(&list), stats::median(&list));
}
