//bad idea
/*pub enum List {
    Empty,
    Elem(i32, Box<List>),
}*/

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            //The only thing you can't do with an &mut is move the value out with no replacement
            //这样会导致head的所有权发生变化，head指针的生命周期结束
            //next:self.head,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[test]
fn test() {
    println!("hello");
}