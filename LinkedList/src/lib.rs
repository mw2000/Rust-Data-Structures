#![allow(unused)]
use std::ops::{Index,IndexMut};


pub struct Node<T> 
where T: Clone + Copy + std::default::Default {
  elem: T,
  next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T>
where T: Copy + Clone+ std::default::Default {
  head: Option<Box<Node<T>>>,
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
    let mut current_node = &mut self.head;
    for _ in 0..n {
      match current_node {
          None => return None,
          Some(current) => current_node = &mut current.next,
      };
    }
    match current_node {
      None => None,
      Some(node) => Some(&mut **node),
    }
  }


  // Adding items at the end of the list
  pub fn append(&mut self, elem: T) {
    let new_node = Box::new(Node { elem: elem, next: None });
    if self.head.is_none() {
      self.head = Some(new_node);
    } else {
      let cur_size = self.size;
      let last_node = self.get_nth_node_mut(cur_size-1).unwrap();
      last_node.next = Some(new_node);
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
    if (index >= self.size) {
      panic!("Invalid index: {}", index)
    } else if self.head.is_none() {
      panic!("Can't get from empty linked list");
    }
    let n = self.get_nth_node_mut(index).unwrap();
    n.elem
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn empty(&self) -> bool {
    self.size == 0
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