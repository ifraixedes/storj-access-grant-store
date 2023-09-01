//!  Types for finding access grants inside AGS content.

use std::cell::RefCell;
use std::rc::Rc;

use regex::Regex;

/// Filters access grants according to its containing predicates.
/// For building a filter use a [`Builder`] instance.
#[derive(Debug)]
pub struct Filter {
    root: Node,
}

impl Filter {}

/// Represents a node of the tree representation of a filter.
#[derive(Debug)]
enum Node {
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
    Exp { field: String, value: Regex },
}

/// Builds a filter with the indicated predicates.
pub struct Builder {
    /// points to the root node of the tree. It's only `None` when filter is create (`new`).
    root: Option<Rc<RefCell<BuilderNode>>>,
    /// points to the last non-leaf node added to the tree. It's `None` when filter is created
    /// (`new`) or only has one node.
    pointer: Option<Rc<RefCell<BuilderNode>>>,
}

impl Builder {
    /// Creates a builder for building a [`Filter`].
    pub fn new() -> Self {
        Self {
            root: None,
            pointer: None,
        }
    }

    // TODO: Implement it in the future.
    // Nests a filter into the current position of the filter that this instances is building.
    // This should be as a parentheses expression, for example: A OR (B AND C).
    fn _nest(self, filter: Filter) -> Connector {
        todo!();
    }

    /// Appends a regular expression predicate to the current position of the filter that this
    /// instances is building.
    ///
    /// `field` is the access grant field's name to match `exp`.
    /// `exp` is the expression to compile a regular expression. If `exp` isn't valid regular
    /// expression, [`Connector::end`] will return an error.
    ///
    /// This method doesn't return an error and delays to returned if there is any, to make the
    /// building syntax (chaining of methods calls) succinct.
    pub fn regex<F>(mut self, field: F, exp: Regex) -> Connector
    where
        F: ToString,
    {
        if let Some(ref pointer) = self.pointer {
            // TODO: I don't understand why I have to call `as_ref()`, otherwise, it doesn't
            // compile while the following code works
            // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=009a905385dc51c0d172fca9cecdade5
            let mut node = pointer.as_ref().borrow_mut();
            node.set_expression_as_right_child(field, exp);
        } else {
            self.root = Some(Rc::new(RefCell::new(BuilderNode::new_leaf(field, exp))));
        }

        Connector { builder: self }
    }

    /// build a filter from the current state of the builder. This method is called by
    /// [`Connector::end`] because it guarantees that [`Builder`] is an appropriated state to build
    /// a filter (i.e. It doesn't have any intermediate node without children or only one child.)
    fn build(self) -> Option<Filter> {
        // Drop the only Rc which has more than one reference.
        drop(self.pointer);

        let root = self.root?;

        fn walk_nodes(refnode: RefCell<BuilderNode>, prev_filter: Option<&Filter>) -> Filter {
            let node = refnode.into_inner();
            // Because we know that this is unbalanced binary tree towards to the right, we walk until
            // the right most leaf.
            match node.this {
                BuilderNodeType::And | BuilderNodeType::Or => {
                    let right = Rc::into_inner(node.right.expect(&format!(
                    "BUG: invalid binary tree, missing a right child. Node: {:?}",
                    node.this,
                )))
                .expect("BUG: Builder::walk_nodes called with some node in the tree which has more than one strong reference. Make sure that walk_nodes is not called by any other method than Builder:build");
                    let right = walk_nodes(right, prev_filter);

                    let left = Rc::into_inner(node.left.expect(&format!(
                    "BUG: invalid binary tree, missing a left child. Node: {:?}",
                    node.this,
                )))
                .expect("BUG: Builder::walk_nodes called with some node in the tree which has more than one strong reference. Make sure that walk_nodes is not called by any other method than Builder:build");
                    let left = walk_nodes(left, prev_filter);

                    match node.this {
                    BuilderNodeType::And => {
                        Filter{root: Node::And{left: Box::new(left.root), right: Box::new(right.root)}}
                    },
                    BuilderNodeType::Or=> {
                        Filter{root: Node::Or{left: Box::new(left.root), right: Box::new(right.root)}}
                    },
                    _ => unreachable!("this is the same match than parent and this match has all the values to match that parent match arm")
                }
                }
                BuilderNodeType::Exp { field, value } => Filter {
                    root: Node::Exp { field, value },
                },
            }
        }

        Some(walk_nodes(
            Rc::into_inner(root).expect(
                "BUG: calling Builder::build with more than one reference to the root node",
            ),
            None,
        ))
    }
}

/// Chains filter predicates to compose parent predicates when building a filter.
pub struct Connector {
    builder: Builder,
}

impl Connector {
    /// A logical `and` operator between a previous predicate and the next one to append.
    pub fn and(self) -> Builder {
        self.connect(BuilderNodeType::And)
    }

