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

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut tail = &mut dummy;

    while let Some(node) = head.take() {
        head = node.next.take();

        if node.val != val {
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}
