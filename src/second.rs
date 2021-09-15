pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(Node {
            elem: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|x| {
            self.head = x.next;
            return x.elem;
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
  use super::List;
  #[test]
  fn basics() {
      let mut list = List::new();

      assert_eq!(list.pop(), None);

      list.push(1);
      list.push(2);
      list.push(3);

      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));

      list.push(4);
      list.push(5);

      assert_eq!(list.pop(), Some(5));
      assert_eq!(list.pop(), Some(4));

      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), None);

  }
}