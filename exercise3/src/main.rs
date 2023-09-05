fn main() {
    let vec: Vec<char> = vec!['a','b','c','d','e'];
    let vec: Vec<char> = vec.iter().map(|x|(*x as u8 + 1) as char).collect::<Vec<char>>();
    for i in vec.iter() {
        println!("{}", i);
    }
}