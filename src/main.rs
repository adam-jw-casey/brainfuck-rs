#![warn(clippy::pedantic)]

use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let bytes = contents.as_bytes();

    let mut data = HashMap::<i32, u8>::new();
    let mut instruction_pointer: usize = 0;
    let mut data_pointer: i32 = 0;

    while instruction_pointer < bytes.len(){
        match bytes[instruction_pointer]{
            b'[' => {
                if *data.entry(data_pointer).or_default() == 0{
                    let mut depth = 0;
                    while instruction_pointer < bytes.len(){
                        match bytes[instruction_pointer]{
                            b'[' => depth += 1,
                            b']' => {
                                depth -= 1;
                                if depth == 0{
                                    break;
                                }
                            },
                            _ => (),
                        };
                        instruction_pointer += 1;
                    }
                }else{
                    instruction_pointer += 1;
                }
            },
            b']' => {
                #[allow(clippy::if_not_else)]
                if *data.entry(data_pointer).or_default() != 0{
                    let mut depth = 0;
                    while instruction_pointer < bytes.len(){
                        match bytes[instruction_pointer]{
                            b']' => depth += 1,
                            b'[' => {
                                depth -= 1;
                                if depth == 0{
                                    break;
                                }
                            },
                            _ => (),
                        };
                        instruction_pointer -= 1;
                    }
                }else{
                    instruction_pointer += 1;
                }
            },
            other => {
                match other{
                    b'<' => data_pointer -= 1,
                    b'>' => data_pointer += 1,
                    b'+' => (*data.entry(data_pointer).or_default(), _) = data.entry(data_pointer).or_default().overflowing_add(1),
                    b'-' => (*data.entry(data_pointer).or_default(), _) = data.entry(data_pointer).or_default().overflowing_sub(1),
                    b'.' => print!("{}", *data.entry(data_pointer).or_default() as char),
                    b',' => {std::io::stdin().read_exact(std::slice::from_mut(data.entry(data_pointer).or_default())).expect("Should be able to read input");},
                    _ => (),
                };
                instruction_pointer += 1;
            }
        }
    }
}
