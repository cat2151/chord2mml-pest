use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChordParser;

/// Maps a chord name to its scale degree (1-7) in C major scale
/// C=1, D=2, E=3, F=4, G=5, A=6, B=7
fn chord_to_degree(chord: &str) -> u8 {
    match chord.to_uppercase().as_str() {
        "C" => 1,
        "D" => 2,
        "E" => 3,
        "F" => 4,
        "G" => 5,
        "A" => 6,
        "B" => 7,
        _ => 0, // Invalid chord
    }
}

/// Parse a chord progression string (e.g., "C-F-G-C") into a vector of scale degrees
pub fn parse_chord_progression(input: &str) -> Result<Vec<u8>, String> {
    let pairs = ChordParser::parse(Rule::chord_progression, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut degrees = Vec::new();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::chord {
                let chord = inner_pair.as_str();
                degrees.push(chord_to_degree(chord));
            }
        }
    }

    Ok(degrees)
}

/// WASM-compatible function that returns a comma-separated string
#[wasm_bindgen]
pub fn parse_chords_wasm(input: &str) -> String {
    match parse_chord_progression(input) {
        Ok(degrees) => degrees
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(","),
        Err(e) => format!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_c_f_g_c() {
        let result = parse_chord_progression("C-F-G-C").unwrap();
        assert_eq!(result, vec![1, 4, 5, 1]);
    }

    #[test]
    fn test_parse_single_chord() {
        let result = parse_chord_progression("C").unwrap();
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_parse_all_chords() {
        let result = parse_chord_progression("C-D-E-F-G-A-B").unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_case_insensitive() {
        let result = parse_chord_progression("c-f-g-c").unwrap();
        assert_eq!(result, vec![1, 4, 5, 1]);
    }

    #[test]
    fn test_wasm_function() {
        let result = parse_chords_wasm("C-F-G-C");
        assert_eq!(result, "1,4,5,1");
    }
}

