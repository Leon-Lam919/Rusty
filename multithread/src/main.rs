use std::thread;
use std::sync::{Mutex, Arc};

// max number of guest
const GUEST: usize = 25;


//Creates threads given the number of guest ~25~
pub fn threads(guest_count: usize){
    let counter = Arc::new(Mutex::new(0));

    let thrd = thread::spawn(move||{
        for i in 1..guest_count+1{
            let counter = Arc::clone(&counter);
            let mut mutex = counter.lock().unwrap();
        
        }
    });
    // Thread returns back and prints which thread it is
    thrd.join().unwrap();
}

fn main(){
    threads(5);
}