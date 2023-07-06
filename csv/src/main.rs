extern crate csv;

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file with journal IDs
    let journal_ids_path = Path::new("input/journal_ids.csv");
    let journal_ids_file = File::open(journal_ids_path)?;
    let mut journal_ids_reader = csv::Reader::from_reader(journal_ids_file);

    // Load journal IDs into a vector
    let mut journal_ids: Vec<u32> = Vec::new();
    for result in journal_ids_reader.records() {
        let record = result?;
        let journal_id: u32 = match record.get(0).map(|s| s.parse()) {
            Some(Ok(parsed_value)) => parsed_value,
            _ => continue, // Skip empty or invalid values
        };
        journal_ids.push(journal_id);
    }

    // Read the input CSV file
    let input_path = Path::new("input/input.csv");
    let input_file = File::open(input_path)?;
    let mut input_reader = csv::Reader::from_reader(input_file);

    // Prepare output CSV writer
    let output_path = Path::new("output/output.csv");
    let output_file = File::create(output_path)?;
    let mut output_writer = csv::Writer::from_writer(output_file);

    // Iterate over each row in the input CSV file
    for result in input_reader.records() {
        let record = result?;
        let journal_id: u32 = match record.get(0).map(|s| s.parse()) {
            Some(Ok(parsed_value)) => parsed_value,
            _ => continue, // Skip empty or invalid values
        };
        let doi = record.get(1).map(|s| s.to_string()).unwrap_or_else(|| String::new());
        let url = record.get(2).map(|s| s.to_string()).unwrap_or_else(|| String::new());
        let doi_resolves_to: String = record.get(3).map(|s| s.to_string()).unwrap_or_else(|| String::new());
        // let domain_match:    String = record.get(4).map(|s| s.to_string()).unwrap_or_else(|| String::new());
        // let domain_match: Strig = match record.get(4).map(|s| s.parse()) {
        //     Some(Ok(parsed_value)) if parsed_value => 'false',
        //     _ => continue, // Skip empty, invalid values, or false
        // };

        let domain_match: bool = match record.get(4).map(|s| s.parse()) {
            Some(Ok(parsed_value)) if parsed_value => false,
            _ => continue, // Skip empty, invalid values, or false
        };

        // Check conditions and write filtered rows to the output CSV
        // if domain_match && journal_ids.contains(&journal_id) {
        if journal_ids.contains(&journal_id) {
            output_writer.write_record(&[journal_id.to_string(), doi, url, doi_resolves_to, domain_match])?;
            // output_writer.write_record(&[journal_id.to_string(), doi, url, doi_resolves_to, domain_match.to_string()])?;
        }
    }

    output_writer.flush()?;
    Ok(())
}
