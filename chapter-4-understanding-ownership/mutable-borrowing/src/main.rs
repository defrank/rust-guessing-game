fn main() {
    let mut s = create();
    s.push_str(", world!");
    println!("{}", s);
}

fn create() -> String {
    String::from("Hello")
}
