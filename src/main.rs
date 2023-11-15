use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    // read file name from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let input_file_name = &args[1];
    let output_file_name = format!("{}.bin", input_file_name);

    // println!("Hello, world!");
    // read a utf-8 string file and convert every 2 char hex to bytes, write out to a file
    let mut file = File::open(input_file_name).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    // println!("With text:\n{}", contents);
    let mut bytes: Vec<u8> = Vec::new();
    let mut chars = contents.chars();
    let mut tmp: Vec<char> = Vec::new();
    loop {
        let c = chars.next();
        if c == None {
            break;
        }

        // should skip \r \n 0x space
        if c == Some('\n') || c == Some('\r') || c == Some(' ') {
            continue;
        }

        tmp.push(c.unwrap());

        if tmp.len() == 2 {
            if tmp[0] == '0' && tmp[1] == 'x' {
                tmp = Vec::new();
                continue;
            }

            let s: String = tmp.into_iter().collect();
            let b = u8::from_str_radix(&s, 16).expect("parse hex failed");
            bytes.push(b);
            tmp = Vec::new();
        }
    }
    let mut file = File::create(output_file_name).expect("create file failed");
    file.write_all(&bytes).expect("write file failed");
}
