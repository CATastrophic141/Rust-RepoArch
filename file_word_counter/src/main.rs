use input_conv;

fn main() {
    let mut counting_string: String = String::new();
    let file_path = "data_files/data.txt";
    let mut line_reader = match input_conv::LineReader::new(file_path) {
        Ok(lr) => lr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    while let Some(line) = line_reader.get_file_next_string().unwrap() {
        counting_string = counting_string + &line;
        //println!("geting line: {}", line);
    }

    let mut count = 0;
    let mut in_group = false;
    for c in counting_string.chars(){
        if c.is_whitespace() {
            in_group = false;
        } else if !in_group {
            count += 1;
            in_group = true;
        }
    }
    println!("Number of character groups: {}", count);
}