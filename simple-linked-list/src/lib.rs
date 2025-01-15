use std::iter::FromIterator;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    len: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        // Note: we can use `std::mem::take` to take ownership from `self.head`.
        // see https://doc.rust-lang.org/std/mem/fn.take.html
        // see https://doc.rust-lang.org/error-index.html#E0507
        //
        // And we can also use `Option.take()` instead of `std::mem::take`.
        // see https://exercism.io/tracks/rust/exercises/simple-linked-list/solutions/d2fef20701d24a2b8d88841b44d74415
        // see https://doc.rust-lang.org/std/option/enum.Option.html#method.take
        self.len += 1;
        self.head = Some(Box::new(Node {
            data: element,
            next: std::mem::take(&mut self.head),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = std::mem::take(&mut self.head);
        match head {
            Some(mut x) => {
                self.head = std::mem::take(&mut x.next);
                self.len -= 1;
                Some(x.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            list.push(x);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for x in iter {
            list.push(x);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = self.rev();
        let mut vec = Vec::new();
        while let Some(x) = list.pop() {
            vec.push(x)
        }
        vec
    }
}
