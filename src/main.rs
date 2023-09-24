use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use base64::decode;
use rand::Rng;
use chrono::{Local, DateTime, Utc};

fn main() {
    // Get the current working directory
    let executable_path = env::args()
    .next()
    .map(PathBuf::from)
    .expect("Failed to get executable path.");

    let executable_dir = executable_path
        .parent()
        .expect("Failed to get executable directory.");

    // raw csv
    let raw_file_name = "raw.csv";
    let raw_file_path = executable_dir.join(raw_file_name);
    let raw_file = File::open(raw_file_path).expect("Failed to open CSV file.");
    let raw_reader = BufReader::new(raw_file);
    let mut combined_rows = Vec::new();
    let mut current_row = String::new();

    for line in raw_reader.lines() {
        let line = line.expect("Failed to read line from raw CSV file.");
        if line.starts_with("\"{") {
            // Start of a new person's data
            if !current_row.is_empty() {
                combined_rows.push(current_row.clone());
                current_row.clear();
            }
            current_row.push_str(&line[1..]);
        } else {
            current_row.push_str(&line);
        }
    }
    combined_rows.push(current_row);

    // Specify the CSV file name
    //let file_name = "input.csv"; // Replace with the name of your CSV file

    // Construct the file path
    //let file_path = executable_dir.join(file_name);

    //let file = File::open(file_path).expect("Failed to open CSV file.");
    //let mut csv_reader = ReaderBuilder::new()
    //    .delimiter(b'\t') // Replace with the delimiter used in your CSV file
    //    .has_headers(false)
    //    .from_reader(file);

    // Iterate over each row in the CSV file
    //for result in csv_reader.records() {
    for result in &combined_rows {
        // Read the Base64 string from the CSV row
        //let record = result.expect("Failed to read CSV record.");
        let base64_string_replace = result.replace("\"", "");
        let base64_string = &base64_string_replace[9..&base64_string_replace.len()-1];
        //println!("{:?}", base64_string);

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
        println!("{:?}", base64_data);

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