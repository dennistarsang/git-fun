use std;
import std::io;


fn main() {
  let i = 0;
  while i <= 16 {
    io::println(#fmt("%d! = %u", i, factorial(i)));
    i = i + 1;
  }
}
fn factorial(x: int) -> uint {
  if (x <= 1) {
    ret 1u;
  } else {
    ret (x as uint) * factorial(x - 1);
  }
}