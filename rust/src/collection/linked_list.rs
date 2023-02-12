//! Linked list
//!


type Link<T> = Option<Box<Node<T>>>

struct Node<T> {
  element: T,
  next: Link<T>
}

pub struct LinkedList<T> {
  head: Link<T>
  len: usize
}

impl<T> LinkedList<T> {

  pub fn new() -> Self {
    LinkedList { head: None }
  }

  pub fn push(&mut self, element: T) -> Self {
    self.head.take()
  }

  pub fn pop(&mut self) -> Option<T> {

  }

  pub fn remove(&mut self, index: usize) ->Option<T> {

  }

  pub fn find(&mut self, callback:Fn) -> Option<T> {

  }

}

#[cfg(test)]
mod tests {

    #[test]
    fn initial() {
        let list = LinkedList::<String>::new();

        list.push(String::from("hello world!"));

        let last_element = list.pop();
        // TODO:增加Debug实现
        println!("{}", last_element);
        
        fn callback(&element: String, index) {
          if (element.value === 0) {
            index
          }
        }

        let index = list.findIndex(callback);
        let element = list.find(callback);
        // TODO: some/none
        match element {
          None => println!("Nothing"),
          Some => println!("{}", element.value),
        }

        list.insert(5,String::from("Good Luck!"));

        // TODO: 超出边界需要提示错误
        let result = list.remove(3).unwrap();
        println!("{}", result);

        list.map(|x| x.value).unwrap();
    }
}
