/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    //双链表
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
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
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 尾插法
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    fn add_node(&mut self, node_ptr: NonNull<Node<T>>) {
        
        // 尾插法
        unsafe{(*node_ptr.as_ptr()).next = None};
        unsafe{(*node_ptr.as_ptr()).prev = self.end};

        match self.end {
            None => self.start = Some(node_ptr),
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = Some(node_ptr) },
        }
        self.end = Some(node_ptr);
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn reverse(&mut self) {
        // let p = self.start;
        // let mut q = self.end;
        // self.start = None;
        // self.end = None;

        // loop {
        //     match (p, q) {
        //         // node_a 和 node_b 解析出来是 NonNull<Node<T>> 类型
        //         // 这种写法是 Rust 中处理 NonNull 指针的标准模式：
        //         // 用 as_ref() 安全地获取引用
        //         // 用 & 避免所有权转移
        //         // 用 unsafe 明确标记潜在危险操作
                
        //         (Some(node_a), Some(node_b)) => {
        //             let mut temp = unsafe{node_b.as_ref()};
        //             q = unsafe{(*node_b.as_ptr()).prev};
        //             self.add_node(temp.into());// ref转换成NonNull格式
        //         }
        //         (_, None) => break,
        //         (None, _) => break,
        //     }
        // }
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe{
                current = (*node_ptr.as_ptr()).next;
                let temp = (*node_ptr.as_ptr()).prev;
                (*node_ptr.as_ptr()).prev = (*node_ptr.as_ptr()).next;
                (*node_ptr.as_ptr()).next = temp;
            }
        }
        std::mem::swap(&mut self.start, &mut self.end);
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
    fn test_reverse_linked_list_one_node() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2];
        let reverse_vec = vec![ 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        println!("Head: {:?}", list.start);
        println!("End: {:?}", list.end);

        for i in 0..original_vec.len() {
            println!("Index {}", i);
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
