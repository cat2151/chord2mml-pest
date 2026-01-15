use chord2mml_pest::parse_chord_progression;

fn main() {
    println!("=== Chord2MML Parser Examples ===\n");

    // Example 1: Basic usage
    let input1 = "C-F-G-C";
    match parse_chord_progression(input1) {
        Ok(degrees) => {
            println!("Input: {}", input1);
            println!("Output: {:?}", degrees);
            println!("As string: {}\n", 
                degrees.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(","));
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 2: All chords
    let input2 = "C-D-E-F-G-A-B";
    match parse_chord_progression(input2) {
        Ok(degrees) => {
            println!("Input: {}", input2);
            println!("Output: {:?}\n", degrees);
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 3: Case insensitive
    let input3 = "c-f-g-c";
    match parse_chord_progression(input3) {
        Ok(degrees) => {
            println!("Input: {} (lowercase)", input3);
            println!("Output: {:?}\n", degrees);
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 4: Another progression
    let input4 = "G-C-D-G";
    match parse_chord_progression(input4) {
        Ok(degrees) => {
            println!("Input: {}", input4);
            println!("Output: {:?}", degrees);
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }
}
