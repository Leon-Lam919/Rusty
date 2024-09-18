use std::fs::File;
use std::io::{BufReader, Read};

/* sol 2015-1 
fn main(){
    let mut str = String::new();
    let file = File::open("src/input.txt").expect("Error opening file");

    let mut buf = BufReader::new(file);

    buf.read_to_string(&mut str);

    let mut bal = 0;
    let mut pos = 0;

    for c in str.chars(){
        if c == '(' {
            bal += 1;
            pos += 1;
        } if c == ')'{
            bal -= 1;
            pos += 1;
        }
        if bal == -1{
            break;
        }
    }

    println!("{}", pos);
} 
*/

fn area(l:u32, w:u32, h:u32) -> u32{
    let mut least: u32 = 0;
    least = l.min(w).min(h);
    (2*l*w)+(2*w*h)+(2*h*l)+least
}

// 2015-2
fn main(){

    let mut str = String::new();
    let file = File::open("src/input.txt").expect("Error opening file");
    
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number = 0;

    for c in str.chars() {
        if c.is_ascii_digit(){
            current_number = current_number *10 + c.to_digit(10).unwrap();
        }else {

        }
    }
    println!("{}", current_number);
}