<<<<<<< HEAD
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
=======
fn add(one: i32, two: i32) -> i32{
    one+two
}


fn main(){
    let array_1: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    let x = 5;
    let collected_vector: Vec<i32> = (0..10).collect();
    println!("Collected vector: {:?}", collected_vector);
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(9);
    println!("XS Vectors: {:?}", xs);
    // println!("The value of x is {x}.");
    // println!("The value of array_1 index 1 is {}.", array_1[1]);
>>>>>>> 913aae11ee42069f138d39cd0841df2db0fc063f
}