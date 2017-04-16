// implementation of first linked list

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}


impl List {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
