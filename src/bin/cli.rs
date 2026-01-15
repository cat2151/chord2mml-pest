use chord2mml_pest::parse_chord_progression;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <chord-progression>", args[0]);
        eprintln!("Example: {} C-F-G-C", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];

    match parse_chord_progression(input) {
        Ok(degrees) => {
            let result = degrees
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(",");
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
