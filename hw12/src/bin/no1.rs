use std::f64::consts::PI;

enum Shape {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
    Triangle(i32, i32, i32, i32, i32, i32)  //1.1
}

impl Shape {
    fn rep_string(&self) -> String {
        match self{
            Shape::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
            Shape::Rectangle(x,y,w,h) => format!("<Rectangle: {}, {}, {}, {}>", x,y, w,h),
            Shape::Triangle(x1,y1,x2,y2,x3,y3) => format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1,y1,x2,y2,x3,y3), //1.1
        }
    }

    fn area(&self) -> f64 {
        match self{
            Shape::Circle(_,_, r) => PI *( r * r) as f64,
            Shape::Rectangle(_,_,w,h) => (w * h) as f64,
            Shape::Triangle(x1,y1,x2,y2,x3,y3) => 0.5 * ((((x1-x3)*(y2-y1)) - ((x1-x2)*(y3-y1))) as f64),  //1.1
        }
    }
}
fn main() {
    let circle1 = Shape::Circle(0,0,1);
    println!("{:?}",circle1.rep_string());
    println!("{:?}",circle1.area());

    //1.1
    let tri1 = Shape::Triangle(-2, 1, 3,2,1,5);
    println!("{:?}",tri1.rep_string());
    println!("{:?}",tri1.area());
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
    Shape::Triangle(-2,1,3,2,1,5)
];

const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: -2, 1, 3, 2, 1, 5>, area: 8.50", 
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list
        .iter()
        .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}
