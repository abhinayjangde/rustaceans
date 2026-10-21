use std::fs::File;

fn main() {
    let open_file = File::open("hello.txt");

    let file = match open_file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("problem creating file. {err}")
            },
            _ => panic!("Something went wrong.")
        }
    };

    println!("this is file {:?}", file);
}
