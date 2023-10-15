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
        println!("{}{}{}", prefix, node_representation, self.data);

        if let Some(ref left) = self.left {
            left.pretty_print(format!("{}{}   ", prefix, if is_left_bar_needed { "│" } else { " " }), true, false, left.right.is_some());
        }
    }
}

struct BinarySearchTree {
    root: Option<Box<Node>>
}

impl BinarySearchTree {
    fn new(data: i32) -> BinarySearchTree {
        debug!("Creating BST with Root {}", data);
        let bst = BinarySearchTree {
            root: Some(
                Box::new(
                    Node {
                        data,
                        left: None,
                        right: None
                    }
                )
            )
        };
        debug!("Created BST with Root {}", data);

        bst
    }

    fn insert(&mut self, data: i32) -> bool {
        let mut curr_node = &mut self.root;

        // Iterate on the nodes
        loop {
            // Check if the node about to be come null
            match curr_node {
                Some(node_ref) => {
                    if data > node_ref.data {
                        if node_ref.right.is_none() {
                            // Don't really need a reference but gotta take it lmao
                            let _ = node_ref.right.insert(Box::new(Node { data, left: None, right: None }));
                            debug!("Inserted data {} into BST list with Root {}", data, self.root.as_ref().unwrap().data);
                            break true
                        }
                        curr_node = &mut node_ref.right
                    } else if data == node_ref.data {
                        // Data duplicate, ignore the data, return false as failure
                        warn!("Failed to insert data {} into BST list with Root {}", data, self.root.as_ref().unwrap().data);
                        break false
                    } else {
                        if node_ref.left.is_none() {
                            // Don't really need a reference but gotta take it lmao
                            let _ = node_ref.left.insert(Box::new(Node { data, left: None, right: None }));
                            debug!("Inserted data {} into BST list with Root {}", data, self.root.as_ref().unwrap().data);
                            break true
                        }
                        curr_node = &mut node_ref.left
                    }

                },
                None => break false
            }
        }
    }

    fn delete(&mut self, data:i32) -> bool {
        let mut curr_node = &mut self.root;

        // Iterator on the nodes
        while let Some(node_ref) = curr_node {
            if data == node_ref.data {
                break;
            } else if data > node_ref.data {
                curr_node = &mut curr_node.as_mut().unwrap().right;
            } else {
                curr_node = &mut curr_node.as_mut().unwrap().left;
            }
        }

        if curr_node.is_none() {
            warn!("Failed to delete data {} from BST list with Root {}", data, self.root.as_ref().unwrap().data);
            return false;
        }

        if curr_node.as_ref().unwrap().left.is_none() && curr_node.as_ref().unwrap().right.is_none() {
            // Delete a leaf node
            *curr_node = None;
        } else if curr_node.as_ref().unwrap().left.is_none() && curr_node.as_ref().unwrap().right.is_some() {
            // Delete a node that has no left sub-child tree
            *curr_node = curr_node.as_mut().unwrap().right.take();
        } else if curr_node.as_ref().unwrap().right.is_none() && curr_node.as_ref().unwrap().left.is_some() {
            // Delete a node that has no right sub-child tree
            *curr_node = curr_node.as_mut().unwrap().left.take();
        } else {
            // Delete a node that has two sub-child trees
            let left_node = curr_node.as_mut().unwrap().left.take().unwrap();
            let mut min_node = &mut curr_node.as_mut().unwrap().right;

            // Find minimum node in the right sub tree
            while min_node.as_ref().unwrap().left.is_some() {
                min_node = &mut min_node.as_mut().unwrap().left;
            }

            // Attach left sub-tree of deleting node to min node of right subtree
            let _ = min_node.as_mut().unwrap().left.insert(left_node);

            // Delete a node that has no left sub-child tree
            *curr_node = curr_node.as_mut().unwrap().right.take();
        }
        debug!("Deleted data {} from BST list with Root {}", data, self.root.as_ref().unwrap().data);
        true
    }

    fn search(&mut self, data:i32) -> bool {
        let mut curr_node = &mut self.root;

        // Iterate on the nodes
        loop {
            match curr_node {
                Some(node_ref) => {
                    if data == node_ref.data {
                        break true
                    } else if data > node_ref.data {
                        curr_node = &mut node_ref.right;
                    } else {
                        curr_node = &mut node_ref.left;
                    }
                }
                None => break false
            }
        }
    }

    // Print formatted
    fn pretty_print(&self) {
        debug!("Starting to pretty print BST with Root {}", self.root.as_ref().unwrap().data);
        println!("{:-^50}", "");
        println!("{:^50}", "BST List");
        println!("{:-^50}", "");
        self.root.as_ref().unwrap().pretty_print("".to_string(), false, false, false);
        println!("{:-^50}", "");
        debug!("Done pretty printing BST with Root {}", self.root.as_ref().unwrap().data);
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<String>().unwrap();
}


fn main() {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();
    debug!("Start of program");

    let mut line : String;

    println!("Enter root for BST:");
    line = read_line();
    let mut bst = BinarySearchTree::new(line.parse::<i32>().unwrap());

    loop {
        println!("Select an option:");
        println!("    1. Insert a node");
        println!("    2. Delete a node");
        println!("    3. Search for a node");
        println!("    4. Print BST");
        println!("    5. Exit");

        line = read_line();

        match line.as_str() {
            "1" => {
                println!("Enter node data:");
                line = read_line();
                bst.insert(line.parse::<i32>().unwrap_or(0));
            }
            "2" => {
                println!("Enter node data:");
                line = read_line();
                bst.delete(line.parse::<i32>().unwrap_or(0));
            }
            "3" => {
                println!("Enter node data:");
                line = read_line();
                if bst.search(line.parse::<i32>().unwrap_or(0)) {
                    println!("{} found!", line);
                } else {
                    println!("{} not found!", line);
                }
            }
            "4" => {
                bst.pretty_print();
            }
            "5" => { break }
            _ => { continue }
        }
    }
    debug!("End of program");
}