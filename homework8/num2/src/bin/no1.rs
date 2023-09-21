use std::fs::File;
use std::io::{Read, Write};

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

fn load_cartesian_file(reader: impl Read) -> Vec<Point> {
    let mut pt_list = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    for r in rdr.records() {
        let record = r.unwrap();
        if record.len() < 2 {
            continue;
        }
        let _x: f32 = record[0].parse().unwrap_or(0.0);
        let _y: f32 = record[1].parse().unwrap_or(0.0);
        let _point = Point::new(_x, _y);
        pt_list.push(_point);
    }
    pt_list
}

fn save_polar_file(writer: impl Write, pt_list: &[PolarPoint]) {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_writer(writer);

    for pt in pt_list {
        wtr.serialize((pt.r, pt.t)).unwrap();
    }

    wtr.flush().unwrap();
}

fn main() {
    let cartesian_input_file = File::open("./cartesian_input.csv").expect("Unable to open the file");
    let cartesian_list = load_cartesian_file(cartesian_input_file);
    let polar_list = to_polar(&cartesian_list);
    let polar_output_file = File::create("./polar_output.csv").expect("Unable to create the file");
    save_polar_file(polar_output_file, &polar_list);
}
