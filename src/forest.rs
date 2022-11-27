use std::fmt;
use std::ops::{Index, IndexMut};

/// Represents status of a nucleotide;
/// its position or optionally a pair if its got one.
/// pos: None represents beginning of a new loop
#[derive(Debug, Default, PartialEq, Eq)]
pub struct DotBracket {
    pub pos: Option<usize>,
    pub pair: Option<usize>,
}

impl DotBracket {
    pub fn new(pos: Option<usize>, pair: Option<usize>) -> Self {
        Self { pos, pair }
    }

    /// creates a new DotBracket but allows
    /// to pass raw usizes and then wraps
    /// them into Some(usize) under the hood;
    /// basically conveninent version of new()
    pub fn newsome(pos: usize, pair: usize) -> Self {
        Self {
            pos: Some(pos),
            pair: Some(pair),
        }
    }

    /// Creates DotBracket with pos set to None
    /// symbolizing beginning of a loop structure
    pub fn new_loop() -> Self {
        Self::new(None, None)
    }
}

/// simple node struct containing its children and a value.
/// Children are meant to be usizes corresponding to indexes of
/// the tree's arena vector which in turn grant access to actual node
#[derive(Debug)]
pub struct Node<T> {
    #[allow(dead_code)]
    idx: usize,
    pub val: T,
    pub children: Vec<usize>,
}

impl<T> Node<T> {
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            children: vec![],
        }
    }
}

impl<T> Node<T> {
    pub fn push(&mut self, val: usize) {
        self.children.push(val);
    }
}

/// Tree Iterator adapter
#[derive(Debug)]
pub struct ChickenOfTheWoods<'a, T> {
    // idx: usize,
    deck: Vec<usize>,
    tree: &'a Tree<T>,
}

impl<'a, T> ChickenOfTheWoods<'a, T> {
    fn new(tree: &'a Tree<T>) -> Self {
        Self {
            deck: vec![0],
            tree,
        }
    }
}

impl<'a, T> Iterator for ChickenOfTheWoods<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.deck.pop() {
            for kid in self.tree[idx].children.iter().rev() {
                self.deck.push(*kid);
            }
            Some(idx)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    arena: Vec<Node<T>>,
}

impl<T> Index<usize> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arena[index]
    }
}

impl<T> IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arena[index]
    }
}

/// Memory arena tree data structure
/// Nodes refer to other nodes by the index
/// of said arena instead of holding direct
/// pointers to each other
impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self { arena: Vec::new() }
    }
}

impl<T> Tree<T> {
    /// creates new node
    /// dumb name alert
    pub fn sprout(&mut self, val: T) -> usize {
        let idx = self.size();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn iter(&self) -> ChickenOfTheWoods<T> {
        ChickenOfTheWoods::new(self)
    }
}

impl<T> Tree<T>
where
    T: fmt::Debug,
{
    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    pub fn full_print(&self) {
        for i in self.iter() {
            println!("{:?}", self[i]);
        }
    }
}

fn stem_walk(
    mut tree: Tree<DotBracket>,
    pair_list: &Vec<Option<usize>>,
    pos: usize,
    tail: usize,
) -> (Tree<DotBracket>, usize) {
    if pair_list[pos] == Some(tail) {
        let node_ix = tree.sprout(DotBracket::newsome(pos, tail));
        let (mut tree, ix) = stem_walk(tree, pair_list, pos + 1, tail - 1);
        tree[node_ix].push(ix);
        (tree, node_ix)
    } else {
        let node_ix = tree.sprout(DotBracket::new_loop());
        rna_walk(tree, pair_list, node_ix, pos, tail)
    }
}

fn rna_walk(
    mut tree: Tree<DotBracket>,
    pair_list: &Vec<Option<usize>>,
    root_ix: usize,
    pos: usize,
    tail: usize,
) -> (Tree<DotBracket>, usize) {
    let mut pos = pos;
    while pos <= tail {
        if let Some(x) = pair_list[pos] {
            let node_ix: usize;
            (tree, node_ix) = stem_walk(tree, pair_list, pos, x);
            tree[root_ix].push(node_ix);
            pos = x + 1
        } else {
            let node_ix = tree.sprout(DotBracket::new(Some(pos), None));
            tree[root_ix].push(node_ix);
            pos += 1;
        }
    }
    (tree, root_ix)
}

pub fn grow_tree(pair_list: &Vec<Option<usize>>) -> Tree<DotBracket> {
    let mut tree = Tree::default();
    let root_ix = tree.sprout(DotBracket::new_loop());
    (tree, _) = rna_walk(tree, pair_list, root_ix, 0, pair_list.len() - 1);
    tree
}
