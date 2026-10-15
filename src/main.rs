
fn main() {
    let user: (&str, u8, bool) = ("abhi", 24, true); // tuple

    greet(user.0);
   
}

fn greet(name: &str) {
    println!("hello, {name}");
}

