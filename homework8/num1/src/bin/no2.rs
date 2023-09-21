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

    let polar_points = vec![
        PolarPoint::new(1.0, 0.0),  
        PolarPoint::new(2.0, 1.047),  
        PolarPoint::new(3.0, 1.571),  
    ];

    let cartesian_points = to_cartesian(&polar_points);

    for (i, cartesian_point) in cartesian_points.iter().enumerate() {
        println!(
            "Point {}: x = {:.2}, y = {:.2}",
            i + 1,
            cartesian_point.x,
            cartesian_point.y
        );
    }
}
