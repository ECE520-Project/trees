use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

pub trait QueryableTreeNode {
    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;

    // fn get_data(&self) -> T;

    fn height(&self) -> usize {
        let left_height = self.get_left().as_ref().map(
            |l| l.borrow().height()
        ).unwrap_or(0);
        let right_height = self.get_right().as_ref().map(
            |r| r.borrow().height()
        ).unwrap_or(0);
        max(left_height, right_height) + 1
    }

    fn count_leaves(&self) -> usize {
        match self.get_left() {
            None => match self.get_right() {
                None => 1,
                Some(r) => r.borrow().count_leaves(),
            }
            Some(l) => match self.get_right() {
                None => l.borrow().count_leaves(),
                Some(r) => {
                    l.borrow().count_leaves() + r.borrow().count_leaves()
                }
            }
        }
    }

    // fn min(&self) -> usize {
    //     let mut now = self;
    //     loop {
    //         match
    //         now = now.get_left();
    //     }
    // }
}

pub trait QueryableTree<QTN: QueryableTreeNode> {
    fn get_root(&self) -> &Option<Rc<RefCell<QTN>>>;

    fn height(&self) -> usize {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().height(),
        }
    }

    fn is_empty(&self) -> bool {
        match self.get_root() {
            None => true,
            Some(_) => false
        }
    }

    fn count_leaves(&self) -> usize {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }
}