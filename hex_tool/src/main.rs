use clap::Parser;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "hex_tool", about = "Read and write binary files in hexadecimal")]
struct Args {
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    file: PathBuf,

    #[arg(short = 'r', long = "read", conflicts_with = "write")]
    read: bool,

    #[arg(short = 'w', long = "write", value_name = "HEX", conflicts_with = "read")]
    write: Option<String>,

    #[arg(short = 'o', long = "offset", value_name = "OFF", default_value = "0")]
    offset: String,

    #[arg(short = 's', long = "size", value_name = "N")]
    size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let offset = match parse_offset(&args.offset) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("error: {msg}");
            std::process::exit(1);
        }
    };

    if args.read {
        let size = match args.size {
            Some(n) => n,
            None => {
                eprintln!("error: --size is required in read mode");
                std::process::exit(1);
            }
        };

        if let Err(e) = read_mode(&args.file, offset, size) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        return;
    }

    if let Some(hex) = args.write {
        if let Err(msg) = write_mode(&args.file, offset, &hex) {
            eprintln!("error: {msg}");
            std::process::exit(1);
        }
        println!("Successfully written");
        return;
    }

    eprintln!("error: choose either --read or --write");
    std::process::exit(1);
}

fn parse_offset(s: &str) -> Result<u64, String> {
    if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(rest, 16).map_err(|_| format!("invalid offset '{s}'"))
    } else {
        s.parse::<u64>().map_err(|_| format!("invalid offset '{s}'"))
    }
}

fn write_mode(path: &PathBuf, offset: u64, hex: &str) -> Result<(), String> {
    let bytes = parse_hex_string(hex)?;

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("cannot open file: {e}"))?;

    file.seek(SeekFrom::Start(offset))
        .map_err(|e| format!("cannot seek: {e}"))?;

    file.write_all(&bytes)
        .map_err(|e| format!("cannot write: {e}"))?;

    file.flush().map_err(|e| format!("cannot flush: {e}"))?;
    Ok(())
}

fn read_mode(path: &PathBuf, offset: u64, size: usize) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; size];
    let n = file.read(&mut buf)?;
    buf.truncate(n);

    print_hexdump(offset, &buf);

    Ok(())
}

fn print_hexdump(offset: u64, data: &[u8]) {
    const WIDTH: usize = 16;

    let base = offset - (offset % WIDTH as u64);
    let shift = (offset - base) as usize;

    let total = shift + data.len();
    let lines = (total + WIDTH - 1) / WIDTH;

    for line_idx in 0..lines {
        let line_addr = base + (line_idx as u64) * (WIDTH as u64);
        print!("{:08x}: ", line_addr);

        for i in 0..WIDTH {
            let global_pos = line_idx * WIDTH + i;

            if global_pos < shift {
                print!("..");
            } else {
                let data_pos = global_pos - shift;
                if data_pos < data.len() {
                    print!("{:02x}", data[data_pos]);
                } else {
                    print!("..");
                }
            }

            if i + 1 != WIDTH {
                print!(" ");
            }
        }

        print!(" |");

        for i in 0..WIDTH {
            let global_pos = line_idx * WIDTH + i;

            let ch = if global_pos < shift {
                '.'
            } else {
                let data_pos = global_pos - shift;
                if data_pos < data.len() {
                    let b = data[data_pos];
                    if b.is_ascii_graphic() || b == b' ' {
                        b as char
                    } else {
                        '.'
                    }
                } else {
                    '.'
                }
            };

            print!("{ch}");
        }

        println!("|");
    }
}

fn parse_hex_string(hex: &str) -> Result<Vec<u8>, String> {
    let s = hex.trim();

    if s.is_empty() {
        return Err("empty hex string".to_string());
    }
    if s.len() % 2 != 0 {
        return Err("hex string must have even length".to_string());
    }

    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let hi = from_hex_digit(bytes[i])?;
        let lo = from_hex_digit(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }

    Ok(out)
}

fn from_hex_digit(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err("invalid hex digit".to_string()),
    }
}