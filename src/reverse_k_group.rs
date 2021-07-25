#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

type Node = Option<Box<ListNode>>;

pub fn list_from_vec(v: Vec<i32>) -> Node {
    v.iter().rev().fold(None, |acc, el| {
        Some(Box::new(ListNode {
            val: *el,
            next: acc,
        }))
    })
}

pub fn list_to_vec(head: &Node) -> Vec<i32> {
    let mut v = Vec::new();

    if let Some(h) = head {
        v.push(h.val);
        let mut cur = (*h).clone();
        while let Some(n) = cur.next {
            v.push(n.val);
            cur = n;
        }
    }
    v
}

pub fn try_take<'a>(head: &'a Node, k: i32) -> Result<&'a Node, &'a Node> {
    let mut cur = head;
    let mut n = 0;
    loop {
        match &cur {
            Some(c) => {
                n += 1;
                cur = &c.next;
                if n == k {
                    break;
                }
            }
            None => break,
        }
    }
    if n == k {
        Ok(cur)
    } else {
        Err(cur)
    }
}

pub fn collect_heads<'a>(head: &'a Node, k: i32) -> Vec<&'a Node> {
    let mut cur = head;
    let mut v = vec![cur];
    while let Ok(next) = try_take(&cur, k) {
        v.push(next);
        cur = next;
    }
    v
}

pub fn reverse_k_group(head: Node, k: i32) -> Node {
    let mut heads = collect_heads(&head, k);
    fn rev_sub_list(head: &Node, tail: Node, k: i32) -> Node {
        if k == 0 {
            tail
        } else {
            if let Some(h) = head {
                rev_sub_list(
                    &h.next,
                    Some(Box::new(ListNode {
                        val: h.val,
                        next: tail,
                    })),
                    k - 1,
                )
            } else {
                None
            }
        }
    }

    let mut new_tail = None;
    if let Some(tail) = heads.pop() {
        new_tail = (*tail).clone();
        while let Some(head) = heads.pop() {
            let new_head = rev_sub_list(head, new_tail, k);
            new_tail = new_head;
        }
    }
    new_tail
}
