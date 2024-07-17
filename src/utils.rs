use crate::ListNode;

pub fn num_to_list(mut n: i32) -> Box<ListNode> {
    let n_string = format!("{}", n);
    if n_string.len() == 1 {
        return Box::new(ListNode::new(n as i32));
    }

    let mut divisor = 10i32.pow(n_string.len() as u32 - 1);
    let mut nodes = vec![];
    while n != 0 {
        let d = n / divisor;
        nodes.push(ListNode::new(d as i32));
        n -= d * divisor;
        divisor /= 10;
    }

    while nodes.len() < n_string.len() {
        nodes.push(ListNode::new(0));
    }

    let nodes = nodes.into_iter().rev().collect::<Vec<_>>();

    println!("Nodes: {:?}", nodes);
    nodes_to_list(nodes).unwrap()
}

// v is in normal order
pub fn nodes_to_list(v: Vec<ListNode>) -> Option<Box<ListNode>> {
    if v.len() == 0 { return None; }
    let mut v = v.into_iter().rev().peekable();
    let Some(mut prev_node) = v.next() else { return None; };
    let mut next_node;
    
    // Check if there is a next node
    while let Some(_) = v.peek() {
        next_node = v.next().unwrap();
        next_node.next = Some(Box::new(prev_node));
        prev_node = next_node;
    }

    Some(Box::new(prev_node))
}

pub fn reverse_list(mut l: Box<ListNode>) -> Box<ListNode> {
    let mut cur_node = l;
    // Take ownership of the remaining lists, and updating cur_node's next to None
    let mut next_node = cur_node.next.take();
    if let None = next_node { return cur_node; }

    // If the next node is not a null,
    while let Some(mut node) = next_node {
        // If the next node points to another node
        if let Some(_) = &node.next {
            // Take ownership of this another node
            let last_node = node.next.take();
            // Update next of last node
            node.next = Some(cur_node);
            cur_node = node;
            next_node = last_node;
        } else {
            // The next node points to None
            node.next = Some(cur_node);
            cur_node = node;
            next_node = None;
        }
    }

    // If the next node is a null

    cur_node
}