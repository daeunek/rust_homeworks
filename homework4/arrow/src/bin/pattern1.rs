fn make_arrow1(size: i32) -> String {
    let mut arrow = String::new();

    for i in 1..=size {
        arrow.push_str(&"*".repeat(i as usize));
        arrow.push('\n');
    }

    for i in (1..size).rev() {
        arrow.push_str(&"*".repeat(i as usize));
        arrow.push('\n');
    }

    arrow
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let binding = String::new();
    let size_arg = args.get(1).unwrap_or(&binding);
    let size: i32 = size_arg.parse().unwrap_or(0);

    let arrow1 = make_arrow1(size);
    print!("{}", arrow1);
}

// Test 
#[test]
fn test_with_5_stars() {
    let star = 5;
    let expected = "*\n**\n***\n****\n*****\n****\n***\n**\n*\n";
    let arrow = make_arrow1(star);

    assert_eq!(expected, arrow);
}

#[test]
fn test2_with_0_stars() {
    let star = 0;
    let expected = "";
    let arrow = make_arrow1(star);

    assert_eq!(expected, arrow);
}