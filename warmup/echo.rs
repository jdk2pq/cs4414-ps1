use std::{os};

fn main() {
  let args: ~[~str] = os::args();
  let mut it = args.iter().skip(1);
  for it.advance() |e| {
	print(*e+" ");
  }
  println("");
}
