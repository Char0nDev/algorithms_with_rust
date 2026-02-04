use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
    let mut connected_components = 0;
    let nums_set: HashSet<i32> = nums.into_iter().collect();

    let mut current = &head;
    let mut in_component = false;

    while let Some(node) = current {
        if nums_set.contains(&node.val) {
            if !in_component {
                connected_components += 1;
                in_component = true;
            }
        } else {
            in_component = false;
        }
        current = &node.next;
    }

    return connected_components;
}
