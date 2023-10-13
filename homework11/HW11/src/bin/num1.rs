fn main() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";

    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs

    for paragraph in &doc1 {
        println!("{:?}", paragraph);
    }
    println!();
    for paragraph in &doc2 {
        println!("{:?}", paragraph);
    }
    println!();
    for paragraph in &doc3 {
        println!("{:?}", paragraph);
    }

    //1.2
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    println!("{:?}", rnk_docs);

}

// fn make_document(text: &str) -> Vec<String> {
//     let paragraphs: Vec<&str> = text.split("\n\n").collect(); // \n\n as the delimeter for splitting text into paras
//     let format_paras:Vec<String> = paragraphs
//         .iter().map(|paras| paras.trim().to_string())
//         .collect(); //.collect() will convert each &str to String
//     format_paras
// }

fn make_document(text: &str) -> Vec<String> {
    let mut format_paras = Vec::new();
    for i in text.split("\n\n"){
        format_paras.push(i.to_string())
    }
    format_paras
}

fn rank_documents(docs: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut ranked_docs = docs.to_vec(); //clone the entire vector
    ranked_docs.sort_by(|b,a| a.len().cmp(&b.len()));
    ranked_docs
}

#[test]
fn test_no1() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";

    let empty_doc = Vec::new();
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![empty_doc.clone(), doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1, empty_doc]);
}


// // Sorting strings by length Examples
// fn main() {
//     let mut strings = vec!["applerukjkj", "ban", "cherry"];
//     strings.sort_by(|a, b|a.len().cmp(&b.len()));
//     strings.sort_by(|a, b|a.len().cmp(&b.len()));
//     print!("{:?}", strings)
// }

