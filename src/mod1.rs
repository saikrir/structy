use std::{
    collections::{HashMap, HashSet},
    i64::MAX,
};

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

fn freq_map(word: &str) -> HashMap<char, i16> {
    let mut freq_map = HashMap::new();
    for ch in word.chars() {
        let freq_val = freq_map.entry(ch).or_insert(0);
        *freq_val = *freq_val + 1;
    }
    freq_map
}

fn compare_maps(map1: HashMap<char, i16>, map2: HashMap<char, i16>) -> bool {
    if map1.len() != map2.len() {
        return false;
    }
    for (k, v) in map1 {
        if !match map2.get(&k) {
            Some(&v2) => v2 == v,
            None => false,
        } {
            return false;
        }
    }
    true
}

pub fn most_frequent_char(some_str: &str) -> char {
    let f_map = freq_map(some_str);

    let mut m_char = '\0';
    for ch in some_str.chars() {
        if m_char == '\0' || f_map.get(&ch).unwrap() > f_map.get(&m_char).unwrap() {
            m_char = ch;
        }
    }
    m_char
}

pub fn anagram(word1: &str, word2: &str) -> bool {
    compare_maps(freq_map(word1), freq_map(word2))
}

pub fn pair_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut inverted_index: HashMap<i32, usize> = HashMap::new();

    for (idx, num) in numbers.iter().enumerate() {
        let req = target - num;

        if inverted_index.contains_key(&req) {
            return Some((*inverted_index.get(&req).unwrap(), idx));
        }
        inverted_index.insert(*num, idx);
    }
    None
}

pub fn pair_product(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut inverted_index: HashMap<i32, usize> = HashMap::new();

    for (idx, num) in numbers.iter().enumerate() {
        if target % *num != 0 {
            continue;
        }

        let req = target / num;

        if inverted_index.contains_key(&req) {
            return Some((*inverted_index.get(&req).unwrap(), idx));
        }
        inverted_index.insert(*num, idx);
    }
    None
}

pub fn intersection(numbersA: &[i32], numbersB: &[i32]) -> Vec<i32> {
    let n_set: HashSet<i32> = numbersA.iter().copied().collect();
    numbersB
        .iter()
        .filter(|v| n_set.contains(*v))
        .copied()
        .collect()
}

pub fn xor(numbersA: &[i32], numbersB: &[i32]) -> Vec<i32> {
    let a_set: HashSet<i32> = numbersA.iter().copied().collect();
    let b_set: HashSet<i32> = numbersB.iter().copied().collect();

    let mut final_vec = Vec::new();

    for n in numbersA {
        if !b_set.contains(n) {
            final_vec.push(*n);
        }
    }

    for n in numbersB {
        if !a_set.contains(n) {
            final_vec.push(*n);
        }
    }

    final_vec
}
