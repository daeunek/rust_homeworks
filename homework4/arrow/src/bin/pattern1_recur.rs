//Pb3.3
//Pattern1 Recursion
//cargo run --bin pattern1_recur

fn make_arrow(size: i32, row_idx: &mut i32, arrow: &mut String) {
    *row_idx += 1;
    
    if *row_idx < size * 2 {
        let mut star_idx: i32 = 0;
        
        if *row_idx <= size {
            star_generator(*row_idx, &mut star_idx, arrow);
        } else {
            let rev_idx = (size * 2) - *row_idx;
            star_generator(rev_idx, &mut star_idx, arrow);
        }
        
        arrow.push_str("\n");
        make_arrow(size, row_idx, arrow);
    }
}

fn star_generator(row_idx: i32, star_idx: &mut i32, arrow: &mut String) {
    *star_idx += 1;
    
    if *star_idx <= row_idx {
        arrow.push('*');
        star_generator(row_idx, star_idx, arrow);
    }
}

fn main() {
    let arrow_size: i32 = 3;
    let mut arrow = String::new();
    let mut row_idx = 0;
    
    make_arrow(arrow_size, &mut row_idx, &mut arrow);
    print!("{}", arrow);
}

#[test]
fn test_make_arrow_recur() {
    let star = 3;
    let mut arrow = String::new();
    let mut index = 0;

    let expected = "*\n**\n***\n**\n*\n";
    make_arrow(star, &mut index, &mut arrow);

    assert_eq!(expected, arrow);
}

#[test]
fn test_star_generator_with_zero() {
    let star_num = 0;
    let mut arrow = String::new();
    let mut index = 0;
    let expected = "";

    star_generator(star_num, &mut index, &mut arrow);

    assert_eq!(expected, arrow);
}

#[test]
fn test_star_generator_with_two() {
    let star_num = 2;
    let mut arrow = String::new();
    let mut index = 0;
    let expected = "**";

    star_generator(star_num, &mut index, &mut arrow);

    assert_eq!(expected, arrow);
}
