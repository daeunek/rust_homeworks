//vount vowel version 2
fn count_vowels_v2(s: &str) -> Vec<(String, usize)> {
    let vowels: Vec<char> = "aeiouAEIOU".chars().collect();
    let mut result = Vec::new();
    
    for word in s.split_whitespace() {
        let mut counter = 0;
    
        for c in word.chars() {
            if vowels.contains(&c) {
                counter += 1;
            }
            else {
                continue;
            }
        }
        
        //tuple
        let t =(word.to_string(), counter);
        result.push(t)
    }
    
    return result;
}


fn main() {
    let s = "Hello World!";
    println!("{:?}", count_vowels_v2(s));
}

#[test]
fn test_vowels_countv2() {
assert_eq!(count_vowels_v2(""), []);

assert_eq!(
count_vowels_v2("ab12Exey5 7x8U3y5z"),
[
("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
("7x8U3y5z".to_string(), 1) // 'U'
]
);

assert_eq!(count_vowels_v2("Hello World!"),
[("Hello".to_string(),2),
 ("World!".to_string(),1)
]);
}
