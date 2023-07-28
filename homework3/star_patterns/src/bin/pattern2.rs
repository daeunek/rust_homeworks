// Problem 3.2
// Star Pattern 2

// It will print the stars up to "stars". eg stars - 5.--> it will be 5 stars.
fn print_stars(stars: i32) {
    for _ in 0..stars {
        print!("*");
    }
    println!();
}

fn star_pattern_2(size: i32) {
    for i in 1..=size {
        for _ in i..size {
            print!(" ");
        }
        print_stars(i);
    }

    for i in (1..size).rev() {
        for _ in i..size {
            print!(" ");
        }
        print_stars(i);
    }
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let line_arg = if args.len() < 2 {""} else {&args[1]};
    let line: i32 = line_arg.parse().unwrap_or(0);

    star_pattern_2(line);
}
