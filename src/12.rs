use std::fs;

fn main() {
    let file = fs::File::open("sample.txt").expect("file not found");
    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
