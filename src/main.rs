use utils::num_to_list;

mod add_two_numbers_opt;
mod add_two_numbers;
mod utils;

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
    let n1 = num_to_list(342);
    let n2 = num_to_list(665);
    println!("n1: {:?}", n1);
    println!("n2: {:?}", n2);
    println!("n1 + n2 = {:?}", add_two_numbers_opt::add_two_opt(Some(n1), Some(n2)));
}