fn max_of_two_numbers(number1: i32, number2: i32) -> i32 {
    if number1 > number2 {
        return number1;
    } else {
        return number2;
    }
}

const WORDS: [&str; 10] = [
    "item",
    "vote",
    "return",
    "prove",
    "ridge",
    "social",
    "recommendation",
    "terrace",
    "provide",
    "advocate",
];

fn find_longest_word<'a>(arr: &'a [&'a str]) -> &'a str {
    let mut longest_word = "";

    for word in arr {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }

    longest_word
}

const NUMBERS: [i32; 10] = [234, 21354, 123, 1243, 213, 645, 756, 657, 546, 5];

fn sum_numbers<'a>(arr: &'a [i32]) -> i32 {
    let mut full_sum = 0;

    for num in arr {
        full_sum += num
    }

    full_sum
}

fn sum() {}

fn average_numbers() {}

fn average_length() {}

fn avg() {}

fn uniquidy_array() {}

fn does_word_exist() {}

fn how_many_times() {}

fn greatest_product() {}

fn main() {
    let biggest = max_of_two_numbers(142, 453);
    let longest = find_longest_word(&WORDS);
    let sum = sum_numbers(&NUMBERS);
    println!(
        "Functions and arrays lab, biggest number is {biggest}, longest is {longest}, sum is {sum}"
    );
}
