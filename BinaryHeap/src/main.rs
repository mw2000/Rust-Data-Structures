use std::mem;

pub struct BinaryHeap<T> {
  data: Vec<T>,
  size: usize,
}

impl<T> BinaryHeap<T>
where T: Copy + Clone + PartialOrd {
  pub fn new() -> Self {
    Self {
      data: Vec::<T>::new(),
      size: 0,
    }
  }
  
  fn root(&self) -> usize {
    0
  }

  fn parent(&self, n: usize) -> usize {
    (n - 1)/2
  }

  fn leftChild(&self, n: usize) -> usize {
    (n * 2) + 1
  }

  fn rightChild(&self, n: usize) -> usize {
    (n * 2) + 2
  }

  fn hasParent(&self, n: usize) -> bool {
    n != self.root()
  }

  fn isNode(&self, n: usize) -> bool {
    n < self.size
  }

  fn compareNodes(&self, i: usize, j: usize) -> bool {
    self.data[i] < self.data[j]
  }

  fn percolateUp(&mut self, mut n: usize) {
    while (self.hasParent(n) && self.compareNodes(n, self.parent(n))) {
      let parent = self.parent(n);
      self.data.swap(n, parent);
      n = parent;
    }
  }

  fn percolateDown(&mut self, mut n: usize) {
    while (self.isNode(self.leftChild(n))) {
      let mut child = self.leftChild(n);

      if self.isNode(self.rightChild(n)) && self.compareNodes(n, self.parent(n)) {
        child = self.rightChild(n)
      }
      if self.compareNodes(child, n) {
        self.data.swap(child, n);
      } else {
        break;
      }
      n = child
    }
  }
}

