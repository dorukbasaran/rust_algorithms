use std::rc::Rc;
use std::cell::RefCell;

pub struct Queue<T> {
    length: usize,
    head: Option<Rc<RefCell<QueueNode<T>>>>,
    tail: Option<Rc<RefCell<QueueNode<T>>>>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(QueueNode {
            value,
            next: None,
        }));

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.length += 1;
    }


    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek(&self) -> Option<Rc<T>> {
        self.head.as_ref().map(|head| Rc::new(head.borrow().value.clone()))
    }

}

pub struct QueueNode<T> {
    value: T,
    next: Option<Rc<RefCell<QueueNode<T>>>>,
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_new_queue() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(queue.length, 0);
        assert!(queue.head.is_none());
        assert!(queue.tail.is_none());
    }

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.length, 1);
        assert_eq!(*queue.peek().unwrap(), 1);
        queue.enqueue(2);
        assert_eq!(queue.length, 2);
        assert_eq!(*queue.peek().unwrap(), 1);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.length, 1);
        assert_eq!(*queue.peek().unwrap(), 2);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.length, 0);
        assert!(queue.peek().is_none());
        assert!(queue.dequeue().is_none());
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        assert!(queue.peek().is_none());
        queue.enqueue(1);
        assert_eq!(*queue.peek().unwrap(), 1);
        queue.enqueue(2);
        assert_eq!(*queue.peek().unwrap(), 1);
        queue.dequeue();
        assert_eq!(*queue.peek().unwrap(), 2);
    }
}


