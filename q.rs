
struct Q {
  items : Vec<int>,
  count : uint,
  position : uint
}

impl Q {
  pub fn new() -> ~Q {
    Q::with_size(10)
  }

  pub fn with_size(size : uint) -> ~Q {
    ~Q {items: Vec::with_capacity(size),
        count: 0,
        position: 0}
  }

  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn prepend(&mut self, value : int) {
    self.items.insert(self.count, value);
    self.count += 1
  }

  pub fn peek(&self) -> int {
    *self.items.get(self.position)
  }

  pub fn remove(&mut self) {
    self.position += 1
  }

  pub fn size(&self) -> uint {
    self.count - self.position
  }
}

#[test]
fn test_empty() {
  let q = Q::new();

  assert!(q.empty() == true);
}

#[test]
fn test_prepend() {
  let mut q = Q::new();

  q.prepend(99);

  assert!(q.empty() == false);
  assert!(q.size() == 1);
}

#[test]
fn test_prepend_with_two_insertions() {
  let mut q = Q::new();

  q.prepend(99);
  q.prepend(113);

  assert!(q.empty() == false);
  assert!(q.size() == 2);
}

#[test]
fn test_prepend_with_too_many_things() {
  let mut q = Q::with_size(1);

  q.prepend(99);
  q.prepend(113);
  q.prepend(678);
}

#[test]
fn test_peek() {
  let mut q = Q::new();

  q.prepend(99);

  assert!(q.peek() == 99);
}

#[test]
fn test_peek_with_removal() {
  let mut q = Q::new();

  q.prepend(99);
  q.remove();
  q.prepend(113);

  assert!(q.peek() == 113);
}

#[test]
fn test_peek_with_a_few_things() {
  let mut q = Q::new();

  q.prepend(99);
  assert!(q.peek() == 99);
  q.prepend(113);
  assert!(q.peek() == 99);
  q.remove();
  assert!(q.peek() == 113);
}
