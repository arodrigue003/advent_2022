use crate::model::{CdTarget, Command, DirEntry, Inode};
use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};

pub fn generate_tree_from_commands<'a, 'b>(commands: &'b [Command<'a>]) -> Tree<Inode<'a>> {
    let mut tree: Tree<Inode> = TreeBuilder::new().build();
    let root_node = tree
        .insert(Node::new(Inode::Dir(DirEntry { name: "/" })), AsRoot)
        .unwrap();

    let mut current_node = root_node.clone();
    for command in commands {
        match command {
            Command::Ls(ls_lines) => {
                for ls_line in ls_lines {
                    tree.insert(Node::new(ls_line.clone()), UnderNode(&current_node))
                        .unwrap();
                }
            }
            Command::Cd(cd_target) => match cd_target {
                CdTarget::Root => {
                    current_node = root_node.clone();
                }
                CdTarget::Up => {
                    current_node = tree.get(&current_node).unwrap().parent().unwrap().clone()
                }
                CdTarget::Directory(dir_name) => {
                    for children in tree.children_ids(&current_node).unwrap() {
                        let node = tree.get(children).unwrap();
                        if let &Inode::Dir(DirEntry { name }) = node.data() {
                            if name == *dir_name {
                                current_node = children.clone();
                                break;
                            }
                        }
                    }
                }
            },
        }
    }

    tree
}

pub fn pretty_print_tree(tree: &Tree<Inode>) {
    let root = tree.root_node_id().unwrap();
    pretty_print_tree_inner(tree, root, String::new());
}

fn pretty_print_tree_inner(tree: &Tree<Inode>, current_node: &NodeId, mut offset: String) {
    let node_data = tree.get(current_node).unwrap().data();
    match node_data {
        Inode::Dir(dir_entry) => {
            println!("{}- {} (dir)", &offset, dir_entry.name);
        }
        Inode::File(file_entry) => {
            println!(
                "{}- {} (file, size={})",
                &offset, file_entry.name, file_entry.size
            );
        }
    }

    offset.push_str("  ");

    for children_id in tree.children_ids(current_node).unwrap() {
        pretty_print_tree_inner(tree, children_id, offset.clone())
    }
}
