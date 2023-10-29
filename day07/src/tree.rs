use crate::model::{CdTarget, Command, DirEntry, Inode};
use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};

pub fn generate_tree_from_commands<'a, 'b>(commands: &'b [Command<'a>]) -> Tree<Inode<'a>> {
    let mut tree: Tree<Inode> = TreeBuilder::new().build();
    let root_node = tree
        .insert(
            Node::new(Inode::Dir(DirEntry { name: "/", size: 0 })),
            AsRoot,
        )
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
                        if let &Inode::Dir(DirEntry { name, size: 0 }) = node.data() {
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

    compute_tree_dir_size(tree)
}

pub fn pretty_print_tree(tree: &Tree<Inode>) {
    let root = tree.root_node_id().unwrap();
    pretty_print_tree_inner(tree, root, String::new());
}

fn pretty_print_tree_inner(tree: &Tree<Inode>, current_node: &NodeId, mut offset: String) {
    let node_data = tree.get(current_node).unwrap().data();
    match node_data {
        Inode::Dir(dir_entry) => {
            println!(
                "{}- {} (dir, size={})",
                &offset, dir_entry.name, dir_entry.size
            );
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

fn compute_tree_dir_size(mut tree: Tree<Inode>) -> Tree<Inode> {
    let root = tree.root_node_id().unwrap().clone();
    compute_tree_dir_size_inner(&mut tree, &root);

    tree
}

fn compute_tree_dir_size_inner(tree: &mut Tree<Inode>, current_node: &NodeId) -> i64 {
    let inode = tree.get(current_node).unwrap().data().clone();
    match inode {
        Inode::Dir(dir_entry) => {
            let mut size: i64 = 0;
            let childrens_id: Vec<_> = tree.children_ids(current_node).unwrap().cloned().collect();
            for children_id in &childrens_id {
                size += compute_tree_dir_size_inner(tree, children_id)
            }
            let mut_inode = tree.get_mut(current_node).unwrap().data_mut();
            *mut_inode = Inode::Dir(DirEntry {
                name: dir_entry.name,
                size,
            });

            size
        }
        Inode::File(file_entry) => file_entry.size,
    }
}
