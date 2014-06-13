
struct Q {
  items : Vec<int>,
  count : uint,
  position : uint,
  limit : uint
}

impl Q {
  pub fn new() -> Box<Q> {
    Q::with_size(10)
  }

  pub fn with_size(size : uint) -> Box<Q> {
    box Q {items: Vec::with_capacity(size),
           count: 0,
           position: 0,
           limit : size}
  }

  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn enqueue(&mut self, value : int) {
    if self.size() == self.limit {
      fail!("Queue is limited to {:u} items", self.limit)
    }
    self.items.insert(self.count, value);
    self.count += 1
  }

  pub fn peek(&self) -> int {
    *self.items.get(self.position)
  }

  pub fn dequeue(&mut self) {
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
fn test_enqueue() {
  let mut q = Q::new();

  q.enqueue(99);

  assert!(q.empty() == false);
  assert!(q.size() == 1);
}

#[test]
fn test_enqueue_with_two_insertions() {
  let mut q = Q::new();

  q.enqueue(99);
  q.enqueue(113);

  assert!(q.empty() == false);
  assert!(q.size() == 2);
}

#[test]
#[should_fail]
fn test_enqueue_with_too_many_things() {
  let mut q = Q::with_size(1);

  q.enqueue(99);
  q.enqueue(113);
  q.enqueue(678);
}

#[test]
fn test_peek() {
  let mut q = Q::new();

  q.enqueue(99);

  assert!(q.peek() == 99);
}

#[test]
fn test_peek_with_removal() {
  let mut q = Q::new();

  q.enqueue(99);
  q.dequeue();
  q.enqueue(113);

  assert!(q.peek() == 113);
}

#[test]
fn test_peek_with_a_few_things() {
  let mut q = Q::new();

  q.enqueue(99);
  assert!(q.peek() == 99);
  q.enqueue(113);
  assert!(q.peek() == 99);
  q.dequeue();
  assert!(q.peek() == 113);
}
