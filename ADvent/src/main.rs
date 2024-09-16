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

// 2015-2
fn main(){

    let mut str = String::new();
    let file = File::open("src/input.txt").expect("Error opening file");
    
}