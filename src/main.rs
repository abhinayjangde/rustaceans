use rustdev::math::simple::{add};
use rand;

fn main(){
    let a = 4;
    let b = 2;

    let num = rand::random::<u8>();
    
    println!("sum is {}", add(a, b));
    println!("random number = {}",num);
}