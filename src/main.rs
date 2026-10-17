
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main(){
    let dim = Rectangle{width:4, height:4};

    println!("area of {:?} is {} ", dim, dim.area()); // borrowing (reference)
}
