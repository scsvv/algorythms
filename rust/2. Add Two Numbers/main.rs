fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n), 
        (Some(x1), Some(x2)) => {
            let sum = x1.val + x2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: Solution::add_two_numbers(x1.next, x2.next)
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: Solution::add_two_numbers(Solution::add_two_numbers(carry, x1.next), x2.next)
                }))
            }
        }
    }
}