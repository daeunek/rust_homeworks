fn fahr2cel(fahr: i32) -> f32 {
    (5.0 / 9.0) * (fahr as f32 - 32.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let start: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let end: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let step: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

    if step == 0 {
        return;
    }

    // Start building the HTML string
    let mut html = String::new();

    // Table and style
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("border: 1px solid #000000;\n");
    html.push_str("border-collapse: collapse;\n");
    html.push_str("}\n");
    html.push_str("</style>\n\n");
    html.push_str("<table>\n");
    html.push_str("<tr>\n");
    html.push_str("<th>Col 1</th>\n");
    html.push_str("<th>Col 2</th>\n");
    html.push_str("</tr>\n");

    // Loop for conversion and table rows
    for fahr in (start..=end).step_by(step) {
        let cel = fahr2cel(fahr);
        html.push_str("<tr>\n");
        html.push_str(&format!("<td>{}</td>\n", fahr));
        html.push_str(&format!("<td>{:.1}</td>\n", cel));
        html.push_str("</tr>\n");
    }

    // Close the table
    html.push_str("</table>");

    // Print the HTML
    println!("{}", html);
}
