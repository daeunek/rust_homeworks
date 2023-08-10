//Pb 3.3
//Pattern2 using recursion

fn make_arrow2(size: i32, row_idx: &mut i32, arrow: &mut String ) {
    *row_idx += 1;
    
    if *row_idx < size * 2 {
        let mut star_idx: i32 = 0;
        let mut space_idx: i32 = *row_idx;
        
        if *row_idx <= size {
            space_generator(size, &mut space_idx, arrow);
            star_generator(*row_idx, &mut star_idx, arrow);
        } else {
            let mut rev_idx = size;
            let mut star_rev_idx = *row_idx;
            
            space_generator(*row_idx, &mut rev_idx, arrow);
            star_generator(size * 2, &mut star_rev_idx, arrow);
        }
        
        arrow.push_str("\n");
        make_arrow2(size, row_idx, arrow);
    }
}

fn space_generator(size: i32, space_idx: &mut i32, arrow: &mut String) {
    *space_idx += 1;
    
    if *space_idx <= size {
        arrow.push(' ');
        space_generator(size, space_idx, arrow);
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
    
    make_arrow2(arrow_size, &mut row_idx, &mut arrow);
    print!("{}", arrow);
}

#[test]
fn test_make_arrow2_recur() {
    let star_num = 3;
    let mut index = 0;
    let mut arrow = String::new();
    let expected = "  *\n **\n***\n **\n  *\n";
    make_arrow2(star_num, &mut index, &mut arrow);
    assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_star_generator_with_zero() {
    let star_num = 0;
    let mut arrow = String::new();
    let mut index = 0;
    let expected = "";
    star_generator(star_num, &mut index, &mut arrow);
    assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_star_generator_with_two() {
    let star_num = 2;
    let mut arrow = String::new();
    let mut index = 0;
    let expected = "**";
    star_generator(star_num, &mut index, &mut arrow);
    assert_eq!(expected, arrow);
}

#[test]
fn test_arrow2_space_generator() {
    let space_num = 1;
    let mut arrow = String::new();
    let mut index = 0;
    let expected = " ";
    space_generator(space_num, &mut index, &mut arrow);
    assert_eq!(expected, arrow);
}
