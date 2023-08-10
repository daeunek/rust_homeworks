//Pb 3.2
//arrow pattern2
//
use std::env;

fn make_arrow2(star: i32) -> String {
    let mut arrows = String::new();

    for i in 1..(star * 2) {
        if i <= star {
            for _j in 0..(star - i) {
                arrows.push(' ');
            }
            for _k in 0..i {
                arrows.push('*');
            }
        } else {
            for _j in 0..(i - star) {
                arrows.push(' ');
            }
            for _k in i..(star * 2) {
                arrows.push('*');
            }
        }
        arrows.push('\n');
    }
    
    arrows
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = String::new();
    let size_arg = args.get(1).unwrap_or(&binding);
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    let arrow = make_arrow2(size);
    print!("{}", arrow);
}

#[test]
fn test2_with_5_stars() {
    let star = 5;
    let expected =  "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n";
    let arrow = make_arrow2(star);

    assert_eq!(expected, arrow);
}

#[test]
fn test2_with_0_stars() {
    let star = 0;
    let expected = "";
    let arrow = make_arrow2(star);

    assert_eq!(expected, arrow);
}
