pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        let node = Node::new(item, self.head.take());
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = *self.head.take()?;
        self.head = node.next;
        self.len -= 1;
        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut node_option: Option<&Node<T>> = self.head.as_ref().map(|node| &**node);
        while let Some(node) = node_option {                               //    node : &Box<Node<T>>
            reversed.push(node.data.clone());                              //   *node : Box<Node<T>>
            node_option = node.next.as_ref().map(|node| &**node);          //  **node : Node<T>
        }                                                                  // &**node : &Node<T>
        reversed                                                           // *node != &**node
    }
}

impl<T: Clone> From<&[T]> for SimpleLinkedList<T> {
    fn from(array: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in array {
            list.push(item.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut node = self.head;
        while let Some(item) = node {
            vec.push(item.data);
            node = item.next;
        }
        vec.reverse();
        vec
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_is_empty() {
        let list = SimpleLinkedList::<i32>::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_increments_length() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        assert_eq!(list.len(), 1);
        list.push(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_decrements_length_and_returns_last_added() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn peek_returns_head() {
        let mut list = SimpleLinkedList::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn from_slice() {
        let array = [1, 2, 3, 4];
        let mut list = SimpleLinkedList::from(array.as_ref());
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn reverse() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list = list.rev();
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_vector() {
        let mut v = Vec::new();
        let mut s = SimpleLinkedList::new();
        for i in 1..4 {
            v.push(i);
            s.push(i);
        }
        let s_as_vec: Vec<i32> = s.into();
        assert_eq!(v, s_as_vec);
    }
}
