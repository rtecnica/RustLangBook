/*
 * Convert strings to pig latin. The first consonant of each word is moved
 * to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
 * Words that start with a vowel have “hay” added to the end instead
 * (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
 * encoding!
 */

/*
 * Parse each line for words
 * Pop first letter of word:
 *  vowel => add '-hay' to end
 *  consonant(c) => add -<c>ay
 *  Capitalize first letter if popped letter was capitalized
 * Move to next word
 * End
 */

mod pig_latin;

fn main() {
    let s = "The Dungeness crab, Metacarcinus magister or Cancer magister,\\
             is a species of crab that inhabits eelgrass beds and water bottoms \\
             on the west coast of North America. It typically grows to 20 centimetres \\
             across the carapace and is a popular seafood. Its common name comes from \\
             the port of Dungeness, Washington, United States, where it is 'a prized \\
             crustacean that supports the most valuable fishery on the west coast', and \\
             where ocean acidification threatens the marine environment.";

}

