use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Parser)]
#[command(name = "hex_tool")]
#[command(about = "Read and write binary files in hexadecimal")]
struct Args {
    #[arg(short = 'f', long = "file")]
    file: Option<String>,

    #[arg(short = 'r', long = "read")]
    read: bool,

    #[arg(short = 'w', long = "write")]
    write: Option<String>,

    #[arg(short = 'o', long = "offset", default_value = "0")]
    offset: String,

    #[arg(short = 's', long = "size")]
    size: Option<usize>,
}

fn parse_offset(s: &str) -> u64 {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

fn main() {
    let args = Args::parse();

    let file_path = match &args.file {
        Some(f) => f,
        None => {
            eprintln!("error");
            std::process::exit(2);
        }
    };

    let offset = parse_offset(&args.offset);

    if let Some(hex_data) = &args.write {
        let bytes: Vec<u8> = (0..hex_data.len())
            .step_by(2)
            .filter_map(|i| u8::from_str_radix(&hex_data[i..i + 2], 16).ok())
            .collect();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(file_path)
            .unwrap();

        file.seek(SeekFrom::Start(offset)).unwrap();
        file.write_all(&bytes).unwrap();

        println!("Successfully written");
    } else if args.read {
        let mut file = File::open(file_path).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();

        let mut buffer = Vec::new();
        match args.size {
            Some(s) => {
                buffer.resize(s, 0);
                let _ = file.read(&mut buffer);
            }
            None => {
                file.read_to_end(&mut buffer).unwrap();
            }
        }

        for (i, chunk) in buffer.chunks(16).enumerate() {
            let addr = offset as usize + i * 16;
            print!("{:08x}: ", addr);

            for byte in chunk {
                print!("{:02x} ", byte);
            }

            for _ in chunk.len()..16 {
                print!(".. ");
            }

            print!("|");
            for byte in chunk {
                if *byte >= 32 && *byte < 127 {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }
            for _ in chunk.len()..16 {
                print!(".");
            }
            println!("|");
        }
    } else {
        eprintln!("error");
        std::process::exit(2);
    }
}
