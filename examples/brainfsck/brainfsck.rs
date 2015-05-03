#![feature(env)]
#![feature(old_io)]
#![feature(old_path)]

use std::collections::HashMap;
use std::char;
use std::old_io as io;
use std::old_io::{File, IoResult};
use std::old_io::SeekStyle::*;
use std::env::args;

const NUM_CELLS: usize = 30000;


fn parse_labels(file: &mut File) -> IoResult<HashMap<u64, u64>> {
    let mut labels: HashMap<u64, u64> = HashMap::new();
    let mut parsed_labels = Vec::new();

    let string = try!(file.read_to_string());

    for (i, c) in string.chars().enumerate() {
        let pos = i + 1;

        match c {
            '[' => {
                parsed_labels.push(pos);
            },
            ']' => {
                if parsed_labels.is_empty() {
                    panic!("']' encountered without matching ']'");
                }

                let last_label = parsed_labels.pop().unwrap();
                labels.insert(pos as u64, last_label as u64);
                labels.insert(last_label as u64, pos as u64);
            }
            _ => {}
        }
    }

    try!(file.seek(0, SeekSet));

    Ok(labels)
}

fn seek_to_label_match(file: &mut File, labels: &HashMap<u64, u64>) -> IoResult<()> {
    let file_pos = &try!(file.tell());
    let label_match = *labels.get(file_pos).unwrap() as i64;
    try!(file.seek(label_match, SeekSet));

    Ok(())
}

fn process_op_code(file: &mut File, labels: &HashMap<u64, u64>, cells: &mut [i32],
                 index: &mut usize) -> IoResult<()> {
    let op_code = char::from_u32(try!(file.read_byte()) as u32).unwrap();

    match op_code {
        '>' => *index += 1,
        '<' => *index -= 1,
        '+' => cells[*index] += 1,
        '-' => cells[*index] -= 1,
        '.' => print!("{}", char::from_u32(cells[*index] as u32).unwrap()),
        ',' => {
            let mut stdin = io::stdin();
            cells[*index] = try!(stdin.lock().read_byte()) as i32;
        },
        '[' => if cells[*index] == 0 {
            let _ = seek_to_label_match(file, labels);
        },
        ']' => if cells[*index] != 0 {
            let _ = seek_to_label_match(file, labels);
        },
        _ => {}
    }

    Ok(())
}

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        println!("Usage: brainfsck [.bf file]");
        return
    }

    let file_path = Path::new(&args[1]);
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => panic!(err),
    };

    let mut cells = [0; NUM_CELLS];
    let mut index = 0;
    let labels = match parse_labels(&mut file) {
        Ok(labels) => labels,
        Err(err) => panic!(err)
    };

    while !file.eof() {
        match process_op_code(&mut file, &labels, &mut cells, &mut index) {
            Ok(_) => {}
            Err(err) => {
                // The file can't be read anymore, don't do anything and assume it worked.
                break
            }
        }
    }
}
