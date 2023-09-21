use std::fs::File;
use std::io::Write;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

struct PolarPoint {
    r: f32,
    t: f32,
}

impl PolarPoint {
    fn new(r: f32, t: f32) -> PolarPoint {
        PolarPoint { r, t }
    }
}

fn to_cartesian(pt_list: &[PolarPoint]) -> Vec<Point> {
    let mut cartesian_pt_list = Vec::new();
    for point in pt_list {
        let x = point.r * point.t.cos();
        let y = point.r * point.t.sin();
        let cartesian = Point::new(x, y);
        cartesian_pt_list.push(cartesian);
    }
    cartesian_pt_list
}


fn main() {
    let mut html: String = String::new();

    // table start
    html.push_str("<table>");

    // table head
    html.push_str("<tr>");
    html.push_str("<th style=\"text-align:center\">X</th>");
    html.push_str("<th style=\"text-align:center\">Y</th>");
    html.push_str("</tr>");

    let polar1 = PolarPoint::new(1.0, 0.0);
    let polar2 = PolarPoint::new(2.0, 1.047);
    let polar3 = PolarPoint::new(3.0, 1.571);

    let polar_pt_list = vec![polar1, polar2, polar3];
    let cartesian_list = to_cartesian(&polar_pt_list);

    for _pt in cartesian_list {
        html.push_str(&format!(
            "<tr>
            <td style=\"text-align:right\">{:.2}</td>
            <td style=\"text-align:right\">{:.2}</td>
            </tr>",
            _pt.x, _pt.y
        ))
    }

    // table end
    html.push_str("</table>");

    // Save HTML content to an HTML file
    let mut file = File::create("output2.html").expect("Unable to create HTML file");
    file.write_all(html.as_bytes()).expect("Unable to write to HTML file");

    println!("HTML content saved to output2.html");
}
