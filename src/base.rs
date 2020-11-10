use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use std::fmt;

pub trait QueryableTreeNode<T: Ord + Copy + fmt::Debug> {
    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_data(&self) -> T;

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

    fn print_inorder(&self) {
        if let Some(l) = self.get_left() {
            l.borrow().print_inorder();
        }
        print!("{:?} ", self.get_data());
        if let Some(r) = self.get_right() {
            r.borrow().print_inorder();
        }
    }

    fn min(&self) -> T {
        self.get_left().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().min()
        )
    }

    fn max(&self) -> T {
        self.get_right().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().max()
        )
    }

    fn contains(&self, value: T) -> bool {
        if self.get_data() == value {
            return true;
        }
        let left_found = self.get_left().as_ref().map(
            |l| l.borrow().contains(value)
        ).unwrap_or(false);
        let right_found = self.get_right().as_ref().map(
            |l| l.borrow().contains(value)
        ).unwrap_or(false);
        return left_found || right_found
    }
}

pub trait QueryableTree<T: Ord + Copy + fmt::Debug, QTN: QueryableTreeNode<T>> {
    fn get_root(&self) -> &Option<Rc<RefCell<QTN>>>;

    fn count_leaves(&self) -> usize {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    fn height(&self) -> usize {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().height(),
        }
    }

    fn print_inorder(&self) {
        match &self.get_root() {
            None => println!("It is an empty tree!"),
            Some(node) => {
                node.borrow().print_inorder();
                println!();
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self.get_root() {
            None => true,
            Some(_) => false
        }
    }

    fn min(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().min()),
        }
    }

    fn max(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().max()),
        }
    }

    fn contains(&self, value: T) -> bool {
        match self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }
}
