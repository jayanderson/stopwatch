extern crate time;

use std::fmt;

pub struct Stopwatch {
  start: time::Timespec,
}

pub struct Elapsed {
  elapsed: u64,
}

/// TODO: more descriptive print
impl fmt::Show for Elapsed {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({})", self.elapsed)
  }
}

impl Stopwatch {
  pub fn new() -> Box<Stopwatch> {
    box Stopwatch {
      start: time::get_time(),
    }
  }

  pub fn elapsed(&self) -> Elapsed {
    let now = time::get_time();
    calc(&self.start, &now)
  }

  pub fn reset(&mut self) -> Elapsed {
    let now = time::get_time();
    let e = calc(&self.start, &now);
    self.start = now;
    e
  }
}

fn calc(start: &time::Timespec, end: &time::Timespec) -> Elapsed {
  let secs = end.sec - start.sec;
  let nsecs = end.nsec - start.nsec;
  Elapsed { elapsed: ((secs as u64) * 1000000) + (nsecs as u64), }
}

#[test]
fn bad_test() {
  let mut t = Stopwatch::new();
  for _ in range(0u8,5u8) {
    let e = t.reset();
    println!("{}", e);
  }
}
