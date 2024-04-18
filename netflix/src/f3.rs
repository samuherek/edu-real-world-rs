use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Find the median between a certain set of values
///
/// Time complexity (insert) -> O(log n)
/// Time complexity (find) -> O(1)
/// Space complexity -> O(n)

#[derive(Debug)]
struct Median {
    top_half: BinaryHeap<Reverse<i32>>,
    bottom_half: BinaryHeap<i32>,
}

impl Median {
    fn new() -> Self {
        Self {
            top_half: BinaryHeap::new(),
            bottom_half: BinaryHeap::new(),
        }
    }

    fn insert(&mut self, value: i32) {
        // Insert int he right place
        if self.top_half.peek().map_or(true, |&Reverse(v)| value > v) {
            self.top_half.push(Reverse(value));
        } else {
            self.bottom_half.push(value);
        }
        // Rebalance the list -> we are looking for a median
        // Either have equal num of elements or bottom_Half has +1
        if self.top_half.len() > self.bottom_half.len() + 1 {
            let Reverse(num) = self.top_half.pop().unwrap();
            self.bottom_half.push(num);
        } else if self.top_half.len() < self.bottom_half.len() {
            self.top_half.push(Reverse(self.bottom_half.pop().unwrap()));
        }
    }

    fn get_value(&self) -> Option<f32> {
        if self.top_half.len() == self.bottom_half.len() {
            let Reverse(top) = self.top_half.peek()?;
            let bottom = self.bottom_half.peek()?;
            Some((top + bottom) as f32 / 2.0)
        } else {
            self.top_half.peek().map(|Reverse(v)| *v as f32)
        }
    }
}

mod test {

    #[test]
    fn test() {
        let mut m = super::Median::new();
        m.insert(20);
        m.insert(18);
        m.insert(50);
        m.insert(1);
        m.insert(2);
        m.insert(244);
        assert_eq!(Some(19.0), m.get_value());
        m.insert(18);
        assert_eq!(Some(18.0), m.get_value());
    }
}
