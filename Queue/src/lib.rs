// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

#[derive(Default)]
pub struct Queue<T> {
  size: usize,
  data: std::collections::LinkedList<T>,
}

impl<T> Queue<T>
where T: Copy + Clone {
  
  pub fn new() -> Self {
    Self {
      size: 0,
      data: std::collections::LinkedList::<T>::new(),
    }
  }

  pub fn push (&mut self, elem: T) {
    self.data.push_back(elem);
    self.size += 1;
  }

  pub fn pop (&mut self) {
    self.data.pop_front();
    self.size -= 1;
  }

  pub fn top (&mut self) -> T {
    if self.empty() {
      panic!("Can't return top of empty stack");
    }
    *self.data.front().unwrap()
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