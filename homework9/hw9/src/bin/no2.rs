use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    circles: Vec<Circle>,
}

fn calculate_average_areas(layers: Vec<Layer>) -> Vec<(String, f32)> {
    let mut result = Vec::new();
    const PI: f32 = 3.142;
    for layer in layers {
        let mut sum = 0.;
        for circle in &layer.circles {
            let area = PI * circle.radius * circle.radius;
            sum += area;
        }

        let avg_area = sum / layer.circles.len() as f32;
        result.push((layer.name, avg_area));
    }
    return result;
}

fn generate_layer(name: String, color: String, rng: &mut impl Rng) -> Layer {
    let mut circles = Vec::new();
    let num = rng.gen_range(20..=50);
    for _ in 0..num {
        let x = rng.gen_range(-100. ..= 100.);
        let y = rng.gen_range(-100. ..= 100.);
        let radius = rng.gen_range(-10. ..= 20.);
        circles.push(Circle { x, y, radius });
    }
    Layer {
        name,
        color,
        circles,
    }
}

fn generate_object_layer_list(rng: &mut impl Rng, n: usize) -> Vec<Layer> {
    let mut layer_list = Vec::new();
    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let r = rng.gen::<u8>();
        let g = rng.gen::<u8>();
        let b = rng.gen::<u8>();
        let a = rng.gen::<u8>();
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
        layer_list.push(generate_layer(name, color, rng));
    }
    layer_list
}

fn save_layer_data(writer: impl Write, layers: &[Layer]) {
    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(writer);
    csv_writer.write_record(["Name", "Color", "Circle (x,y,radius)"]).unwrap();

    for layer in layers {
        let mut circle_infos = String::new();
        for circle in &layer.circles {
            circle_infos.push_str(format!("({}, {}, {})", circle.x, circle.y, circle.radius).as_str());
            circle_infos.push_str(",");
        }
        csv_writer.serialize((
            &layer.name,
            &layer.color,
            &circle_infos,
        )).unwrap();
        csv_writer.flush().unwrap();
    }
}

fn load_layer_data(reader: impl Read) -> Vec<Layer> {
    let mut layer_list = Vec::new();
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(reader);

    for result in csv_reader.records() {
        let record = result.unwrap();
        if record.len() < 3 {
            continue;
        }

        let name = record[0].to_string();
        let color = record[1].to_string();
        let circles_data = record[2].trim_matches(|c| c == '(' || c == ')');
        let mut circles = Vec::new();

        for coords_str in circles_data.split("),(") {
            let coords: Vec<&str> = coords_str.split(',').collect();
            if coords.len() == 3 {
                let x = coords[0].trim().parse().unwrap_or(0.0);
                let y = coords[1].trim().parse().unwrap_or(0.0);
                let radius = coords[2].trim().parse().unwrap_or(0.0);
                circles.push(Circle { x, y, radius });
            }
        }

        let layer = Layer { name, color, circles };
        layer_list.push(layer);
    }
    layer_list
}

fn save_circle_average_areas(writer: impl Write, area_list: &[(String, f32)]) {
    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_writer(writer);

    csv_writer.write_record(["Name", "Area Average"]).unwrap();

    for (name, area) in area_list {
        csv_writer.serialize((
            name,
            area,
        )).unwrap();
        csv_writer.flush().unwrap();
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let n = 3;

    // 2.1
    let layers = generate_object_layer_list(&mut rng, n);
    let output_layer_file = File::create("layers.csv").expect("File cannot be created.");
    save_layer_data(output_layer_file, &layers);

    // 2.2
    let input_file = File::open("layers.csv").expect("File cannot be opened.");
    let layers = load_layer_data(input_file);
    let result = calculate_average_areas(layers);
    let output_area_file = File::create("circle_area.csv").expect("File cannot be created.");
    save_circle_average_areas(output_area_file, &result);
}
