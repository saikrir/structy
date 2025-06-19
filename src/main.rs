mod mod1;
mod mod2;

use mod1::{
    anagram, fizz_buzz, greet, intersection, is_prime, longest_word, max_value, most_frequent_char,
    pairs,
};
use mod2::{factorial, sum_number_recursive};

use crate::mod1::{all_unique, intersection_with_dupes, pair_product, pair_sum, xor};
use crate::mod2::{fibonocci, palindrome, reverse_string, sum_of_lengths};

fn main() {
    //let var = greet("Sai Katterishetty");
    //let var = max_value(&[-5, -2, -1, -11]);
    //let var = longest_word("the quick brown fox jumped over the lazy dog");
    //let var = is_prime(31);
    //let var = fizz_buzz(20);

    // let var = pairs(&[
    //     "cherry",
    //     "cranberry",
    //     "banana",
    //     "blueberry
    //     "lime",
    //     "papaya",
    // ]);

    //let var = most_frequent_char("mississippi");
    //let var = pair_product(&[4, 6, 8, 2], 16);
    //let var = xor(&[4, 2, 1, 6], &[3, 6, 9, 2, 10]);
    //let var = xor(&[4, 2, 1, 6], &[3, 6, 9, 2, 10]);
    //let var = all_unique(&["cat", "cat", "dog"]);
    //let var = intersection_with_dupes(&["a", "a", "a", "a", "a", "a"], &["a", "a", "a", "a"]);
    let var = fibonocci(8);
    println!("output {:?}", var);
}
