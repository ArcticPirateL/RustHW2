# Rust HW2



3210104927   刘子鸣



## Exercise 1



实现一个Buffer<T>，Buffer只有一个成员Vec<T>，并实现了一个方法Sum，这个方法会返回全部成员的和。



- 代码：

  ```rust
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
  ```

- 测试样例：

  ```rust
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
  ```

- 运行结果：

  ![image-20230905142318867](/images/image-20230905142318867.png)



## Exercise 2



尝试实现一个函数`fn compareString(x:&str，y:&st tr)-> bool`，返回true表示x比y在字典序上更大，否则返回false。不要使用现成的比较函数和库。



- 代码：

  ```rust
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
  ```

- 测试样例：

  ```rust
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
  ```

- 测试结果：

  ![image-20230905144151704](/images/image-20230905144151704.png)



## Exercise 3



将一个`Vec<char>`内容为`a,b,c,d,e`，通过闭包+迭代代器生成一个新的`Vec<char>`，内容是`b,c,d,e,f`。



- 代码：

  ```rust
  fn main() {
      let vec: Vec<char> = vec!['a','b','c','d','e'];
      let vec: Vec<char> = vec.iter().map(|x|(*x as u8 + 1) as char).collect::<Vec<char>>();
      for i in vec.iter() {
          println!("{}", i);
      }
  }
  ```

- 运行结果：

  ![image-20230905170447053](/images/image-20230905170447053.png)