    /// A logical `or` operator between a previous predicate and the next one to append.
    pub fn or(self) -> Builder {
        self.connect(BuilderNodeType::Or)
    }

    /// Ends the build process returning the built filter.
    ///
    /// It returns an error if there was an error when building the filter, for example, an
    /// expression that has to be a valid regular expression isn't valid.
    pub fn end(self) -> Filter {
        self.builder.build().expect("BUG: calling Connector::end with a builder which doesn't have any node. Connect must be built with Builder methods")
    }

    /// Add a `op` to the builder. It panics if `op` is a leaf node, which should only happen if
    /// there is a bug because this method should be called only by other [`Self`] methods.
    fn connect(mut self, op: BuilderNodeType) -> Builder {
        assert!(
            !op.is_leaf(),
            "BUG: calling Connector::connect with a leaf BuilderNodeType"
        );

        if let Some(ref pointer) = self.builder.pointer {
            // This case is adding a new condition to the filter which is building, so the new
            // non-leaf node (specified by `op`) becomes the right child.
            let non_leaf = Rc::new(RefCell::new(BuilderNode {
                this: op,
                left: None,
                right: None,
            }));

            let right = pointer.as_ref().borrow_mut().right.replace(Rc::clone(&non_leaf)).expect(
                "BUG: Builder::pointer must have a right child when Connector::and method is called",
            );

            non_leaf.as_ref().borrow_mut().set_left(right);
            self.builder.pointer = Some(non_leaf);
        } else {
            // This case happens only when this is the first non-leaf node (specified by `op`) of
            // the filter which is building. This non-leaf node becomes the root of the filter.
            let leaf = self
                .builder
                .root
                .expect("BUG: Builder::root cannot be `None` when Connector::and method is called");
            let pointer = Rc::new(RefCell::new(BuilderNode {
                this: op,
                left: Some(Rc::clone(&leaf)),
                right: None,
            }));
            self.builder.pointer = Some(Rc::clone(&pointer));
            self.builder.root = Some(pointer);
        }

        self.builder
    }
}

/// Represents a node of the builder's tree.
#[derive(Debug)]
struct BuilderNode {
    this: BuilderNodeType,
    left: Option<Rc<RefCell<BuilderNode>>>,
    right: Option<Rc<RefCell<BuilderNode>>>,
}

impl BuilderNode {
    fn new_leaf<F>(field: F, exp: Regex) -> Self
    where
        F: ToString,
    {
        Self {
            this: BuilderNodeType::Exp {
                field: field.to_string(),
                value: exp,
            },
            left: None,
            right: None,
        }
    }

    /// Set a left child.
    fn set_left(&mut self, child: Rc<RefCell<BuilderNode>>) {
        self.left = Some(child);
    }

    /// Set `exp` as a right child. It panics if `self` is a leaf node or it already has a right
    /// child.
    fn set_expression_as_right_child<F>(&mut self, field: F, exp: Regex)
    where
        F: ToString,
    {
        assert!(
            !self.this.is_leaf(),
            "BUG: Setting a right child to a leaf node"
        );
        assert!(
            self.right.is_none(),
            "BUG: Setting a right child to a node that it already has a right child"
        );

        self.right = Some(Rc::new(RefCell::new(BuilderNode::new_leaf(field, exp))));
    }
}

#[derive(Debug)]
enum BuilderNodeType {
    And,
    Or,
    Exp { field: String, value: Regex },
}

impl BuilderNodeType {
    fn is_leaf(&self) -> bool {
        match self {
            BuilderNodeType::And | BuilderNodeType::Or => false,
            BuilderNodeType::Exp { field: _, value: _ } => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_filter_with_num_conds_1() {
        let bar = Regex::new("bar").expect("valid regex");
        let _ = Builder::new().regex("foo", bar).end();
    }

    #[test]
    fn build_filter_with_num_conds_2() {
        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let _ = Builder::new()
            .regex("foo", bar)
            .and()
            .regex("foo2", bar2)
            .end();
    }

    #[test]
    fn build_filter_with_num_conds_3() {
        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let bar3 = Regex::new("bar3").expect("valid regex");
        let _ = Builder::new()
            .regex("foo", bar)
            .and()
            .regex("foo2", bar2)
            .or()
            .regex("foo2", bar3)
            .end();
    }

    #[test]
    fn build_filter_with_num_conds_4() {
        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let bar3 = Regex::new("bar3").expect("valid regex");
        let bar4 = Regex::new("bar4").expect("valid regex");
        let _ = Builder::new()
            .regex("foo", bar)
            .or()
            .regex("foo2", bar2)
            .and()
            .regex("foo3", bar3)
            .or()
            .regex("foo4", bar4)
            .end();
    }
}
