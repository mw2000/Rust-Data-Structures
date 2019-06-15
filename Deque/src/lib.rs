// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

#[derive(Default)]
pub struct Deque<T> {
  size: usize,
  data: std::collections::LinkedList<T>,
}

impl<T> Deque<T>
where T: Copy + Clone {
  
  pub fn new() -> Self {
    Self {
      size: 0,
      data: std::collections::LinkedList::<T>::new(),
    }
  }

  pub fn push_front(&mut self, elem: T) {
    self.data.push_front(elem);
    self.size += 1;
  }

  pub fn push_back(&mut self, elem: T) {
    self.data.push_back(elem);
    self.size += 1;
  }

  pub fn pop_front(&mut self) {
    if self.size <= 0 {
      panic!("Can't pop from empty deque")
    }
    self.data.pop_front();
    self.size -= 1;
  }

  pub fn pop_back(&mut self) {
    if self.size <= 0 {
      panic!("Can't pop from empty deque")
    }
    self.data.pop_back();
    self.size -= 1;
  }

  pub fn front (&mut self) -> T {
    if self.empty() {
      panic!("Can't return front of empty deque");
    }
    *self.data.front().unwrap()
  }

  pub fn back (&mut self) -> T {
    if self.empty() {
      panic!("Can't return back of empty deque");
    }
    *self.data.back().unwrap()
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn empty(&self) -> bool {
    if self.size == 0 {
      true
    } else {
      false
    }
  }
}