//1.1
fn vflip(img: &[String]) -> Vec<String> {
    let mut vflipped = Vec::new();
    for line in img.iter().rev(){
        // line output is &string, so we have to change to string
        //let a = format!("{}", _i); //it is like a change to string
        vflipped.push(line.to_string());
    }
    vflipped
}

//1.2
fn hflip(img: &[String]) -> Vec<String> {
    let mut hflipped = Vec::new();
    //map method needs to provide type
    let max_line_len = img.iter().map(|v: &String| v.len()).max().unwrap_or(0);

    for line in img.iter(){
        let reversed_line: String = line.chars().rev().collect();
        let result = format!("{:>width$}", reversed_line, width = max_line_len);
        hflipped.push(result);
    }
    hflipped
}

fn main(){
    //using map to transform each element of the array
    let data = ["<--", "#####", "<=="].map(|v|v.to_string());
    println!("vertical flip: {:?}", vflip(&data));
    println!("Horizontal flip: {:?}", hflip(&data));
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);

    let data = [
        "<--",
        "######",
        "<=="
        ].map(|v| v.to_string());

    assert_eq!(vflip(&data), [
        "<==",
        "######",
        "<--"]);

    assert_eq!(hflip(&data), [
        "   --<",
        "######",
        "   ==<"]);
}
