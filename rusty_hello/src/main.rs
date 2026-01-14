use clap::Parser;

#[derive(Parser)]
#[command(name = "rusty_hello")]
struct Args {
    #[arg(default_value = "World")]
    name: String,

    #[arg(long)]
    upper: bool,

    #[arg(long, default_value_t = 1)]
    repeat: usize,
}

fn main() {
    let args = Args::parse();

    let mut msg = format!("Hello, {}!", args.name);

    if args.upper {
        msg = msg.to_uppercase();
    }

    for _ in 0..args.repeat {
        println!("{msg}");
    }
}
