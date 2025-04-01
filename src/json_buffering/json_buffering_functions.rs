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

        let result : String = "[\n  ".to_owned() + &String::from_utf8_lossy(&buffer).to_string() + "\n]";
        Ok(result)
}


pub fn read_json_chunk(file_path: &str, max_num_of_obj: u64, start_index: u64, _forward: bool) -> Result<String, Box<dyn Error>> {
    
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::Start(start_index))?;

    let mut buffer = [0; 1]; // The actual buffer containing the current character

    let mut object_check_value = 0; // Value for checking how deeply nested the current character is
    let mut object_count = 0; // Value for keeping track of how many objects we have already read

    let mut beggining_index : usize = 0; // Starting index inside the file

    let mut current_index = 0;

    while reader.read(&mut buffer)? > 0 {
        let c = buffer[0] as char;
        //println!("Read character: {}", c); // To see where we are in the file

        match c {
            '{' => {
                if object_check_value == 0 && beggining_index == 0 {
                    beggining_index = current_index;
                }

                object_check_value += 1;
            }

            '}' => {
                object_check_value -= 1;
                if object_check_value == 0 {
                    object_count += 1;
                    if object_count == max_num_of_obj {
                        current_index += 1;

                        println!("Broke out of the loop for some reason. Object count -> {}", object_count);
                        break
                    } 
                }
            }

            ']' => {
                break
            }

            _ => {}
            
        }


        
        //println!("-check-value-{}-----", object_check_value);
        //println!("-objects-----{}-----", object_count);
        current_index += 1;
    }

    println!("Ending index {}", current_index);
    println!("Beggining index {}", beggining_index);
    return Ok(read_file_range(file_path, beggining_index.try_into()?, current_index - beggining_index)?)
    

}
