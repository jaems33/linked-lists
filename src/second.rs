pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            elem: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            return x.elem;
        })
    }

    pub fn peek(&self) -> Option<&T> {
      self.head.as_ref().map(|x|{
        &x.elem
      })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
      self.head.as_mut().map(|x| {
        &mut x.elem
      })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
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

  #[test]
  fn try_peek() {
      let mut list = List::new();      

      list.push(1);
      assert_eq!(list.peek(), Some(&1));

      assert_eq!(list.peek_mut(), Some(&mut 1));
      assert_eq!(list.peek_mut(), Some(&mut 1));

      list.peek_mut().map(|n| {
        *n = 42;
      });
      assert_eq!(list.peek_mut(), Some(&mut 42));
      assert_eq!(list.pop(), Some(42));
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }
}
