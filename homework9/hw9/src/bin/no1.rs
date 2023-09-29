extern crate rand;
use rand::{Rng, thread_rng};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}

fn gen_obj_layer_list(rng: &mut impl Rng, n: usize) -> Vec<Layer> {
    let mut layers = Vec::new();

    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();
        let a = rng.gen::<u8>();
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);

        let num_circles = rng.gen_range(20..=50);
        let mut circles = Vec::new();

        for _ in 0..num_circles {
            let x = rng.gen_range(-100. ..= 100.);
            let y = rng.gen_range(-100. ..= 100.);
            let r = rng.gen_range(-10. ..= 20.);
            circles.push(Circle { x, y, r });
        }

        layers.push(Layer {
            name,
            color,
            objects: circles,
        });
    }

    layers
}

fn cal_average_area(layers: &Vec<Layer>) -> Vec<(String, f64)> {
    let mut result = Vec::new();
    const PI: f64 = 3.14159265359;

    for layer in layers {
        let total_area: f64 = layer.objects.iter().map(|circle| PI * circle.r * circle.r).sum();
        let avg_area = total_area / layer.objects.len() as f64;
        result.push((layer.name.clone(), avg_area));
    }

    result
}

fn main() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);
    let result = cal_average_area(&layers);

    println!("{:?}", layers);
    println!("{:?}", result);
}

#[test]
fn test_gen_obj_layer_list() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);

    assert_eq!(layers.len(), n);

    for layer in &layers {
        assert!(!layer.objects.is_empty());
    }
}

#[test]
fn test_cal_average_area() {
    let layers = vec![
        Layer {
            name: "Layer 1".to_string(),
            color: "#47C6E92B".to_string(),
            objects: vec![
                Circle {
                    x: -47.73065,
                    y: 32.04605,
                    r: 15.582638,
                },
                Circle {
                    x: 51.935318,
                    y: -24.768623,
                    r: -7.6575804,
                },
            ],
        },
    ];
    let result = cal_average_area(&layers);
    assert_eq!(vec![("Layer 1".to_string(),  473.5277754575741)], result);
}

    