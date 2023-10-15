extern crate simplelog;

use simplelog::*;


struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn pretty_print(&self, prefix: String, is_tail: bool, is_left_bar_needed: bool, is_right_bar_needed: bool) {
        if let Some(ref right) = self.right {
            right.pretty_print(format!("{}{}   ", prefix, if is_right_bar_needed { "│" } else { " " }), false, right.left.is_some(), false);
        }

        let node_representation = if prefix == "" { "─── " } else if is_tail { "└── " } else { "┌── " };
        info!("{}{}{}", prefix, node_representation, self.data);

        if let Some(ref left) = self.left {
            left.pretty_print(format!("{}{}   ", prefix, if is_left_bar_needed { "│" } else { " " }), true, false, left.right.is_some());
        }
    }
}