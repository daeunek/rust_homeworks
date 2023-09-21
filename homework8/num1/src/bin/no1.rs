struct Point {
    x: f32,
    y: f32,
}

struct PolarPoint {
    r: f32,
    t: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
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
    let cartesian_points = vec![
        Point::new(2.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(3.0, 4.0),
    ];

    let polar_points = to_polar(&cartesian_points);

    for (i, polar_point) in polar_points.iter().enumerate() {
        println!(
            "Point {}: r = {:.2}, θ = {:.2} radians",
            i + 1,
            polar_point.r,
            polar_point.t
        );
    }
}
