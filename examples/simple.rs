extern crate parse_tree;

use parse_tree::ParseTree;

fn main() {
    println!("{:#?}", ParseTree::from("a*b(cd|e)"));
}