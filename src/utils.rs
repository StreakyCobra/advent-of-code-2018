use std::fs::File;
use std::io::Read;
use std::process;

pub fn parse_file(name: &str) -> String {
    let mut file = match File::open(name) {
        Ok(file) => file,
        Err(_) => {
            println!("The input file «{}» is missing", name);
            process::exit(1)
        }
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            println!("The input file «{}» can not be read", name);
            process::exit(1)
        }
    };

    return content;
}
