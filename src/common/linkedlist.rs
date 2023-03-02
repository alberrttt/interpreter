use std::ptr;

pub struct LinkedList<T> {
    pub head: *mut LinkedListNode<T>,
    pub tail: *mut LinkedListNode<T>,
    pub len: usize,
}
pub struct LinkedListNode<T> {
    pub next: *mut LinkedListNode<T>,
    pub value: T,
}
#[allow(unsafe_code)]
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        if self.len == 0 {
            let node = Box::new(LinkedListNode {
                next: self.head,
                value,
            });
            let ptr = Box::into_raw(node);
            self.head = ptr;
            self.tail = ptr;
            self.len += 1;
        } else {
            let node = Box::new(LinkedListNode {
                next: ptr::null_mut(),
                value,
            });
            let ptr = Box::into_raw(node);
            unsafe {
                (*self.tail).next = ptr;
            }
            self.tail = ptr;
            self.len += 1;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let node = unsafe { Box::from_raw(self.head) };
            self.head = node.next;
            Some(node.value)
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
