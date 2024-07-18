use std::env::args;

use longest_nonrepeating_substring::length_of_longest_substring;
use utils::num_to_list;

mod add_two_numbers_opt;
mod add_two_numbers;
mod utils;
mod longest_nonrepeating_substring;
mod two_sum_v2;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
  let v = vec![2, 3, 4];
  let answer = two_sum_v2::two_sum(v, 6);
  println!("{:?}", answer);
}