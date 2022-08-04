
extern crate env_logger;
extern crate log;
use log::debug;

use clap::Parser;

#[macro_use]
extern crate more_asserts;

/// reads a file into a u8 vector
/// - `prefix_length` : the prefix in bytes to read from `filename`. 0 means to read the entire file
pub fn file2byte_vector(filename: &str, prefix_length : usize) -> Vec<u8> {
    use std::fs;
    use std::io::Read;

    let path = std::path::Path::new(filename);
    let mut f = fs::File::open(&path).expect("no file found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let buffer_length = if prefix_length > 0 { std::cmp::min(prefix_length as u64, metadata.len()) } else { metadata.len() as u64};
    assert!(buffer_length <= std::usize::MAX as u64);
    let mut buffer = Vec::new();
    buffer.resize(buffer_length as usize, 0u8);

    match f.read_exact(&mut buffer) {
        Ok(()) => (), //assert_eq!(length, buffer.len()),
        Err(x) =>  panic!("{}", x)
    };
    buffer
}

#[derive(Debug,Clone)]
struct StackElement {
    text_pos : usize,
    period : usize
}

fn successor_element(text : &[u8], start: usize, value: u8) -> Option<usize> {
    if start >= text.len() {
        return None;
    }
    let list = leftmost_distinct_characters(& text[start..]);
    for el in list {
        if text[start+el] >= value {
            return Some(start+el);
        }
    }
    None
}

fn leftmost_distinct_characters(text : &[u8]) -> Vec<usize> {
    let mut charmap = std::collections::HashMap::new();
    for i in 0..text.len() {
        if charmap.contains_key(& text[i]) {
            continue;
        }
        charmap.insert(text[i], i);
    }
    let mut list = Vec::new();
    for el in charmap.values() {
        list.push(*el);
    }
    list.sort_by(|a, b| text[*a].partial_cmp(& text[*b]).unwrap());
    list
}

fn subsequence(text: &[u8], stack: &[StackElement]) -> Vec<u8> {
    let mut subseq = Vec::new();
    subseq.reserve_exact(stack.len());
    for el in stack {
        subseq.push(text[el.text_pos]);
    }
    // if ! stack.is_empty() {
    //     subseq.extend(stack.last().unwrap().period.to_string().as_bytes().into_iter());
    // }
    subseq
}

fn longest_lyndon_subsequence(text : &[u8]) -> Vec<StackElement> {
    let mut larray = Vec::new();
    larray.resize(text.len()+1, usize::MAX);
    
    let mut longest_lyndon_subsequence = Vec::new();

    let mut stack = Vec::new();
    let mut lastchildedgelabel = 0u8;
    let mut upwardmove = false;
    for starting_position in leftmost_distinct_characters(& text) { 
        stack.push(StackElement { text_pos: starting_position, period : 1 });
        if longest_lyndon_subsequence.is_empty() {
            longest_lyndon_subsequence = stack.clone();
        }
        while !stack.is_empty() {
            let top = stack.last().unwrap();
            let immature_character = text[stack[stack.len()-top.period as usize].text_pos];
            let compare_char = if upwardmove { lastchildedgelabel } else { immature_character };
            match successor_element(& text, top.text_pos+1, compare_char) {
                None => {
                    upwardmove = true;
                    lastchildedgelabel = text[top.text_pos]+1;
                    stack.pop(); () 
                },
                Some(i) => {
                    assert_lt!(top.text_pos, i);
                    assert_le!(compare_char, text[i]);
                    let subsequence_length = stack.len()+1;
                    if larray[subsequence_length] < i {
                        upwardmove = true;
                        lastchildedgelabel = text[i]+1;
                        debug!("{:?} + {} DISCARD", std::str::from_utf8(& subsequence(& text, & stack)).unwrap(), text[i]);
                    }
                    else {
                        let new_period = 
                            if immature_character == text[i] { top.period } else { subsequence_length };

                        stack.push(StackElement { text_pos: i, period : new_period });
                        if new_period == subsequence_length { //@ only update larray if we have a Lyndon subsequence
                            larray[subsequence_length] = i;
                            if longest_lyndon_subsequence.len() < subsequence_length {
                                longest_lyndon_subsequence = stack.clone();
                            }
                        }
                        upwardmove = false;
                    }
                }
            }
            debug!("{:?} FOUND", std::str::from_utf8(& subsequence(& text, & stack)).unwrap());
            // println!("{:?}", larray);
        }
    }
    longest_lyndon_subsequence
}

fn check_subsequence(text: &[u8], result: &[u8]) {
    assert_eq!(subsequence(& text, & longest_lyndon_subsequence(& text)), result);
}


#[test]
fn test_lyndon_subsequence() {
    check_subsequence(b"bccadbaccbcd", b"bccbccbcd");
    check_subsequence(b"bccadbaccbc", b"abaccbc");
    check_subsequence(b"bccadbaccb", b"abaccb");
    check_subsequence(b"bccadbacc", b"bccdcc");
    check_subsequence(b"a", b"a");
    check_subsequence(b"aa", b"a");
    check_subsequence(b"aaa", b"a");
    check_subsequence(b"aaab", b"aaab");
    check_subsequence(b"aaaba", b"aaab");
}


/// Computes the longest Lyndon subsequence
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// input filename
   #[clap(short, long, value_parser)]
   filename: String,

   /// the number of characters to read from the input file
   #[clap(short, long, value_parser, default_value_t = 0)]
   prefix: usize,
}

fn main() {
	let args = Args::parse();
    //
    //
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Usage: {} [string]", args[0]);
    //     std::process::exit(1);
    // }
    let text = file2byte_vector(& args.filename, args.prefix);
    // println!("{:?} {:?}", text.len(), args.prefix);

    let stack_subsequence = longest_lyndon_subsequence(& text);
    println!("{}", std::str::from_utf8(& subsequence(& text, & stack_subsequence)).unwrap());
}
