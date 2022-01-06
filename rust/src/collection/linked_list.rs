//! ssss
//!
//!


// LinkedList 本身是个类型，当定义为变量的时候，需要给这个变量分配内存，该变量分配的内存是需要明确的，但不用管变量内存存储的是什么内容
// LinkedList 定义这个类型的变量的时候，需要给这个变量分配内容，比如32位整数类型定义变量的时候，需要分配一段32的内存

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
  head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
  element: T,
  next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList { head: None,}
  }

  pub fn push(&mut self, element: T) {
    let fresh: Box<Node<T>> = Box::new(Node {
      element: elem,
      next: self.head.take()
    });

    self.head = Some(fresh)
  }

  pub fn pop() -> Option<T> {

  }
}

impl<T> Drop for LinkedList<T> {
  drop(LinkedList)
}

[#cfg(test)]
mod tests {

  #[test]
  fn initial() {
    const linked_list = LinkedList::new();
  }

}
