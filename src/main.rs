use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::u64;


fn read_file_range(file_path: &str, start: u64, length: usize) 
    -> Result<String, Box<dyn Error>> {

        let mut file = File::open(file_path)?;
        file.seek(SeekFrom::Start(start))?;

        let mut buffer = vec![0; length];

        file.read_exact(&mut buffer)?;

        Ok(String::from_utf8_lossy(&buffer).to_string())
}

//Consider result with box pointer for errors so when the function returns an error it passes it
//upwards
fn read_json_chunk(file_path: &str, max_num_of_obj: u64) -> Result<String, Box<dyn Error>> {
    
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 1]; // The actual buffer containing the current character

    let mut object_check_value = 0; // Value for checking how deeply nested the current character is
    let mut object_count = 0; // Value for keeping track of how many objects we have already read

    let mut beggining_index : usize = 0; // Starting index inside the file
    let mut ending_index : usize = 0; 

    let mut current_index = 0;

    while reader.read(&mut buffer)? > 0 {
        let c = buffer[0] as char;
        println!("Read character: {}", c); // To see where we are in the file

        if c == '{' {
            if object_check_value == 0 && beggining_index == 0{
                beggining_index = current_index;
            }
            println!("Found the character '{}'", c);
            object_check_value += 1;
        }
        if c == '}' {
            println!("Found the character '{}'", c);
            object_check_value -= 1;

        }

        if object_check_value == 0 && c == '}'{
            object_count += 1;
            println!("Object count -> {}", object_count);
            if object_count == max_num_of_obj {
                ending_index = current_index;

                println!("Broke out of the loop for some reason. Object count -> {}", object_count);
                break;
            } 
        }

        println!("-check-value-{}-----", object_check_value);
        println!("-objects-----{}-----", object_count);
        current_index += 1;
    }

    if ending_index != 0 {

        println!("Ending index {}", ending_index);
        println!("Beggining index {}", beggining_index);
        return Ok(read_file_range(file_path, beggining_index.try_into()?, ending_index - beggining_index)?)
    }
    

    Ok("Ok".to_string())
}

fn main() {

    match read_file_range("json_reader_dummy.json", 4, 165) {
        Ok(extracted_text) => {
            println!("-------------------\n  File read: \n------------------- \n {}", extracted_text);
        }
        Err(e) => {
            println!("Error reading file {}", e)
        }
    };


    match read_json_chunk("json_reader_dummy.json", 6) {
        Ok(extracted_text) => {
            println!("-------------------\n  Chunk processer: \n------------------- \n {}", extracted_text);
        }
        Err(e) => {
            println!("Error reading file {}", e)
        }
    };
}
