//! main.rs
//! This program plots all property coordinates to visualize if they form the shape of Connecticut

use plotters::prelude::*;
use csv::ReaderBuilder;

fn main() {
    // Open CSV file (expected to have lon/lat in columns 14 and 15)
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("cords_split_only.csv")
        .expect("Cannot open file");

    // Collect all valid coordinate pairs
    let points: Vec<(f64, f64)> = reader.records()
        .filter_map(|result| result.ok())
        .filter_map(|record| {
            let lon = record.get(14)?.parse::<f64>().ok()?;
            let lat = record.get(15)?.parse::<f64>().ok()?;
            Some((lon, lat))
        })
        .collect();

    println!("Loaded {} points", points.len());

    // Set up drawing canvas
    let root = BitMapBackend::new("ct_shape.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Define map bounds (Connecticut's lat/lon range)
    let (min_lon, max_lon) = (-73.8, -71.7);
    let (min_lat, max_lat) = (40.9, 42.1);

    // Build chart with bounds and no mesh
    let mut chart = ChartBuilder::on(&root)
        .caption("CT Property Dot Plot", ("sans-serif", 30))
        .margin(10)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)
        .unwrap();

    chart.configure_mesh().disable_mesh().draw().unwrap();

    // Plot every point as a 1px black dot
    chart.draw_series(
        points.iter().map(|(lon, lat)| Circle::new((*lon, *lat), 1, BLACK.filled()))
    ).unwrap();

    println!("Plot saved to ct_shape.png");
}
