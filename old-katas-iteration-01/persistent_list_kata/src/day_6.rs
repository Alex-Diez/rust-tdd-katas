use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Link<T>
}

impl<T: Copy> List<T> {
    pub fn empty() -> Self {
        List { head: None }
    }

    pub fn single(item: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                item: item,
                next: None
            }))
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn append(&self, item: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                item: item,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn drop(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        while n > 0 {
            match *current {
                Some(ref node) => current = &node.next,
                None => break
            }
            n -= 1;
        }
        List { head: current.clone() }
    }

    pub fn drop_while<P: Fn(T) -> bool>(&self, predicate: P) -> Self {
        let mut current = &self.head;
        loop {
            match *current {
                Some(ref node) if predicate(node.item) => current = &node.next,
                _ => break
            }
        }
        List { head: current.clone() }
    }

    pub fn reverse(&self) -> Self {
        let mut current = &self.head;
        let mut ret = List::empty();
        while let Some(ref node) = *current {
            ret = ret.append(node.item);
            current = &node.next;
        }
        ret
    }

    pub fn take(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        let mut ret = List::empty();
        while n > 0 {
            match *current {
                Some(ref node) => {
                    ret = ret.append(node.item);
                    current = &node.next;
                },
                _ => break
            }
            n -= 1;
        }
        ret.reverse()
    }

    pub fn take_while<P: Fn(T) -> bool>(&self, predicate: P) -> Self {
        let mut current = &self.head;
        let mut ret = List::empty();
        loop {
            match *current {
                Some(ref node) if predicate(node.item) => {
                    current = &node.next;
                    ret = ret.append(node.item);
                },
                _ => break
            }
        }
        ret.reverse()
    }

    pub fn map<R: Copy, M: Fn(T) -> R>(&self, map: M) -> List<R> {
        let mut current = &self.head;
        let mut ret = List::empty();
        while let Some(ref node) = *current {
            ret = ret.append(map(node.item));
            current = &node.next;
        }
        ret.reverse()
    }
}

impl<T: Copy> From<Vec<T>> for List<T> {
    fn from(items: Vec<T>) -> Self {
        let mut ret = List::empty();
        for item in items {
            ret = ret.append(item);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_list() {
        let list: List<i32> = List::empty();

        assert_eq!(list.head(), None);
    }

    #[test]
    fn head_of_a_nonempty_list() {
        assert_eq!(List::from(vec![1, 2]).head(), Some(&2));
    }
    
    #[test]
    fn tail_of_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).tail(), List::from(vec![1, 2]));
    }

    #[test]
    fn drops_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).drop(2), List::single(1));
    }

    #[test]
    fn drops_from_a_list_by_predicate() {
        assert_eq!(List::from(vec![1, 2, 3]).drop_while(|item| item > 2), List::from(vec![1, 2]));
    }

    #[test]
    fn reverses_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).reverse(), List::from(vec![3, 2, 1]));
    }

    #[test]
    fn takes_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).take(2), List::from(vec![2, 3]));
    }

    #[test]
    fn takes_from_a_list_by_predicate() {
        assert_eq!(List::from(vec![1, 2, 3]).take_while(|item| item > 1), List::from(vec![2, 3]));
    }

    #[test]
    fn maps_over_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).map(|item| item * item), List::from(vec![1, 4, 9]));
    }
}