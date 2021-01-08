


// const CHR_A : u8 = 'a' as u8;
// const CHR_B : u8 = 'b' as u8;
//
// fn thuemorse(i : u8) -> Vec<u8> {
//     // if i == 0 { return None }
//     let n = 1<<i;
//     let mut str : Vec<u8> = Vec::with_capacity(n);
//     unsafe { str.set_len(n); }
//     str[0] = CHR_A;
//     for j in 0..i {
//         let powerj = 1<<j;
//         for k in 0..powerj {
//             str[powerj+k] = if str[k] == CHR_A { CHR_B } else { CHR_A }; 
//         }
//     }
//     return str
// }

// fn output_string<W: std::fmt::Write>(f: &mut W, s : &Vec<u8>) -> Result<(), std::fmt::Error> {
//     for val in s {
//         f.write_char(if *val == 0 as u8 { 'a' } else { 'b' })?;
//     }
//     Ok(())
// }

extern crate byte_string;
extern crate bit_vec;

#[macro_use]
extern crate more_asserts;

use byte_string::ByteString;

fn suffixarray(text: & ByteString) -> Vec<usize> {
    let mut sa = vec![0; text.len()];
    for i in 0..text.len() {
        sa[i] = i as usize;
    }
    sa.sort_by_key(|x| &text[*x..] );
    sa
}

fn inverse(sa : &Vec<usize>) -> Vec<usize> {
    let mut isa = vec![0; sa.len()];
    for i in 0..sa.len() {
        isa[sa[i]] = i as usize;
    }
    isa
}



fn duval(text: & ByteString) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let n = text.len();
    while k < n {
        let mut i = k;
        let mut j = k + 1;
        while j != n && text[i] <= text[j] {
            if text[i] < text[j] {
                i = k;
            }
            if text[i] == text[j] {
                i += 1;
            }
            j += 1;
        }
        loop {
            assert_lt!(i,j);
            k += j-i;
            ending_positions.push(k-1 as usize);
            if k >= i { break }
        }
    }
    return ending_positions;
}


fn lyndonfactorization(isa : &Vec<usize>) -> Vec<usize> {
    let mut ending_positions = Vec::new();
    let mut k = 0;
    let mut current_val = isa[k];
    let n = isa.len();
    k += 1;
    while k < n {
        if isa[k] < current_val {
            ending_positions.push(k-1 as usize);
            current_val = isa[k];
        }
        k += 1;
    }
    ending_positions.push(n-1);
    return ending_positions;
}

// fn random_string_alpha(len : usize) -> ByteString {
//     use rand::{thread_rng, Rng};
//     use rand::distributions::Alphanumeric;
//     return thread_rng().sample_iter(&Alphanumeric).take(len).map(u8::from).collect();
// }

#[allow(dead_code)]
fn random_string_binary(len : usize) -> ByteString {
    use rand::{thread_rng, Rng};
    use rand::distributions::Open01;
    return thread_rng().sample_iter(&Open01).take(len).map(|f : f32| -> u8 { return if f < 0.5 { 'a' as u8 } else { 'b' as u8}} ).collect();
}



#[test]
fn test_duval() {
    use std::cmp;
    for length in 1..30 {
        for _ in 0..cmp::min(1<<length,100) {
            let text = random_string_binary(length);
            let sa = suffixarray(&text);
            let isa = inverse(&sa);
            assert_eq!(duval(&text), lyndonfactorization(&isa));
        }
    }
}

use bit_vec::BitVec;
const L_TYPE: bool = true;
const S_TYPE: bool = false;


