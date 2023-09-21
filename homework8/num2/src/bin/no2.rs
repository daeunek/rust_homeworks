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

fn load_polar_file(reader: impl Read) -> Vec<PolarPoint> {
    let mut pt_list = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    for r in rdr.records() {
        let record = r.unwrap();
        if record.len() < 2 {
            continue;
        }
        let _r: f32 = record[0].parse().unwrap_or(0.0);
        let _t: f32 = record[1].parse().unwrap_or(0.0);
        let _point = PolarPoint::new(_r, _t);
        pt_list.push(_point);
    }
    pt_list
}

fn save_cartesian_file(writer: impl Write, pt_list: &[Point]) {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_writer(writer);

    for pt in pt_list {
        wtr.serialize((pt.x, pt.y)).unwrap();
    }

    wtr.flush().unwrap();
}

fn main() {
    let polar_input_file = File::open("./polar_input.csv").expect("Unable to open the file");
    let polar_list = load_polar_file(polar_input_file);
    let cartesian_list = to_cartesian(&polar_list);
    let cartesian_output_file = File::create("./cartesian_output.csv").expect("Unable to create the file");
    save_cartesian_file(cartesian_output_file, &cartesian_list);
}
