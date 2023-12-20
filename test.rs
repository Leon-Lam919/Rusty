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
}