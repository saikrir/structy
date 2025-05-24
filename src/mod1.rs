use std::i64::MAX;

pub fn greet(name: &str) -> String {
    format!("hey {}", name)
}

pub fn max_value(numbers: &[i64]) -> i64 {
    let mut max: i64 = -MAX;

    for n in numbers {
        if *n > max {
            max = *n;
        }
    }
    max
}

pub fn longest_word<'a>(sentence: &'a str) -> &'a str {
    let mut longest = "";
    for word in sentence.split_ascii_whitespace() {
        if word.len() >= longest.len() {
            longest = word;
        }
    }
    longest
}

pub fn fizz_buzz(num: i32) -> Vec<String> {
    let mut ret = Vec::new();

    for i in 1..=num {
        let is_div_by_3 = i % 3 == 0;
        let is_div_by_5 = i % 5 == 0;

        let push_str = if is_div_by_3 && is_div_by_5 {
            "fizzbuzz".to_owned()
        } else if is_div_by_3 {
            "fizz".to_owned()
        } else if is_div_by_5 {
            "buzz".to_owned()
        } else {
            i.to_string()
        };
        ret.push(push_str);
    }
    ret
}

pub fn is_prime(num: i32) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..=num.isqrt() {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn pairs<'a>(input: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut ret = Vec::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            ret.push((input[i], input[j]));
        }
    }
    ret
}
