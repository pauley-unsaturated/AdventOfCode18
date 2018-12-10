
type Link<T> = Option<Rc<Node<T>>>;

struct CircularList<T> {
    head: Link<T>
}

impl<T> CircularList<T> {
    fn new(value: T) -> CircularList<T> {
        let mut head = Rc::new(Node{ value: value, next: None });
        Rc::get_mut(&mut head).unwrap().next = Some(head.clone());
        CircularList{ head: Some(head) }
    }
}

struct Node<T> {
    value: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn insert_after<'a, 'b>(&'b mut self, value: T) -> Link<T> {
        let new_next = Some(Rc::new(Node {value: value,
                                           next: self.next.take()
        }));
        self.next = new_next;
        // return an rc-pointer to the newly inserted value
        self.next.take().map(|node| {
            let res = node.clone();
            self.next = Some(node);
            res
        })
    }
}

// Now we need a CircularList Iterator, and we can zip around the circle
struct CircularListIter<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for CircularListIter<'a, T> {
    type Item = &'a mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> { 
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            node
        })
    }
}
