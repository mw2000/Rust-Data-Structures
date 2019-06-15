// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

#[derive(Default)]
pub struct Steque<T> {
  size: usize,
  data: std::collections::LinkedList<T>,
}

impl<T> Steque<T>
where T: Copy + Clone {
  
  pub fn new() -> Self {
    Self {
      size: 0,
      data: std::collections::LinkedList::<T>::new(),
    }
  }

  pub fn push(&mut self, elem: T) {
    self.data.push_front(elem);
    self.size += 1;
  }

  pub fn enqueue(&mut self, elem: T) {
    self.data.push_back(elem);
    self.size += 1;
  }

  pub fn pop(&mut self) {
    if self.size <= 0 {
      panic!("Can't pop from empty steque")
    }
    self.data.pop_front();
    self.size -= 1;
  }

  pub fn front(&mut self) -> T {
    if self.empty() {
      panic!("Can't return front of empty steque");
    }
    *self.data.front().unwrap()
  }

  pub fn back(&mut self) -> T {
    if self.empty() {
      panic!("Can't return back of empty steque");
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