use std::fs::File;
use std::io::{Seek, SeekFrom, Read};


fn read_file_range(file_path: &str, start: u64, length: usize) 
    -> Result<String, std::io::Error> {

        let mut file = File::open(file_path)?;
        file.seek(SeekFrom::Start(start))?;

        let mut buffer = vec![0; length];

        file.read_exact(&mut buffer)?;

        Ok(String::from_utf8_lossy(&buffer).to_string())
}


fn main() {

    match read_file_range("large_file.txt", 6, 12) {
        Ok(extracted_text) => {
            println!("Extracted text: \n{}", extracted_text);
        }
        Err(e) => {
            println!("Error reading file {}", e)
        }
    };
}
