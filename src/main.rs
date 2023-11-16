use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
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
    if Path::new(&output_file_name).exists() {
        println!("{} already exists", output_file_name);
        fs::remove_file(&output_file_name).expect("remove file failed");
    }

    // println!("Hello, world!");
    // read a utf-8 string file and convert every 2 char hex to bytes, write out to a file
    let in_file = File::open(&input_file_name).expect("file not found");
    let mut buf_reader = BufReader::new(in_file);

    let out_file = File::create(&output_file_name).expect("create file failed");
    let mut buf_writer = BufWriter::new(out_file);

    let mut tmp: Vec<char> = Vec::new();
    let mut u8arr: [u8; 1] = [0];

    let mut index: u64 = 0;
    let mut count: u64 = 0;
    loop {
        let size = buf_reader.read(&mut u8arr);
        if size.is_err() {
            break;
        }

        let size = size.unwrap();
        if size == 0 {
            break;
        }

        index += 1;
        let c = u8arr[0] as char;

        // should skip \r \n 0x space
        if c == '\n' || c == '\r' || c == ' ' {
            continue;
        }

        tmp.push(c);

        if tmp.len() == 2 {
            if tmp[0] == '0' && tmp[1] == 'x' {
                tmp = Vec::new();
                continue;
            }

            let s: String = tmp.into_iter().collect();
            let b = u8::from_str_radix(&s, 16);
            if b.is_err() {
                println!("{}: {} is not a hex number", index, s);
                return;
            }

            let b = b.unwrap();
            count += 1;
            buf_writer.write(&[b]).expect("write file failed");
            tmp = Vec::new();
        }
    }
    buf_writer.flush().expect("flush failed");

    println!(
        "{} chars read and {} bytes written to {}",
        index, count, output_file_name
    );
}
