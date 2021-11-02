/*
 *
 *
 *
 *
 */
use std::collections::HashMap;


pub fn mean(v: &Vec<i32>) -> f64 {
    let mut mean = 0;
    for x in v {
        mean += x
    }
    mean as f64 / v.len() as f64
}

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut counter = HashMap::new();
    for x in v {
        match counter.get(&x) {
            Some(&t) => counter.insert(x, t + 1),
            _ => counter.insert(x, 1),
        };
    }
    
    let mut hold = (0i32, 0i32);
    for (&x, k) in counter{
        if k > hold.1 {
            hold = (x, k);
        }
    };
    hold.0
}

pub fn median(v: &Vec<i32>) -> i32 {
    let mut new_v = v.clone();
    new_v.sort();
    let l2 = (new_v.len() as f64 / 2.0_f64) as i32;
    new_v[l2 as usize] 
}

#[test]
fn basic () {
    let list = vec![4, 4, 4, 4, 5, 5, 5, 6, 6];
    assert_eq!(4.777777777777778, mean(&list));
    assert_eq!(4, mode(&list));
    assert_eq!(5, median(&list));

    let list2 = vec![4, 4, 4, 4, 5, 5, 5, 6];
    assert_eq!(4.625, mean(&list2));
    assert_eq!(4, mode(&list2));
    assert_eq!(5, median(&list2));
}
