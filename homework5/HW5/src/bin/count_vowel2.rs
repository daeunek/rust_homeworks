// Recursion
fn count_vowels_r(s: &str) -> usize {
    let vowels: Vec<char> = "aeiouAEIOU".chars().collect();

    if s.is_empty() {
        return 0;
    } else {
        let c = s.chars().next().unwrap();
        let rem_c = &s[c.len_utf8()..];
        let counter = count_vowels_r(rem_c);
        if vowels.contains(&c) {
            return 1 + counter;
        } else {
            return counter;
        }
    }
}

fn main() {
    let s = "hello world";
    println!("{}", count_vowels_r(s));
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_r(""), 0);
    assert_eq!(count_vowels_r("abEcd"), 2);
    assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels_r("hello world"),3);
}
