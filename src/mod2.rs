// Recursive

pub fn sum_number_recursive(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        return 0;
    }
    if numbers.len() == 1 {
        return numbers[0];
    }

    numbers[0] + sum_number_recursive(&numbers[1..])
}

pub fn factorial(n: i32) -> i64 {
    if n == 0 {
        return 1;
    }

    (n as i64) * factorial(n - 1)
}

pub fn sum_of_lengths(words: &[&str]) -> i32 {
    if words.is_empty() {
        return 0;
    }

    if words.len() == 1 {
        return words[0].len() as i32;
    }

    words[0].len() as i32 + sum_of_lengths(&words[1..])
}

pub fn reverse_string(s: &str) -> String {
    if s.is_empty() || s.len() == 1 {
        return s.to_string();
    }

    [&reverse_string(&s[1..]), &s[0..1]].concat()
}

pub fn palindrome(s: &str) -> bool {
    if s.len() <= 2 {
        return true;
    }

    if s.chars().next().unwrap() != s.chars().last().unwrap() {
        return false;
    }

    palindrome(&s[1..s.len() - 1])
}

pub fn fibonocci(n: i8) -> i32 {
    if n == 0 || n == 1 {
        return n as i32;
    }

    fibonocci(n - 1) + fibonocci(n - 2)
}
