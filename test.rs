fn add(one: i32, two: i32) -> i32{
    one+two
}


fn main(){
    let x: i32 = 5;
    let alpha_a: char = 'a';
    let alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    println!("The value of x is {x}.");
    println!("The letter of alpha_a is {alpha_a}.");
    println!("letter 3 of alphabet is {}.", alphabet[2]);
}