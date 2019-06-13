#![allow(unused)]
use std::ops::{Index,IndexMut};

pub struct Node<T> {
    elem: T,
    next: *mut Node<T>
}

// impl<T> Node<T>
// where T: std::default::Default {

// }


// #[derive(Default)]
pub struct LinkedList<T> {
  head: *mut Node<T>,
  size: usize,
}


impl<T> LinkedList<T>
where T: Copy + PartialEq + std::default::Default {

  // Creating a new list and initializing size to 0 and 
  // its data members to default
  pub fn new() -> Self {
    Self {
      size: 0,
      head: std::ptr::null_mut(),
    }
  }
  /*

  // Adding items at the end of the list
  pub fn append(&mut self, elem: T) {
    if (self.size >= 6) {
      // If size is greater than the capacity, panic
      panic!("Can't add elements, list full");
    }
    self.data[self.size] = elem;
    self.size += 1;
  }

  // Removing item at a given position from the list
  pub fn remove(&mut self, pos: usize) {
    if (pos > self.size) {
      // If given position is greater than size, panic
      panic!("Invalid position: {}", pos);
    } else if (self.empty()) {
      // If list is empty, panic
      panic!("Cannot remove from empty list");
    }
    for i in pos..5 {
      // Move elements after pos, 1 index to the left
      self.data[i] = self.data[i+1];
    }
    self.size -= 1;
  }


  pub fn insert(&mut self, elem: T, pos: usize) {
    if (pos > self.size) {
      panic!("Invalid position: {}", pos);
    }
    if (pos != self.size) {
      for i in (pos..(self.size)).rev() {
        self.data[i+1] = self.data[i];
      }
    }
    self.data[pos] = elem;
    self.size+=1;
  }

  pub fn find(&self, elem: T) -> i32 {
    let mut x = -1;
    for i in 0..self.size() {
      if (self.data[i] == elem) {
        x = i as i32;
      }
    }
    x
  }
 */
  pub fn get(&self, index: usize) -> T {
    if (index > self.size) {
      panic!("Invalid index: {}", index)
    }
    let mut n = self.head;
    for i in 0..index {
        n = (*n).next;
    }
    (*n).elem
    // self.data[index]
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


impl<T> Index<usize> for LinkedList<T> {
  type Output = T;
  fn index<'a>(&'a self, index: usize) -> &'a T {
    if (index > self.size) {
      panic!("Invalid index: {}", index)
    }
    let mut n = self.head;
    for i in 0..index {
        n = (*n).next;
    }
    &(*n).elem
  }
}


impl<T> IndexMut<usize> for LinkedList<T> {
  fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
    if (index > self.size) {
      panic!("Invalid index: {}", index)
    }
    let mut n = self.head;
    for i in 0..index {
        n = (*n).next;
    }
    &mut (*n).elem
  }
}


fn main() {
    let list = LinkedList::<i32>::new();
}