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
    let output_file_name = format!("{}.txt", input_file_name);
    if Path::new(&output_file_name).exists() {
        println!("{} already exists, remove", output_file_name);
        fs::remove_file(&output_file_name).expect("remove file failed");
    }

    // read a utf-8 string file and convert every 2 char hex to bytes, write out to a file
    let in_file = File::open(&input_file_name).expect("file not found");
    let mut buf_reader = BufReader::new(in_file);

    let out_file = File::create(&output_file_name).expect("create file failed");
    let mut buf_writer = BufWriter::new(out_file);

    let mut u8arr: [u8; 1] = [0];

    let mut index: u64 = 0;
    let mut count: u64 = 0;
    let mut line_count: u64 = 0;
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
        let c = u8arr[0];

        count += 3;
        line_count += 1;
        buf_writer
            .write_fmt(format_args!("{:02x} ", c))
            .expect("write file failed");

        if line_count == 16 {
            line_count = 0;
            count += 1;
            buf_writer.write(b"\n").expect("write file failed");
        }
    }
    buf_writer.flush().expect("flush failed");

    println!(
        "{} bytes read and {} chars written to {}",
        index, count, output_file_name
    );
}
