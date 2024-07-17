use crate::{utils::nodes_to_list, ListNode};

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (&l1, &l2) {
        (Some(_), Some(_)) => {},
        (_) => { return None; }
    }

    let mut nodes = vec![];
    let (Some(mut prev_node1), Some(mut prev_node2)) = (l1, l2) else { return None; };
    let mut result = prev_node1.val + prev_node2.val;
    let mut carry = 0;
    println!("{} + {}", prev_node1.val, prev_node2.val);
    println!("Result before carry: {}", result);
    if result >= 10 {
        carry = 1;
        result -= 10;
    }
    println!("Result: {}", result);
    nodes.push(ListNode::new(result));

    let mut next_node1;
    let mut next_node2;
    while let (Some(inext_node1), Some(inext_node2)) = (prev_node1.next.as_ref(), prev_node2.next.as_ref()) {
        let inext_node1 = prev_node1.next.unwrap();
        let inext_node2 = prev_node2.next.unwrap();
        next_node1 = inext_node1;
        next_node2 = inext_node2;
        result = next_node1.val + next_node2.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }
        println!("r: {}", result);
        nodes.push(ListNode::new(result));
        prev_node1 = next_node1;
        prev_node2 = next_node2;
    }

    let mut next_node1: Box<ListNode>;
    let mut next_node2: Box<ListNode>;
    while let Some(inext_node1) = prev_node1.next.as_ref() {
        let next_node1 = prev_node1.next.unwrap();
        result = next_node1.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }
        nodes.push(ListNode::new(result));
        prev_node1 = next_node1;
    }
    while let Some(inext_node2) = prev_node2.next.as_ref() {
        let next_node2 = prev_node2.next.unwrap();
        result = next_node2.val + carry;
        carry = 0;
        if result >= 10 {
            carry = 1;
            result -= 10;
        }
        nodes.push(ListNode::new(result));
        prev_node2 = next_node2;
    }

    if carry != 0 {
        nodes.push(ListNode::new(carry));
    }

    nodes_to_list(nodes)   
}