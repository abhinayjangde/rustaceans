
struct Person {
    name: String,
    age: u32,
    is_admin: bool
}
fn main() {
    
    let user = Person {
        name: String::from("Abhi"),
        age: 24,
        is_admin: true,
    };

    println!("name {} age {} is_admin {}", user.name, user.age, user.is_admin);

}
