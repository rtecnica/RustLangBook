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

pub fn latinize(s: &str) -> &str {
    let ss = String::from(s);
    let mut idx = 0usize;
    for (i, c) in s.char_indices() {
        if let false = c.is_alphabetic() {
            let range = idx..i;
            let mut word = String::from(&s[range]);
            if let Some(x) = word.chars().next() {
               // Check if vowel or consonant
               //   - if vowel, add "-hay"
               //   - if consonant, check case
               //       - if lowercase 
            }
            idx = i + 1;
        }
    };

}
