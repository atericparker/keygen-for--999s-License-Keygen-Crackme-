
use std::io::Read;

use sha2::{Sha256, Digest};

pub fn GenerateSegment(input : String) -> String {
    let mut hasher = Sha256::new();
    let _ = hasher.update(input.bytes().collect::<Vec<u8>>());
    let binding = hasher.finalize();
    let sha = binding.bytes();
    println!("{:?}", &input);
    let shachars : Vec<u8> = sha.into_iter().map(|b|b.unwrap()).collect();
    let mut buf = String::new();
    for i   in (0 as usize..2 as usize) {
        let num = (shachars[i]  % 36);
        let num2 = (shachars[i + 2] % 36);
        let value;
        if num < 10 {
            value = (48+num) as char;
        } else {
            value = (55 + (num ))as char;
        }

        let value2;
        if num2 < 10 {
            value2 = (48+num2) as char;
        } else {
            value2 = (55 + (num2 ))as char;
        }

        buf.push(value);
        buf.push(value2);
    }
    buf

}
pub fn generatePassword(input: String) -> String {
    let part1 = GenerateSegment(input.clone());
    let part2 = GenerateSegment(format!("{}{}", &input, "salt1"));
    let part3 = GenerateSegment(format!("{}{}", &input, "salt2"));
    let wholepassword = format!("{}{}{}", part1,&part2, &part3);
    wholepassword
}   