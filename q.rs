
struct Q<T> {
  items : Vec<T>,
  count : uint,
  position : uint,
  limit : uint
}

impl<T> Q<T> {
  pub fn new() -> ~Q<T> {
    Q::with_size(10)
  }

  pub fn with_size(size : uint) -> ~Q<T> {
    ~Q {items: Vec::with_capacity(size),
        count: 0,
        position: 0,
        limit: size}
  }

  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn prepend(&mut self, value : T) {
    self.items.insert(self.count, value);
    self.count += 1
  }

  pub fn peek(&self) -> Option<T> {
    if self.empty() {
      None
    } else {
      Some(*self.items.get(self.position))
    }
  }

  pub fn remove(&mut self) {
    self.position += 1
  }

  pub fn size(&self) -> uint {
    self.count - self.position
  }

  pub fn full(&self) -> bool {
    self.size() == self.limit
  }
}


#[test]
fn test_empty() {
  let q = Q::<int>::new();

  assert!(q.empty() == true);
}

#[test]
fn test_prepend() {
  let mut q = Q::<int>::new();

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
fn test_prepend_with_all_the_things() {
  let mut q = Q::<int>::with_size(1);

  q.prepend(99);

  assert!(q.full() == true);
}

#[test]
fn test_prepend_with_too_many_things() {
  let mut q = Q::<int>::with_size(1);

  q.prepend(99);
  q.prepend(113);
  q.prepend(678);
}

#[test]
fn test_peek() {
  let mut q = Q::new();

  q.prepend(99);

  assert!(q.peek().unwrap() == 99);
}

#[test]
fn test_peek_with_removal() {
  let mut q = Q::new();

  q.prepend(99);
  q.remove();
  q.prepend(113);

  assert!(q.peek().unwrap() == 113);
}

#[test]
fn test_peek_on_empty_queue_returns_none() {
  let q = Q::<int>::new();

  assert!(q.peek().is_none());
}

#[test]
fn test_peek_with_a_few_things() {
  let mut q = Q::new();

  q.prepend(99);
  assert!(q.peek().unwrap() == 99);
  q.prepend(113);
  assert!(q.peek().unwrap() == 99);
  q.remove();
  assert!(q.peek().unwrap() == 113);
}
