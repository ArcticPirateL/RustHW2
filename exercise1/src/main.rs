struct Buffer<T> {
    vector: Vec<T>,
}
impl<T: for<'a> std::ops::AddAssign<&'a T> + Default> Buffer<T> {
    fn sum(&self) -> T {
        let mut s: T = T::default();
        for i in &(self.vector) {
            s += i;
        }
        return s;
    }
}
//测试样例
fn main() {
    let v1 = vec![1,2,3,4,5];
    let buf1 = Buffer::<usize> {vector: v1};
    println!("{}",buf1.sum());
    let v2 = vec![1.1, 2.2, 3.3];
    let buf2 = Buffer::<f32> { vector: v2};
    println!("{}",buf2.sum());
    let v3 = vec![11, 22, 33, 22];
    let buf3 = Buffer::<i8> { vector: v3};
    println!("{}",buf3.sum());
}