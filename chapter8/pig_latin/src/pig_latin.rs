/*
 *
 *
 *
 *
 *
 */

#[test]
fn one_word_uc() {
    assert_eq!("Ord-way", latinize("Word"));
}

#[test]
fn one_word_lc() {
    assert_eq!("ord-way", latinize("word"));
}

#[test]
fn many_words() {
    assert_eq!("Ord-way erd-hay Lerd-fay", latinize("Word herd Flerd"));
}

#[test]
fn zero_word() {
    assert_eq!("", latinize(""));
}

pub fn latinize(s: &str) -> String {
    let mut ss = String::from("");
    let mut idx = 0usize;
    let mut range;
    let mut idx2 = 0usize;
    for (i, c) in s.char_indices() {
        if false == c.is_alphabetic() {
            range = idx..i;
            if range.len() < 2 {
            } else {
                let mut word = String::from(&s[range.clone()]);
                ss.push_str(latinize_word(&mut word));
            }
            ss.push(c);
            idx = i + 1;
        }
        idx2 = i;
    }
    range = idx..idx2 + 1;
    if range.len() > 2 {
        let mut word = String::from(&s[range.clone()]);
        ss.push_str(latinize_word(&mut word));
    }
    ss
}

fn latinize_word(word: &mut String) -> &mut String {
    let x = word.remove(0);
    {
        match x.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                word.insert(0, x);
                word.push_str("-ay");
            }
            _ => {
                word.push_str(&(format!("-{}ay", x.to_ascii_lowercase())[..]));
            }
        };

        if x.is_uppercase() {
            word.replace_range(
                0..1,
                &format!("{}", word.chars().next().unwrap().to_ascii_uppercase())[..],
            );
        };
    }
    word
}
