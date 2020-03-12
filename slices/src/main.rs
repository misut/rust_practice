fn main() {
    let s = String::from("Hello, world!");

    let idx = first_word(&s);

    println!("{}", idx);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
