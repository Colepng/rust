// aux-build:issue_2723_a.rs

use issue_2723_a;
import issue_2723_a::*;

fn main() unsafe {
  f(~[2]);
}