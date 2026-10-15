fn main() {
    let nums: [i32; 5] = [1,2,3,4,5]; // array - similary data type
    let user: (&str, u8, bool) = ("Abhinay", 24, true); // tuple

    // for dynamic array we can use Vector

    greet();
    println!("nums {}", nums[3]);
    println!("name is {} ", user.0)
}

fn greet() {
    println!("hello");
}