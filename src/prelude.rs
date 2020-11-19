//! Convenience re-export of common members
//!
//! Like the standard library's prelude, this module simplifies importing of
//! common items. Unlike the standard prelude, the contents of this module must
//! be imported manually:
//!
//! ```
//! use trees::prelude::*;
//! ```

pub use crate::avltree::AVLTree;
pub use crate::bstree::BinarySearchTree;
pub use crate::rbtree::RedBlackTree;
pub use crate::base::QueryableTree;
