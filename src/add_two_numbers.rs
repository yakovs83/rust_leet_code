// Definition for singly-linked list.
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

type OBLN = Option<Box<ListNode>>;

pub fn add_two_numbers(l1: OBLN, l2: OBLN) -> OBLN {
    fn add_rec(l1: OBLN, l2: OBLN, cr: i32) -> OBLN {
        match (l1, l2) {
            (Some(bl1), Some(bl2)) => {
                let sum = bl1.val + bl2.val + cr;
                let (d, c) = (sum % 10, sum / 10);
                Some(Box::new(ListNode {
                    next: add_rec(bl1.next, bl2.next, c),
                    val: d,
                }))
            }
            (Some(bl1), None) => {
                if cr == 0 {
                    Some(bl1)
                } else {
                    let sum = bl1.val + cr;
                    let (d, c) = (sum % 10, sum / 10);
                    Some(Box::new(ListNode {
                        next: add_rec(bl1.next, None, c),
                        val: d,
                    }))
                }
            }
            (None, Some(bl2)) => {
                if cr == 0 {
                    Some(bl2)
                } else {
                    let sum = bl2.val + cr;
                    let (d, c) = (sum % 10, sum / 10);
                    Some(Box::new(ListNode {
                        next: add_rec(None, bl2.next, c),
                        val: d,
                    }))
                }
            }
            (None, None) => {
                if cr == 0 {
                    None
                } else {
                    Some(Box::new(ListNode::new(cr)))
                }
            }
        }
    }
    add_rec(l1, l2, 0)
}
//helper function for tests
pub fn list_from_vec(v: Vec<i32>) -> OBLN {
    fn rec_from_vec(mut it: impl Iterator<Item = i32>) -> OBLN {
        if let Some(n) = it.next() {
            Some(Box::new(ListNode {
                val: n,
                next: rec_from_vec(it),
            }))
        } else {
            None
        }
    }
    rec_from_vec(v.into_iter())
}

#[test]
fn basic_test() {
    assert_eq!(
        add_two_numbers(list_from_vec(vec![1, 2, 3]), list_from_vec(vec![9, 8, 9])),
        list_from_vec(vec![0, 1, 3, 1])
    );
}
