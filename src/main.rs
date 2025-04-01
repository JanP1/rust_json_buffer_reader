use buffers::json_buffering::json_buffering_functions;

fn main() {

    let ammount = 20;

    match json_buffering_functions::read_json_chunk("json_reader_dummy.json", ammount, 0, false) {
        Ok(extracted_text) => {
            println!("-------------------\n {}  json object(s): \n------------------- \n{}", ammount, extracted_text);
        }
        Err(e) => {
            println!("Error reading file {}", e)
        }
    };
}
