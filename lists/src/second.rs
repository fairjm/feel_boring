use std::mem::replace;
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, e: i32) {
        let node = Node {
            elem: e,
            next: replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => result = None,
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        return result;
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(boxed_node) = replace(&mut self.head, Link::Empty) {
            self.head = boxed_node.next;
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut lst = List::new();
        assert_eq!(lst.pop(), None);
        lst.push(1);
        lst.push(2);
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.pop(), None);
    }
}
