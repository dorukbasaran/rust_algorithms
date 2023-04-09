// new - to create a new empty linked list
// is_empty - to check if the list is empty
// len - to get the length of the list
// push_front - to add an element to the front of the list
// push_back - to add an element to the back of the list
// pop_front - to remove and return the first element of the list
// pop_back - to remove and return the last element of the list
// delete - to remove an element at a specified index
// insert - to insert an element at a specified index

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            count += 1;
            current = &node.next;
        }
        count
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        if let Some(mut tail) = self.head.as_mut() {
            while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
            tail.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &self.head {
            None => None,
            Some(_) => {
                let mut current = &mut self.head;
                while current.as_ref().unwrap().next.is_some() {
                    current = &mut current.as_mut().unwrap().next;
                }
                current.take().map(|node| node.value)
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut()?.next;
        }
        current.take().map(|node| {
            *current = node.next;
            node.value
        })
    }

    pub fn insert(&mut self, index: usize, value: T) {
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node {
            value,
            next: current.take(),
        });
        *current = Some(new_node);
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn push_front_and_len() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn push_back_and_len() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn insert() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
        list.insert(1, 3);
        list.insert(1, 2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn delete() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.delete(1), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }
}
