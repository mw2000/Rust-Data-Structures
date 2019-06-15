#![allow(unused)]
use std::ops::{Index,IndexMut};


pub struct Node<T> 
where T: Clone + Copy + std::default::Default {
  elem: T,
  next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T>
where T: Copy + Clone+ std::default::Default {
  head: Option<Node<T>>,
  size: usize,
}


impl<T> LinkedList<T>
where T: Copy + Clone + std::default::Default + PartialEq {

  // Creating a new list and initializing size to 0 and 
  // its data members to default
  pub fn new() -> Self {
    Self {
      size: 0,
      head: None,
    }
  }
  
  fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node<T>> {
    let mut nth_node = self.head.as_mut();
    for _ in 0..n {
      nth_node = match nth_node {
        None => return None,
        Some(node) => node.next.as_mut().map(|node| &mut **node),
      }
    }
    nth_node
  }


  // Adding items at the end of the list
  pub fn append(&mut self, elem: T) {
    let mut new_node = Node { elem: elem, next: None };
    if self.head.is_none() {
      self.head = Some(new_node);
    } else {
      let cur_size = self.size - 1;
      let mut last_node = self.get_nth_node_mut(cur_size).unwrap();
      last_node.next = Some(Box::new(new_node));
    }
    self.size += 1;
  }


  pub fn find(&mut self, elem: T) -> i32 {
    let mut x = -1;
    for i in 0..self.size() {
      if (self.get_nth_node_mut(i).unwrap().elem == elem) {
        x = i as i32;
      }
    }
    x
  }
 

  pub fn get(&mut self, index: usize) -> T {
    if (index > self.size) {
      panic!("Invalid index: {}", index)
    } else if self.head.is_none() {
      panic!("Can't get from empty linked list");
    }
    let mut n = self.get_nth_node_mut(index).unwrap();
    n.elem
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn empty(&self) -> bool {
    if (self.size == 0) {
      true
    } else {
      false
    }
  }
}


fn main() {
  let mut list = LinkedList::<i32>::new();
  list.append(3);
  list.append(4);
  list.append(5);
  list.append(6);
  list.append(7);
  println!("{}", list.find(4));
}