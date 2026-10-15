fn main() {
    let s: String = String::from("abhi");
    let (s, l) = cal_len(s); // shadowing
    println!("length of {} {}", s, l);
}

fn cal_len(name: String) -> (String, usize) {
    let len = name.len();
    (name, len)
}
