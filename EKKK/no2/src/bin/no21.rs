//2.1
fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for _i in img1{
        let a = format!("{}", _i);
        result.push(a);
    }
    for _j in img2{
        let b = format!("{}", _j);
        result.push(b);
    }
    result
}

//2.2
// fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
//     let mut result = Vec::new();
//     let maxlen1 = img1.iter().map(|v| v.len()).max().unwrap_or(0);
//     let maxlen2 = img2.iter().map(|c| c.len()).max().unwrap_or(0);

//     if img1.len() > img2.len(){
        
//     }
// }
fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut hcat = Vec::new();
    let max_img1 = img1.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_img2 = img2.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_length = max_img1.max(max_img2);
    // println!("{}", max_length);
    for (_i, _j) in img1.iter().zip(img2.iter()) {
        // println!("{}{}", _i, _j);
        let repeated = " ".repeat(max_length-_i.len());
        let line = format!("{}{}{}", _i, repeated, _j);
        hcat.push(line);
    }    
    if img1.len() > img2.len() {
        // println!("{}", img1[img1.len()-1]);
        let line = format!("{}", img1[img1.len()-1]);
        hcat.push(line);
    }
    else if img1.len() < img2.len() {
        // println!("{}{}", " ".repeat(img1[img1.len()-1].len()), img2[img2.len() -1]);
        let repeated = " ".repeat(max_length);
        // println!("{}", max_length);
        let line = format!("{}{}", repeated, img2[img2.len()-1]);
        hcat.push(line);
    }
    return hcat;
}

fn main() {
    let img1 = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());
    let img2 = [
        "<--",
        "#####",
        "<==",
        "*****"
        ].map(|v| v.to_string());
    
    println!("{:?}", vcat(&img1, &img2))
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data1 = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());

    let data = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());

    // assert_eq!(vcat(&emp, &data), data);
    // assert_eq!(vcat(&data, &emp), data);
    // assert_eq!(vcat(&data, &data), [
    //                                 "<--",
    //                                 "#####",
    //                                 "<==",
    //                                 "<--",
    //                                 "#####",
    //                                 "<=="
    //                             ]);
    assert_eq!(hcat(&data, &data[..2]), [
                                    "<--  <--",
                                    "##########",
                                    "<=="]);

    assert_eq!(hcat(&data[..2], &data), [
                                    "<--  <--",
                                    "##########",
                                    "     <=="]);
}