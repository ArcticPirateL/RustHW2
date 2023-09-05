fn compareString(x: &str, y: &str) -> bool {
    let char_x : Vec<char> = x.chars().collect();
    let char_y : Vec<char> = y.chars().collect();
    let length = x.len().min(y.len());
    for i in 0..length {
        if char_x[i] > char_y[i] {
            return true;
        }
    }
    return false;
}
//测试样例
fn main() {
    let x: &str = "aabcd";
    let y: &str = "abcde";
    println!("{}",compareString(x, y));
    let x: &str = "aabcd";
    let y: &str = "aabcd";
    println!("{}",compareString(x, y));
    let x: &str = "abcde";
    let y: &str = "aaaaa";
    println!("{}",compareString(x, y));
}