#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    if v.is_empty() {
        None
    } else {
        v.into_iter().rev().fold(None, |acc, el| {
            Some(Box::new(ListNode { val: el, next: acc }))
        })
    }
}

pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut length = 0;
    let mut pointer = &mut head;
    while let Some(cur) = pointer {
        pointer = &mut cur.next;
        length += 1;
    }
    let (kb, ke) = (k, length - k + 1);
    let (left, right) = (kb.min(ke), kb.max(ke));
    println!("length is {}, left is {}, right is {}", length, left, right);
    //go to the left pointer, grab it's value
    pointer = &mut head;
    for _ in 1..left {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    let left_val = pointer.as_mut().unwrap().val;
    println!("we moved to left position, value is {}", left_val);
    //go to the right pointer, grap it's value and then set it's value to that of a left pointer
    for _ in left..right {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    let right_val = pointer.as_mut().unwrap().val;
    println!("we moved to right position, value is {}", right_val);
    pointer.as_mut().unwrap().val = left_val;
    //start from the beginning go to the left pointer again, set it's value to that of a right one
    pointer = &mut head;
    for _ in 1..left {
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    pointer.as_mut().unwrap().val = right_val;
    head
}
