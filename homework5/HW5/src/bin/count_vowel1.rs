//Ordinary Loop
fn count_vowels(s: &str) -> usize{
    let vowels: Vec<char> = "aeiouAEIOU".chars().collect();
    let mut count = 0;

    for c in s.chars() {
        if vowels.contains(&c) {
            count += 1
        }

    }
    return count;
}

fn main() {
    let s = "hello world";
    println!("{}", count_vowels(s));
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("hello world"),3);
}