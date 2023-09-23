use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use base64::decode;
use rand::Rng;
use chrono::{Local, DateTime, Utc};
use csv::ReaderBuilder;

fn main() {
    // Get the current working directory
    let executable_path = env::args()
    .next()
    .map(PathBuf::from)
    .expect("Failed to get executable path.");
    //println!("executable_path");
    //println!("{:?}", executable_path);
    let executable_dir = executable_path
        .parent()
        .expect("Failed to get executable directory.");
    //println!("executable_dir");
    //println!("{:?}", executable_dir);
    // Specify the CSV file name
    let file_name = "input.csv"; // Replace with the name of your CSV file
    //println!("{:?}", file_name);
    // Construct the file path
    let file_path = executable_dir.join(file_name);
    //println!("{:?}", file_path);
    let file = File::open(file_path).expect("Failed to open CSV file.");
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b'\t') // Replace with the delimiter used in your CSV file
        .has_headers(false)
        .from_reader(file);

    // Iterate over each row in the CSV file
    for result in csv_reader.records() {
        // Read the Base64 string from the CSV row
        let record = result.expect("Failed to read CSV record.");
        let base64_string = record.get(0).unwrap_or_else(|| {
            panic!("Invalid Base64 string in CSV file.");
        });

        // Skip empty cells
        if base64_string.trim().is_empty() {
            continue;
        }

        // Extract the Base64-encoded audio data from the data URI
        let base64_data = match base64_string.strip_prefix("data:audio/wav;base64,") {
            Some(data) => data,
            None => {
                eprintln!("Invalid format: Base64 string does not have the expected data URI prefix.");
                continue;
            }
        };

        // Decode the Base64 string
        let decoded_data = match decode(base64_data) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to decode Base64 string: {}", error);
                continue;
            }
        };

        // Generate a random number
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen();

        // Get the current datetime
        let current_datetime: DateTime<Utc> = Local::now().into();

        // Create the file name
        let file_name = format!("{}_{}.wav", random_number, current_datetime.format("%Y%m%d%H%M%S"));

        // Create a file with the generated file name
        let file_path = executable_dir.join(&file_name);
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Failed to create file: {}", error);
                continue;
            }
        };

        // Write the decoded data to the file
        if let Err(error) = file.write_all(&decoded_data) {
            eprintln!("Failed to write to file: {}", error);
            continue;
        }

        println!("File created successfully: {}", file_name);
    }
}