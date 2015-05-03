#![feature(old_io,unicode)]

use std::old_io as io;
use std::collections::HashSet;

fn split_sentences(input: String) {
    if input.len() < 3 {
        // Assume that the input is a single sentence.
        println!("{}", input);
    }

    let chars: Vec<char> = input.chars().collect();
    let mut sentence_boundary = 0;

    // Find the next plausible sentence boundary.
    for (mut pos, _) in chars.iter().enumerate().filter(
            |&(_, &c)| c == '.' || c == '!' || c == '?') {
        // Periods followed by whitespace followed by a lower case letter are
        // not sentence boundaries.
        if pos + 2 < input.len() &&
              chars[pos + 1].is_whitespace() &&
              chars[pos + 2].is_lowercase() {
            continue
        }

        // Periods followed by a digit with no intervening whitespaces are not
        // sentence boundaries.
        if pos + 1 < input.len() && chars[pos + 1].is_digit(10) {
            continue
        }

        // Periods folled by whitespace and then an upper case letter, but
        // preceded by any of a short list of titles are not sentence
        // boundaries.
        let titles: HashSet<&'static str> =
                ["Mr.", "Mrs.", "Dr.", "Jr."].iter().map(|&x| x).collect();
        if pos >= 2 {
            let two_previous_chars: String =
                chars[pos-2..pos+1].iter().map(|&x| x).collect();
            if titles.contains(&two_previous_chars[..]) {
                continue
            }
        }
        if pos >= 3 {
            let three_previous_chars: String =
                chars[pos-3..pos+1].iter().map(|&x| x).collect();
            if titles.contains(&three_previous_chars[..]) {
                continue
            }
        }

        // Periods internal to a sentence of letter with no adjacent whitespace
        // are not sentence boundaries (e.g., websites).
        if pos + 1 < input.len() &&
              chars[pos + 1].is_alphabetic() &&
              chars[pos - 1].is_alphabetic() {
            continue
        }

        if pos + 1 < input.len() && chars[pos + 1] == ',' {
            continue
        }

        // Do nothing, we are just trying to consume any periods that are next
        // to each other.
        while pos + 1 < input.len() && chars[pos + 1] == '.' {
            pos += 1;
        }

        // We passed all the special conditions, so we probably have a sentence.
        if sentence_boundary < pos {
            let sentence: String =
                    chars[sentence_boundary..pos + 1].iter()
                    .map(|&x| x).collect();
            println!("{}", sentence);
        }

        sentence_boundary = pos + 2;
    }
}

fn main() {
    let mut stdin = io::stdin();
    let input = String::from_utf8(stdin.read_to_end().unwrap()).unwrap();
    split_sentences(input);
}
