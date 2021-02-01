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

/*
其实这是一个栈
 */
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

//*******************************drop的显示实现*****************

impl Drop for List {
    fn drop(&mut self) {
        // 注意：在实际Rust代码中你不能显式调用`drop`，
        // 我们假装自己是编译器！
        list.head.drop(); // 尾递归——好！
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        /*match list.head {
            Link::Empty => {} // 完成！
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // 尾递归——好！
            }
        }*/
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == “在这个模式不匹配之前持续循环”
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node在这里退出作用域然后被丢弃；
            // 但是其节点的`next`字段被设置为 Link::Empty
            // 所以没有多层递归产生。
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // 糟糕，不是尾递归！
        deallocate(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // 检查空列表行为正确
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // 推入更多元素来确认没有问题
        list.push(4);
        list.push(5);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // 检查完全移除
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}