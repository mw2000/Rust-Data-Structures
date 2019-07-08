use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::LinkedList;

pub struct HashTable<T> {
  data: Vec<Vec<T>>,
  size: usize,
  num_keys: usize,
}


impl<T> HashTable<T>
where T: Hash + std::fmt::Display + Clone + Copy + PartialEq {
  fn temp<'b>(&self, backup: &mut Vec<T>) {
    for i in 0..self.size {
      for e in &self.data[i] {
        let backup_len = backup.len();
        backup.insert(backup_len, *e);
      }
    }
  }


  fn resize(&mut self) {
    let mut backup = Vec::new();
    self.temp(&mut backup);

    let mut new_data = Vec::new();
    new_data.resize(self.size*2, Vec::<T>::new());

    self.data = new_data;
    self.num_keys = 0;
    self.size *= 2;

    for e in &backup {
      self.insert(*e);
    }
  }

  fn down_resize(&mut self) {
    let mut backup = Vec::new();
    self.temp(&mut backup);

    let mut new_data = Vec::new();
    new_data.resize((self.size / 2) + 1, Vec::<T>::new());

    self.data = new_data;
    self.num_keys = 0;
    self.size /= 2;
    self.size += 1;

    for e in &backup {
      self.insert(*e);
    }
  }

  pub fn new() -> Self {
    Self {
      data: {
        let mut v = Vec::with_capacity(7);
        for i in 0..7 {
          v.push(Vec::<T>::new());
        }
        v
      },
      size: 7,
      num_keys: 0
    }
  }

  pub fn calculate_hash<H: Hash>(&self, t: &H) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() % self.size as u64
  }

  pub fn insert(&mut self, elem: T) {
    let hash = self.calculate_hash(&elem);
    self.data[hash as usize].push(elem);
    self.num_keys += 1;

    if (self.num_keys > self.size) {
      self.resize();
    }
  }

  pub fn remove(&mut self, elem: T) {
    let hash = self.calculate_hash(&elem);

    for i in 0..self.data[hash as usize].len() {
      if self.data[hash as usize][i as usize] == elem {
        self.data[hash as usize].remove(i);
      }
    }
    self.num_keys -= 1;

    if (self.num_keys <= self.size / 2) {
      self.down_resize();
    }
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn contains(&self) -> bool {
    true
  }

  pub fn print(&self) {
    for i in 0..self.size {
      print!("[{}]: ", i);
      for e in &self.data[i as usize] {
        print!("{}, ", e);
      }
      println!("");
    }
  }
}

fn main() {
  let mut hashtab = HashTable::<i32>::new();

  for i in 0..15 {
    hashtab.insert(i);
  }
  for j in 0..3{
    hashtab.remove(j);
  }
  hashtab.print();

}


