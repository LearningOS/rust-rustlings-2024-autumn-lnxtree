/*
	heap
	This question requires you to implement a binary heap function
*/


pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Heap {
            items: Vec::new(),
            comparator,
        }
    }

    pub fn insert(&mut self, item: T) {
        self.items.push(item);
        self.bubble_up(self.items.len() - 1);
    }

    pub fn extract(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let last_index = self.items.len() - 1;
        self.items.swap(0, last_index);
        let item = self.items.pop();
        self.bubble_down(0);
        item
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        while self.left_child_idx(idx) < self.items.len() {
            let smallest_child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                self.items.swap(idx, smallest_child);
                idx = smallest_child;
            } else {
                break;
            }
        }
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right >= self.items.len() {
            return left;
        }
        if (self.comparator)(&self.items[left], &self.items[right]) {
            right
        } else {
            left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let comparator = |a: &i32, b: &i32| a < b;
        let heap: Heap<i32> = Heap::new(comparator);
        assert_eq!(heap.items.len(), 0);
    }

    #[test]
    fn test_min_heap() {
        let comparator = |a: &i32, b: &i32| a < b;
        let mut heap = Heap::new(comparator);
        heap.insert(3);
        heap.insert(2);
        heap.insert(1);
        assert_eq!(heap.extract(), Some(1));
        assert_eq!(heap.extract(), Some(2));
        assert_eq!(heap.extract(), Some(3));
    }

    #[test]
    fn test_max_heap() {
        let comparator = |a: &i32, b: &i32| a > b;
        let mut heap = Heap::new(comparator);
        heap.insert(1);
        heap.insert(2);
        heap.insert(3);
        assert_eq!(heap.extract(), Some(3));
        assert_eq!(heap.extract(), Some(2));
        assert_eq!(heap.extract(), Some(1));
    }
}