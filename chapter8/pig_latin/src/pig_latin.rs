/*
 *
 *
 *
 *
 *
 */

#[test]
fn one_word_C() {
   assert_eq!("Ord-Way", latinize("Word"));
}

#[test]
fn one_word_c() {
   assert_eq!("ord-Way", latinize("word"));
}

#[test]
fn many_words() {
    assert_eq!("Ord-hay erd-hay Lerd-Fay", latinize("Word herd Flerd"));
}

#[test]
fn zero_word() {
   assert_eq!("", latinize(""));
}

pub fn latinize (s: &str) -> &str {
    s
}
