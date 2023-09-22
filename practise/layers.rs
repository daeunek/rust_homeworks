use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Writer;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

fn gen_layer<R: Rng>(name: String, color: String, rng: &mut R) -> Layer {
    let num_points = rng.gen_range(20..=50);
    let mut points = Vec::new();
    for _ in 0..num_points {
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        points.push(Point { x, y });
    }

    Layer {
        name,
        color,
        points,
    }
}

fn gen_layer_list<R: Rng>(rng: &mut R, n: u32) -> Vec<Layer> {
    let mut layers = Vec::new();

    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let color = format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255)
        );
        let layer = gen_layer(name, color, rng);
        layers.push(layer);
    }
    layers
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: program_name <n> <output_filename>");
        std::process::exit(1);
    }

    let n = args[1].parse::<u32>().unwrap_or(0);
    let output_filename = &args[2];

    let mut rng = rand::thread_rng();
    let layers = gen_layer_list(&mut rng, n);

    let mut file = File::create(output_filename)?;
    let mut csv_writer = Writer::from_writer(file);

    csv_writer.write_record(&["name", "color", "points"])?;

    for layer in &layers {
        let points_str: Vec<String> = layer.points.iter()
            .map(|point| format!("{},{}", point.x, point.y))
            .collect();

        csv_writer.write_record(&[&layer.name, &layer.color, &points_str.join(";")])?;
    }

    println!("Generated {} layers and saved to '{}'", n, output_filename);

    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_generate_and_parse_csv() {
        let n = 3;
        let output_filename = "test_output.csv";

        // Capture the current arguments
        let original_args = std::env::args().collect::<Vec<String>>();

        // Set custom arguments for testing
        std::env::set_var("RUST_BACKTRACE", "1"); // Enable backtrace for better error messages
        std::env::set_args(vec![
            String::from("program_name"), // Placeholder for the program name
            n.to_string(),
            output_filename.to_string(),
        ]);

        let result = main();

        // Restore the original arguments
        std::env::set_args(original_args);

        assert!(result.is_ok(), "Program execution failed");

        // Check the existence of the output file
        assert!(
            std::path::Path::new(output_filename).exists(),
            "Output file not found"
        );

        // Optionally, you can check the content of the generated file as well
        let file_content = std::fs::read_to_string(output_filename)
            .expect("Failed to read output file");
        assert!(!file_content.is_empty(), "Output file is empty");

        // Clean up: delete the generated output file
        std::fs::remove_file(output_filename).expect("Failed to remove output file");
    }
}
