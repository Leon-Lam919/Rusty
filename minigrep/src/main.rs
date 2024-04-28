use std::fs;
use std::env;
mod read;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    read::read_in(&filename);
}