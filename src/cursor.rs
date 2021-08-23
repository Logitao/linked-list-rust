use crate::node::Node;

#[derive(Clone)]
pub struct Cursor<T>
where
    T: Copy,
{
    curr: Node<T>,
}

impl<T> Iterator for Cursor<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.curr {
            Node::None => None,
            Node::Tail { item } => {
                self.curr = Node::None;
                Some(item)
            }
            Node::Node { item, ref mut next } => {
                let mut next_item = Box::new(Node::None);

                std::mem::swap(next, &mut next_item);
                self.curr = *next_item;

                Some(item)
            }
        }
    }
}

impl<T> IntoIterator for Node<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor { curr: self }
    }
}
