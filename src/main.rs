fn main() {
  println!("Hello World!");
  let i: i32 = 64;
  let mut sum: i32 = 0;
  for x in 0..=64 {
    sum = sum + x 
  }
  let sql: i32 = sqr(sum);
  println!("{}", sql);
}

fn sqr(x: i32) -> i32 {
  x*x
}
