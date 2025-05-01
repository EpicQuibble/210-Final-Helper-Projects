use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use csv::{ReaderBuilder, WriterBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "Real_Estate_Sales_2001-2022_GL.csv";
    let output_path = "cords_split_only.csv";

    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(input_file));

    let headers = rdr.headers()?.clone();
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(BufWriter::new(output_file));

    // Write new header: old headers + Longitude, Latitude
    let mut new_headers = headers.clone();
    new_headers.push_field("Longitude");
    new_headers.push_field("Latitude");
    wtr.write_record(&new_headers)?;

    for result in rdr.records() {
        let record = result?;
        let mut new_record = record.clone();

        // Location is assumed to be the 14th column (index 13)
        if let Some(location) = record.get(13) {
            if location.trim().is_empty() {
                continue; // skip if empty
            }
            if location.starts_with("POINT") {
                let coords = location.trim_start_matches("POINT (").trim_end_matches(")").split_whitespace().collect::<Vec<_>>();
                if coords.len() == 2 {
                    let lon = coords[0];
                    let lat = coords[1];
                    new_record.push_field(lon);
                    new_record.push_field(lat);
                    wtr.write_record(&new_record)?;
                }
            }
        }
    }

    wtr.flush()?;
    println!("Finished filtering and splitting coordinates!");
    Ok(())
}
