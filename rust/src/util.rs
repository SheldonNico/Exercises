// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn add(&mut self, node: Self) {
        match self.next {
            None => {
                self.next = Some(Box::new(node));
            }
            Some(ref mut li) => {
                li.add(node);
            }
        }
    }
}

#[macro_export]
macro_rules! listnode {
    ( $a:expr, $( $x:expr ),* ) => {
        {
            let mut temp = ListNode::new($a);
            $(
                temp.add( ListNode::new($x) );
            )*
            temp
        }
    }
}
