#[derive(Clone)]
/// A Linked Node, containing three possible options:
///     - Another `Node` containing three possible options
///     - A `Tail` Node containing only one item
///     - Nothing (`Option<T>`)
///
/// # Types
///     - Any type that implements the ```Copy``` trait
///
/// # Examples
///
///  ```
///  //Creates a new Node<T>
///  let mut list_a: Node<i32> = Node::new();
///  ```
///
pub enum Node<T>
where
    T: Copy,
{
    None,
    Tail { item: T },
    Node { item: T, next: Box<Node<T>> },
}

impl<T> Node<T>
where
    T: Copy,
{
    /// Creates an empty Node
    ///
    /// # Examples
    ///  ```
    ///    let mut list_a: Node<i32> = Node::new();
    ///  ```
    ///
    pub fn new() -> Self {
        Self::None
    }

    /// Converts a Linked Node into `Tail` variant.
    /// If it is:
    ///     - A `Tail` Node, it panics.
    ///     - Another `Node`, it removes the next item and converts it to `Tail` Node
    ///     - Nothing, just converts it to `Tail` Node
    ///
    /// # Arguments
    ///     - The current node item, that would be present in the new `Tail`
    ///
    fn tail(&mut self, new: T) {
        *self = match self {
            Self::None => Self::Tail { item: new },
            Self::Node { item: _, next: _ } => Self::Tail { item: new },
            _ => panic!("Could not convert to tail"),
        };
    }

    /// Converts a Linked Node into `Node` variant.
    /// If it is:
    ///     - A `Tail` Node, it panics.
    ///     - Another `Node`, it removes the next item and converts it to `Tail` Node
    ///     - Nothing, just converts it to `Tail` Node
    ///
    /// # Arguments
    ///     - The current node item, that would be present in the new `Tail`
    ///
    fn link(&mut self, x: T) {
        *self = match self {
            Self::Tail { item } => {
                let tail = Self::Tail { item: x };

                Self::Node {
                    item: *item,
                    next: Box::new(tail),
                }
            }
            _ => panic!("Could not convert to link"),
        }
    }

    fn none(&mut self) {
        *self = std::mem::replace(self, Node::None)
    }

    fn next(&mut self, next: Node<T>) {
        *self = next
    }

    pub fn push(&mut self, x: T) {
        match self {
            Self::None => self.tail(x),
            Self::Tail { .. } => self.link(x),
            Self::Node { next, .. } => next.push(x),
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { item } => {
                let item = *item;
                self.none();

                Some(item)
            }
            Self::Node { item, next } => {
                let mut next_item = Box::new(Self::None);
                let item = *item;

                std::mem::swap(next, &mut next_item);

                self.next(*next_item);

                Some(item)
            }
        }
    }
}