fn s_star_positions(text : & ByteString, lyndon_factorization: & Vec<usize>) -> Vec<usize> {
    let n = text.len();
    let mut sl_types = BitVec::from_elem(n, S_TYPE);
    let mut s_star_positions = Vec::new();
    // sl_types.set(n-1, S_TYPE);
    let mut current_lyndon_factor = lyndon_factorization.len()-1;
    //@ invariant: sl_type[0] = S_TYPE
    s_star_positions.push(n-1); //@ the last character has to be an S* type
    for k in (1..n).rev() {
        assert_lt!(k, n);
        if k == lyndon_factorization[current_lyndon_factor] && k-1 == lyndon_factorization[current_lyndon_factor-1] { 
            s_star_positions.push(k);
            continue; //@ if our Lyndon factor consists only of one character, it is S-type
        }
        let next_character = 
            if k != lyndon_factorization[current_lyndon_factor] { text[k+1] } 
            else {
            current_lyndon_factor -= 1;
            text[lyndon_factorization[current_lyndon_factor]] };
        if text[k] > next_character {
            sl_types.set(k, L_TYPE);
        } else if text[k] == next_character { 
           assert_eq!(text[k+1], next_character); //@ if they are the same, then there is no way we wrap around since Lyndon factors have no borders. So k+1 is the index of `next_character`
           sl_types.set(k, sl_types.get(k+1).unwrap()); 
        }
        if lyndon_factorization[current_lyndon_factor]+1 == k {
            assert_eq!(S_TYPE, sl_types.get(k).unwrap());
        }
        if sl_types.get(k).unwrap() == S_TYPE && sl_types.get(k+1).unwrap() == L_TYPE {
            s_star_positions.push(k);
        }
    }
    s_star_positions.push(0); //@ the first character has to be an S* type
    return s_star_positions;
}

fn lms_substrings(text : & ByteString, lyndon_factorization: & Vec<usize>, s_star_positions : & Vec<usize>) -> Vec<ByteString> {
    let mut lms_substrings = Vec::new();
    let mut current_lyndon_factor = 0;
    for i in 0..s_star_positions.len() {
        assert_ge!(s_star_positions[i], lyndon_factorization[current_lyndon_factor]);
        if s_star_positions[i+1] > lyndon_factorization[current_lyndon_factor] {
            let mut tail = text[s_star_positions[i]..lyndon_factorization[current_lyndon_factor]].to_vec();
            // let a = vec!(*tail);
            // tail.push('a' as u8);
            tail.push(if current_lyndon_factor == 0 { text[0] } else { text[lyndon_factorization[current_lyndon_factor-1]+1] });
            lms_substrings.push(ByteString::new(tail));
            current_lyndon_factor += 1;
        } else  {
            lms_substrings.push(ByteString::new(text[s_star_positions[i]..s_star_positions[i+1]].to_vec()));
        }
    }
    return lms_substrings;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [string]", args[0]);
        std::process::exit(1);
    }
    let string : String = args[1].parse().unwrap();
    let text = {
        let mut msg = ByteString::from(string.as_bytes().to_vec());
        msg[string.len()-1] = 0;
        msg
    };

    let lyndon_factorization = duval(&text);
    println!("{:?}", text);
    println!("duval: {:?}", duval(&text));
    let s_star_positions = s_star_positions(&text, &lyndon_factorization);
    println!("s-stars: {:?}", s_star_positions);
    println!("{:?}", lms_substrings(&text, &lyndon_factorization, &s_star_positions));


    // println!("string length: {}", text.len());
    // println!("duval: {:?}", duval(&text));
    //
    // let sa = suffixarray(&text);
    // let isa = inverse(&sa);
    // println!("lyndonfactorization: {:?}", lyndonfactorization(&isa));

    


    // let args: Vec<String> = std::env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("prints the i-th Thue-Morse word\nUsage: {} [number]", args[0]);
    //     std::process::exit(1);
    // }
    // let index : u8 = args[1].parse().unwrap();
    // use std::io::Write;
    // std::io::stdout().write_all(thuemorse(index).as_slice()).unwrap();
    // for i in 0..8 {
        // let s : Vec<u8> = thuemorse(i).into_iter().map(|x| if x == 0 as u8 { 'a' as u8 } else { 'b' as u8 }).collect();
        // let mut s = String::new();
        // output_string(&mut s, & thuemorse(i)).unwrap();
        // print!("{}", s);
        // // println!("{} {:?}", i, thuemorse(i));
    // }
}
