use std::{fs::File, io::Read, process::exit};

const MEM_SIZE: usize = 30_000;

fn main() {
    let filename: &str = "test.bf";
    println!("Opening file \"{}\"", filename);
    let file_result: Result<File, std::io::Error> = File::open(filename);
    match file_result {
        Err(_) => {
            println!("Fatal Error: Failed to open the file");
            exit(1);
        },
        Ok(_) => {}
    }
    let mut file: String = String::new();
    file_result.unwrap().read_to_string(&mut file).unwrap(); // Trust me, this will surely work ... surely ._.

    exit(interpret_bf(file) as i32);
}

/* 
    @returns exit code of the bf
*/
fn interpret_bf(code: String) -> u8 {
    if code.len() == 0 {
        return 0;
    }
    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mut mem_ptr: usize = 0;
    let mut current_index: usize = 0;
    let mut current_char_option: Option<char> = code.chars().nth(current_index);

    while current_char_option.is_some() {
        let mut current_char: char = current_char_option.expect("Fatal Error: failed to unwrap() current_char_option!");
        match current_char {
            '+' => mem[mem_ptr] += 1,
            '-' => mem[mem_ptr] -= 1,
            '>' => mem_ptr += 1,
            '<' => mem_ptr -= 1,
            '.' => print!("{}", mem[mem_ptr] as char),
            ',' => mem[mem_ptr] = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8)
            .expect("Failed to read char from stdin!"),
            '[' => {},
            ']' => 'label: {
                if mem[mem_ptr] == 0 {
                    break 'label;
                }
                let mut depth: u64 = 1;
                while depth != 0 {
                    current_index -= 1;
                    current_char_option = code.chars().nth(current_index);
                    // current_char = current_char_option.expect("Fatal Error: failed to unwrap() current_char_option!");
                    current_char = current_char_option.unwrap(); // It will panic if there are more ]'s than ['s
                    match current_char {
                        '[' => depth -= 1,
                        ']' => depth += 1,
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        current_index += 1;
        current_char_option = code.chars().nth(current_index);
    }
    return mem[current_index];
}