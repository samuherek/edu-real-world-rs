use std::collections::LinkedList;

/// We have top movies in different countries.
/// We want to merge them together based on the position and the
/// id order. 1 is the most popular
/// Complexity is
/// Time: O(n x k2) -> n is num of list, k is len of the string
/// Space: O(1) -> n is num of list, k is len of the string

pub fn execute(list: Vec<LinkedList<i32>>) -> LinkedList<i32> {
    let mut res = LinkedList::new();

    if list.len() > 0 {
        let mut prev = list[0].clone();

        for l in 1..list.len() {
            let mut next = list[l].clone();
            while !prev.is_empty() && !next.is_empty() {
                if prev.front() <= next.front() {
                    res.push_back(prev.pop_front().unwrap());
                } else {
                    res.push_back(next.pop_front().unwrap());
                }
            }
            if prev.is_empty() {
                res.append(&mut next);
            } else {
                res.append(&mut prev);
            }
            if l != list.len() - 1 {
                prev = res;
                res = LinkedList::new();
            }
        }
    }

    res
}

mod test {
    use std::collections::LinkedList;
    #[test]
    fn merge_lists_by_ascending_order() {
        let mut a = LinkedList::new();
        a.push_back(11);
        a.push_back(41);
        a.push_back(51);
        let mut b = LinkedList::new();
        b.push_back(21);
        b.push_back(23);
        b.push_back(42);
        let mut c = LinkedList::new();
        c.push_back(25);
        c.push_back(56);
        c.push_back(66);
        c.push_back(72);
        let list = vec![a, b, c];
        let res = super::execute(list);

        assert_eq!(
            res,
            LinkedList::from([11, 21, 23, 25, 41, 42, 51, 56, 66, 72])
        );
    }
}
