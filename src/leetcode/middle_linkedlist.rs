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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut current = &head;
    while let Some(node) = current {
        len += 1;
        current = &node.next;
    }

    let middle_idx = len / 2;

    if middle_idx == 0 {
        return head;
    }

    let mut curr = head;
    for _ in 0..middle_idx - 1 {
        if let Some(node) = curr {
            curr = node.next;
        }
    }

    if let Some(mut node) = curr {
        node.next.take()
    } else {
        None
    }
}
