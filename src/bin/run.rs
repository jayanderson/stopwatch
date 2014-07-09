extern crate stopwatch;

use stopwatch::Stopwatch;

fn main() {
  let mut t = Stopwatch::new();
  for _ in range(0u8,5u8) {
    let e = t.reset();
    println!("{}", e);
  }
}
