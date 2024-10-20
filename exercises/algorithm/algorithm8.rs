/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/
/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/


use std::cell::RefCell;

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct MyStack<T> {
    q1: RefCell<Queue<T>>,
    q2: RefCell<Queue<T>>,
}

impl<T: Clone> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: RefCell::new(Queue::new()),
            q2: RefCell::new(Queue::new()),
        }
    }

    pub fn push(&self, elem: T) {
        self.q1.borrow_mut().enqueue(elem);
    }

    pub fn is_empty(&self) -> bool {
        self.q1.borrow().is_empty() && self.q2.borrow().is_empty()
    }

    pub fn pop(&self) -> Result<T, &str> {
        if self.q1.borrow().is_empty() {
            return Err("Stack is empty");
        }

        // 将 q1 中的元素移动到 q2，直到只剩下最后一个元素
        while self.q1.borrow().size() > 1 {
            if let Ok(elem) = self.q1.borrow_mut().dequeue() {
                self.q2.borrow_mut().enqueue(elem);
            }
        }

        // 直接出队最后一个元素
        let result = self.q1.borrow_mut().dequeue().unwrap(); // 使用?操作符处理错误并返回最后一个元素
        // 修改为返回拥有所有权的值
        let result = result.clone(); // 确保返回的是值而不是引用

        // 清空 q1，准备下一次操作
        // 重新将 q2 中的元素放回 q1
        while let Ok(elem) = self.q2.borrow_mut().dequeue() {
            self.q1.borrow_mut().enqueue(elem);
        }

        // 返回结果
        Ok(result) // 直接返回出队的结果
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}

fn main() {
    let stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{:?}", stack.pop()); // 输出: Ok(3)
}