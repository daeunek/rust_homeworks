//2.1
fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for _i in img1 {
        result.push(_i.to_string());
    }
    //the same TT
    for _j in img2.iter(){
        result.push(_j.to_string());
    }
    result
}

//2.2
fn hcat(img1: &[String], img2: &[String])-> Vec<String> {
    let mut result = Vec::new();
    let maxlen1 = img1.iter().map(|v| v.len()).max().unwrap_or(0);

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        // zip can be used for same length. so we have to consider for last element.
        let combine = format!("{:<width$}{}", line1, line2, width = maxlen1);
        result.push(combine);
    }

    if img1.len() > img2.len() {
        let combine = format!("{}", img1[img1.len()-1]);
        result.push(combine);
    }
    else if img1.len() < img2.len() {
        let space = " ";
        let combine = format!("{:<width$}{}",space, img2[img2.len()-1], width = maxlen1);
        result.push(combine);
    }
     
    result
} 

fn main() {
    let img1 = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());
    let img2 = [
        "<-",
        "#####",
        "<=="
        ].map(|c| c.to_string());
    
    println!("{:?}", vcat(&img1, &img2));
    println!("{:?}", hcat(&img1, &img2[..2]));
    println!("{:?}", hcat(&img1[..2], &img2));
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = [
        "<--",
        "#####",
        "<=="
        ].map(|v| v.to_string());

    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(vcat(&data, &data), [
                                    "<--",
                                    "#####",
                                    "<==",
                                    "<--",
                                    "#####",
                                    "<=="
                                ]);
    assert_eq!(hcat(&data, &data[..2]), [
                                    "<--  <--",
                                    "##########",
                                    "<=="]);

    assert_eq!(hcat(&data[..2], &data), [
                                    "<--  <--",
                                    "##########",
                                    "     <=="]);
}