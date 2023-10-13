use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone)]
struct Document {
    name: String,
    content: Vec<String>,
    para_count: usize,
    word_count: usize,
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

impl Document {
    fn make_document(file: &str, text: &str) -> Document {
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let format_paras: Vec<String> = paragraphs
            .iter()
            .map(|paras| paras.trim().to_string())
            .collect();
        Document {
            name: file.to_string(),
            content: format_paras.clone(),
            para_count: format_paras.len(),
            word_count: count_words(text),
        }
    }

    //1.1
    fn rank_by_para_count(unranked_docs: &[Document]) -> Vec<Document> {
        let mut ranked_docs = unranked_docs.to_vec();
        ranked_docs.sort_by(|b, a| a.para_count.cmp(&b.para_count));
        ranked_docs
    }

    fn generate_html(ranked: &[Document]) -> String {
        let mut html_table = String::new();
        html_table.push_str("<table>\n");
        html_table.push_str("<table border=\"1\" style=\"text-align: right;\">\n");
        html_table.push_str("<tr><th>File</th><th>Para_Count</th></tr>\n");
        for doc in ranked {
            html_table.push_str("<tr>");
            html_table.push_str(&format!("<td>{}</td>", doc.name));
            html_table.push_str(&format!("<td>{}</td>", doc.para_count));
            html_table.push_str("</tr>\n");
        }
        html_table.push_str("</table>\n");
        html_table
    }

    //1.2
    fn rank_by_word_count(unranked_docs: &[Document]) -> Vec<Document> {
        let mut ranked_docs = unranked_docs.to_vec();
        ranked_docs.sort_by(|b, a| a.word_count.cmp(&b.word_count));
        ranked_docs
    }

    fn generate_html_word(ranked: &[Document]) -> String {
        // Generate an HTML table
        let mut html_table = String::new();
        html_table.push_str("<table>\n");
        html_table.push_str("<table border=\"1\" style=\"text-align: right;\">\n");
        html_table.push_str("<tr><th>File</th><th>Word_Count</th></tr>\n");
        for doc in ranked {
            html_table.push_str("<tr>");
            html_table.push_str(&format!("<td>{}</td>", doc.name));
            html_table.push_str(&format!("<td>{}</td>", doc.word_count));
            html_table.push_str("</tr>\n");
        }
        html_table.push_str("</table>\n");
        html_table
    }
}

fn main() {
    let files = vec!["f1.txt", "f2.txt", "f3.txt"];
    let mut unranked_docs = Vec::new();
    for f in &files {
        let file = File::open(f).expect("Can't open the file!");
        let reader = BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line.unwrap());
            content.push_str("\n");
        }
        unranked_docs.push(Document::make_document(f, &content));
    }

    // No2.1
    let ranked_docs = Document::rank_by_para_count(&unranked_docs);
    let para_html_table = Document::generate_html(&ranked_docs);

    let mut para_output_file = File::create("paracount_ranked_docs.html").expect("Failed to create output file");
    write!(para_output_file, "{}", para_html_table).expect("Failed to write to output file");

    // No2.2
    let ranked_docs_word = Document::rank_by_word_count(&unranked_docs);
    let word_html_table = Document::generate_html_word(&ranked_docs_word);

    let mut word_output_file = File::create("wordcount_ranked_docs.html").expect("Failed to create output file");
    write!(word_output_file, "{}", word_html_table).expect("Failed to write to output file");
}
