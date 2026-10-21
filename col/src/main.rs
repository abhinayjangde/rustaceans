fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero."));
    }
    Ok(a/b)
}
fn main() {
    let result = match divide(4,3)   {
        Ok(num) => num,
        Err(err) => {
            println!("Error {err}");
            -1
        }
    }; 

    println!("result is {:?}", result);
}
