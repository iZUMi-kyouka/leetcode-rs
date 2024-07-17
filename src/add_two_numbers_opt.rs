use crate::{utils::reverse_list, ListNode};

pub fn add_two_opt(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (Some(mut l1), Some(mut l2)) = (l1, l2) else { return None; };
    let mut carry;
    let mut result_list = ListNode::new(0);

    let mut result = l1.val + l2.val;
    if result >= 10 {
        carry = 1;
        result -= 10;
    } else {
        carry = 0;
    }
    result_list.val = result;

    while let (Some(next1), Some(next2)) = (l1.next.as_ref(), l2.next.as_ref()) {
        result = next1.val + next2.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }

        let mut new_node = ListNode::new(result);
        new_node.next = Some(Box::new(result_list));
        result_list = new_node;

        l1 = l1.next.unwrap();
        l2 = l2.next.unwrap();
    }

    while let Some(next1) = l1.next.as_ref() {
        result = next1.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }

        let mut new_node = ListNode::new(result);
        new_node.next = Some(Box::new(result_list));
        result_list = new_node;
        
        l1 = l1.next.unwrap();
    }

    while let Some(next2) = l2.next.as_ref() {
        result = next2.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }

        let mut new_node = ListNode::new(result);
        new_node.next = Some(Box::new(result_list));
        result_list = new_node;

        l2 = l2.next.unwrap();
    }

    if carry == 1 {
        result_list = ListNode{
            val: 1,
            next: Some(Box::new(result_list))
        };
    }

    Some(reverse_list(Box::new(result_list)))
}