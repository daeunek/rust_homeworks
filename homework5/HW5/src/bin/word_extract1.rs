// ordinary loop
fn extract_quoted_words(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    
    for word in s.split_whitespace() {
        let open_star = word.chars().next().unwrap_or('_');
        let close_star = word.chars().rev().next().unwrap_or('_');

        if open_star == '*' && close_star == '*' {
            let mut _w = word.to_string();
            _w.remove(0);
            _w.pop();
            result.push(_w);
        }
    }
    
    return result
}

fn main() {
    let s = "C ** *C++* *Java *Python* Rust*";
    println!("{:?}", extract_quoted_words(s));
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
    extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
    ["", "C++", "Python"] // "**", "*C++*", "*Python*"
    );
    assert_eq!(
        extract_quoted_words("Hello *world* !"),
        ["world"]
    );
}
