//! main.rs
//! Plots all points from the dataset, highlighting those within Connecticut's bounding box.

use csv::ReaderBuilder;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::Circle;
use plotters::style::{BLACK, WHITE, RGBColor, ShapeStyle};
use plotters_bitmap::BitMapBackend;

fn main() {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("cords_split_only.csv")
        .expect("Cannot open file");

    // Parse all valid lon/lat pairs
    let points: Vec<(f64, f64)> = reader.records()
        .filter_map(|result| result.ok())
        .filter_map(|record| {
            let lon = record.get(14)?.parse::<f64>().ok()?;
            let lat = record.get(15)?.parse::<f64>().ok()?;
            Some((lon, lat))
        })
        .collect();

    println!("Loaded {} points", points.len());

    // Drawing area
    let root = BitMapBackend::new("ct_inclusive_plot.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Bounding box for all data, slightly wider than CT
    let (min_lon, max_lon) = (-74.5, -71.0); // widen view
    let (min_lat, max_lat) = (40.0, 43.0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Printing all our data", ("sans-serif", 30))
        .margin(10)
        .build_cartesian_2d(min_lon..max_lon, min_lat..max_lat)
        .unwrap();

    chart.configure_mesh().disable_mesh().draw().unwrap();

    // Define CT box
    let ct_min_lon = -73.8;
    let ct_max_lon = -71.7;
    let ct_min_lat = 40.9;
    let ct_max_lat = 42.1;

    chart.draw_series(
        points.iter().map(|(lon, lat)| {
            let is_inside = *lon >= ct_min_lon && *lon <= ct_max_lon &&
                            *lat >= ct_min_lat && *lat <= ct_max_lat;
    
            let color = if is_inside {
                BLACK
            } else {
                RGBColor(200, 200, 200) // Outside CT = gray
            };
    
            let radius = if is_inside { 1 } else { 4 }; // Larger dots for outliers
            Circle::new((*lon, *lat), radius, ShapeStyle::from(&color).filled())
        })
    ).unwrap();
/// I added junk data points to test if our data was inside or outside the plot and this is how i tested it 

    println!("Plot saved to ct_inclusive_plot.png");
}
