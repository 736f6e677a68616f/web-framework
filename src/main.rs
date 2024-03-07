use std::{cell::RefCell, collections::VecDeque, rc::Rc, result};
fn main() {
    
}
fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, lower: i32, upper: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val <= lower || node.val >= upper {
                return false;
            } else {
                helper(node.left.clone(), lower, node.val) && helper(node.right.clone(), node.val, upper)
            }
        } else {
            true
        }
    }
    helper(root, i32::MIN - 1, i32::MAX + 1)
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}