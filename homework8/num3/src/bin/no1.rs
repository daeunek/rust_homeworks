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

fn to_polar(pt_list: &[Point]) -> Vec<PolarPoint> {
    let mut polar_pt_list = Vec::new();
    for point in pt_list {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let t = point.y.atan2(point.x); 
        let polar = PolarPoint::new(r, t);
        polar_pt_list.push(polar);
    }
    polar_pt_list
}


fn main() {
    let mut html: String = String::new();

    // table start
    html.push_str("<table>");

    // table head
    html.push_str("<tr>");
    html.push_str("<th style=\"text-align:center\">Radian</th>");
    html.push_str("<th style=\"text-align:center\">Theta</th>");
    html.push_str("</tr>");

    let p1 = Point::new(2.0, 1.0);
    let p2 = Point::new(2.0, 2.0);
    let p3 = Point::new(3.0, 4.0);

    let pt_list = vec![p1, p2, p3];
    let polar_list = to_polar(&pt_list);

    for _pt in polar_list {
        html.push_str(&format!(
            "<tr>
            <td style=\"text-align:right\">{:.2}</td>
            <td style=\"text-align:right\">{:.2}</td>
            </tr>",
            _pt.r, _pt.t
        ))
    }

    // table end
    html.push_str("</table>");

    // Save HTML content to a file
    let mut file = File::create("output.html").expect("Unable to create HTML file");
    file.write_all(html.as_bytes()).expect("Unable to write to HTML file");
    
    println!("HTML content saved to output.html");
}
