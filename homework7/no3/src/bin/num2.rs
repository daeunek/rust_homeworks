fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let start_arg = if args.len() < 2 {""} else {&args[1]};
    let end_arg = if args.len() < 3 {""} else {&args[2]};
    let step_arg = if args.len() < 4 {""} else {&args[3]};

    let mut start: i32 = start_arg.parse().unwrap_or(0);
    let mut end: i32 = end_arg.parse().unwrap_or(0);
    let mut step: usize = step_arg.parse().unwrap_or(0);

    if step == 0 {
        start = 1;
        end = 1;
        step = 1;
    }

    let mut html: String = "".to_string();
    // table start
    html.push_str("<table>\n");
    
    // table head
    html.push_str("<tr>\n");
    html.push_str("<th style=\"text-align:right\">x</th>\n");
    html.push_str("<th style=\"text-align:right\">x<sup>2</sup></th>\n");
    html.push_str("<th style=\"text-align:right\">x<sup>3</sup></th>\n");
    html.push_str("</tr>\n");

    if start <= end {
        for i in (start..=end).step_by(step){
            html.push_str(&format!("
            <tr>\n
            <td style=\"text-align:right\">{}</td>\n
            <td style=\"text-align:right\">{}</td>\n
            <td style=\"text-align:right\">{}</td>\n
            </tr>\n
            ",i, i.pow(2), i.pow(3)))
        }
    }
    else {
        for i in (end..=start).rev().step_by(step){
            html.push_str(&format!("
            <tr>\n
            <td style=\"text-align:right\">{}</td>\n
            <td style=\"text-align:right\">{}</td>\n
            <td style=\"text-align:right\">{}</td>\n
            </tr>\n
            ",i, i.pow(2), i.pow(3)))
        }
    }

    // table end
    html.push_str("</table>\n");

    println!("{}", html);
}
