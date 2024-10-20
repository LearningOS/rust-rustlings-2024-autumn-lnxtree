/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    fn add(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let new_node = Some(Box::leak(new_node).into());

        match self.end {
            Some(mut end) => unsafe {
                end.as_mut().next = new_node;
            },
            None => self.start = new_node,
        }

        self.end = new_node;
        self.length += 1;
    }

    fn get(&self, index: u32) -> Option<&T> {
        let mut current = self.start;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(unsafe { &(*node.as_ptr()).val });
            }
            current = unsafe { (*node.as_ptr()).next };
            current_index += 1;
        }
        None
    }
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        while let (Some(a), Some(b)) = (list_a.start, list_b.start) {
            let a_val = unsafe { &(*a.as_ptr()).val };
            let b_val = unsafe { &(*b.as_ptr()).val };

            if a_val <= b_val {
                merged_list.add(a_val.clone());
                list_a.start = unsafe { (*a.as_ptr()).next };
            } else {
                merged_list.add(b_val.clone());
                list_b.start = unsafe { (*b.as_ptr()).next };
            }
        }

        while let Some(a) = list_a.start {
            let a_val = unsafe { &(*a.as_ptr()).val };
            merged_list.add(a_val.clone());
            list_a.start = unsafe { (*a.as_ptr()).next };
        }

        while let Some(b) = list_b.start {
            let b_val = unsafe { &(*b.as_ptr()).val };
            merged_list.add(b_val.clone());
            list_b.start = unsafe { (*b.as_ptr()).next };
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as u32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as u32).unwrap());
        }
    }
}