use std::{cell::RefCell, fmt, rc::Rc};

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> fmt::Debug for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(next) = self.next.as_ref() {
            let next = next.borrow();
            write!(f, " ({}) ->{:?}", self.data, next)
        } else {
            write!(f, " ({})", self.data)
        }
    }
}

impl<T> Node<T> {
    fn new(
        data: T,
        next: Option<Rc<RefCell<Node<T>>>>,
        prev: Option<Rc<RefCell<Node<T>>>>,
    ) -> Self {
        Self { data, next, prev }
    }
}

pub(crate) struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> fmt::Debug for LinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(head) = &self.head {
            writeln!(f, "head: {}", head.borrow().data)?;
            writeln!(f, "{:?}", head.borrow())?;
        } else {
            writeln!(f, "head: None")?;
        }
        if let Some(tail) = &self.tail {
            write!(f, "tail: {}", tail.borrow().data)
        } else {
            write!(f, "tail: None")
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, arg: T) {
        if let Some(curr_head) = &self.head {
            let new_node = Rc::new(RefCell::new(Node::new(arg, Some(curr_head.clone()), None)));
            curr_head.borrow_mut().prev = Some(new_node.clone());
            self.head = Some(new_node);
        } else {
            assert!(self.tail.is_none());
            let new_node = Rc::new(RefCell::new(Node::new(arg, None, None)));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> T {
        if let Some(curr_head) = self.head.clone() {
            assert!(self.tail.is_some());
            let curr_tail = self.tail.as_ref().unwrap();
            if Rc::ptr_eq(&curr_head, curr_tail) {
                self.head = None;
                self.tail = None;
            } else {
                let mut next_node = curr_head.borrow().next.to_owned();
                next_node.as_mut().unwrap().borrow_mut().prev = None;
                self.head = next_node;
            }
            let popped = Rc::try_unwrap(curr_head)
                .unwrap_or_else(|_| panic!("The curr_head was shared, failed to unwrap"));
            self.len -= 1;
            popped.into_inner().data
        } else {
            panic!("Underflow")
        }
    }

    pub fn push_back(&mut self, arg: T) {
        if let Some(curr_tail) = &self.tail {
            let new_node = Rc::new(RefCell::new(Node::new(arg, None, Some(curr_tail.clone()))));
            curr_tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            assert!(self.head.is_none());
            let new_node = Rc::new(RefCell::new(Node::new(arg, None, None)));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> T {
        if let Some(curr_tail) = self.tail.clone() {
            assert!(self.head.is_some());
            let curr_head = self.head.as_ref().unwrap();
            if Rc::ptr_eq(curr_head, &curr_tail) {
                self.head = None;
                self.tail = None;
            } else {
                let mut prev_node = curr_tail.borrow().prev.to_owned();
                prev_node.as_mut().unwrap().borrow_mut().next = None;
                self.tail = prev_node;
            }
            let popped = Rc::try_unwrap(curr_tail)
                .unwrap_or_else(|_| panic!("The last_elem was shared, failed to unwrap"));
            self.len -= 1;
            popped.into_inner().data
        } else {
            panic!("Underflow")
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
