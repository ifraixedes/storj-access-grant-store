//!  Types for finding access grants inside AGS content.

use std::cell::{OnceCell, RefCell};
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
    // TODO: Try using a OnceCell.
    /// points to the root node of the tree.
    root: Option<Rc<OnceCell<BuilderNode>>>,
    /// points to the last non-leaf node added to the tree.
    pointer: Option<Rc<OnceCell<BuilderNode>>>,
}

impl Builder {
    /// Creates a builder for building a [`Filter`].
    pub fn new() -> Self {
        Self {
            root: None,
            pointer: None,
        }
    }

    /// Nests a filter into the current position of the filter that this instances is building.
    pub fn nest(self, filter: Filter) -> Connector {
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
            let mut node = pointer.as_ref().get_mut().expect("TODO");
            node.set_expression_as_right_child(field, exp);
        } else {
            let root = OnceCell::new();
            root.set(BuilderNode::new_leaf(field, exp)).expect("TODO");
            self.root = Some(Rc::new(root));
        }

        Connector { builder: self }
    }

    fn build(self) -> Filter {
        drop(self.pointer);

        // TODO: has to build this method
        Filter {
            root: Node::Exp {
                field: String::from("todo"),
                value: Regex::new(".*").unwrap(),
            },
        }
    }
}

/// Chains filter predicates to compose parent predicates when building a filter.
pub struct Connector {
    builder: Builder,
}

impl Connector {
    /// A logical `and` operator between a previous predicate and the next one to append.
    pub fn and(mut self) -> Builder {
        self.connect(BuilderNodeType::And)
    }

    /// A logical `or` operator between a previous predicate and the next one to append.
    pub fn or(mut self) -> Builder {
        self.connect(BuilderNodeType::Or)
    }

    /// Ends the build process returning the built filter.
    ///
    /// It returns an error if there was an error when building the filter, for example, an
    /// expression that has to be a valid regular expression isn't valid.
    pub fn end(mut self) -> Filter {
        self.builder.build()
    }

    fn connect(mut self, op: BuilderNodeType) -> Builder {
        // TODO: check if it's possible to only compile these assertions during tests
        assert!(
            !op.is_leaf(),
            "BUG: calling Connector::connect with a leaf BuilderNodeType"
        );

        if let Some(ref pointer) = self.builder.pointer {
            // This case is adding a new condition to the filter which is building, so the new
            // non-leaf node (in this case an AND operator) has to be added to the right.
            let and = OnceCell::new();
            and.set(BuilderNode {
                this: op,
                left: None,
                right: None,
            });
            let and = Rc::new(and);
            let right = pointer.as_ref().get_mut().expect("TODO").right.replace(Rc::clone(&and)).expect(
                "BUG: Builder::pointer must have a right child when Connector::and method is called",
            );

            and.as_ref().get_mut().expect("TODO").set_left(right);
            self.builder.pointer = Some(and);
        } else {
            // This case happens only when the first non-leaf node (in this case an AND operator) is
            // added to the filter which is building. This non-leaf node become the root of the
            // filter.
            let root = self
                .builder
                .root
                .expect("BUG: Builder::root cannot be `None` when Connector::and method is called");
            let non_leaf = OnceCell::new();
            non_leaf.set(BuilderNode {
                this: op,
                left: Some(Rc::clone(&root)),
                right: None,
            });
            self.builder.pointer = Some(Rc::new(non_leaf));
            self.builder.root = Some(Rc::clone(&root));
        }

        self.builder
    }
}

#[derive(Debug)]
struct BuilderNode {
    this: BuilderNodeType,
    left: Option<Rc<OnceCell<BuilderNode>>>,
    right: Option<Rc<OnceCell<BuilderNode>>>,
    /*
    this: Rc<BuilderNodeType>,
    left: Option<Rc<BuilderNode>>,
    right: Option<Rc<BuilderNode>>,
    */
    /*
    this: Rc<RefCell<BuilderNodeType>>,
    left: Option<Rc<RefCell<BuilderNodeType>>>,
    right: Option<Rc<RefCell<BuilderNodeType>>>,
    */
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

    /*
    fn set_right_child(&mut self, node: BuilderNodeType) {
        // TODO: check if it's possible to only compile these assertions during tests
        assert!(!self.this.is_leaf(), "Setting a right child to a leaf node");
        assert!(
            self.right.is_some(),
            "Setting a right child to a node that it already has a right child"
        );

        self.right = Some(Rc::new(node));
    }
    */

    /*
    fn extract_right_child(&mut self) -> Option<Rc<RefCell<BuilderNode>>> {
        self.right.take()
    }
    */

    fn set_left(&mut self, child: Rc<OnceCell<BuilderNode>>) {
        self.left = Some(child);
    }

    fn set_right(&mut self, child: Rc<OnceCell<BuilderNode>>) {
        self.right = Some(child);
    }

    fn set_expression_as_right_child<F>(&mut self, field: F, exp: Regex)
    where
        F: ToString,
    {
        // TODO: check if it's possible to only compile these assertions during tests
        assert!(
            !self.this.is_leaf(),
            "BUG: Setting a right child to a leaf node"
        );
        assert!(
            self.right.is_none(),
            "BUG: Setting a right child to a node that it already has a right child"
        );

        let right = OnceCell::new();
        right.set(BuilderNode::new_leaf(field, exp));
        self.right = Some(Rc::new(right));
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

/*
impl Node {
    fn new_emtpy_and() -> Self {
        Node::And {
            left: Box::new(Node::Empty),
            right: Box::new(Node::Empty),
        }
    }

    fn is_empty(&self) -> bool {
        if let Node::Empty = self {
            true
        } else {
            false
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Node::And { left: _, right: _ } | Node::Or { left: _, right: _ } => false,
            _ => true,
        }
    }

    fn has_children(&self) -> (bool, bool) {
        match self {
            Node::And { left, right } | Node::Or { left, right } => (
                !matches!(**left, Node::Empty),
                !matches!(**right, Node::Empty),
            ),
            _ => (false, false),
        }
    }

    fn set_left_child(&mut self, child: Node) {
        match self {
            Node::And { left, right: _ } | Node::Or { left, right: _ } => {
                *left = Box::new(child);
            }
            _ => todo!(),
        }
    }

    fn append_and_to_left(&mut self) {
        match self {
            Node::And { left: _, right } | Node::Or { left: _, right } => {
                if right.is_leaf() {
                    let and = Node::And {
                        right: *right,
                        left: Box::new(Node::Empty),
                    };
                    *right = Box::new(and);
                } else {
                }
            }
            _ => todo!(),
        };
    }

    /*
    fn get_right_child(&mut self) -> Node {
        match self {
            Node::And { left: _, right } | Node::Or { left: _, right } => **right,
            _ => todo!(),
        }
    }
    */

    /*
    fn derive_right_child(&mut self, child: Node) {
        match self {
            Node::And { left: _, right } | Node::Or { left: _, right } => {
                if right.is_leaf() {
                    *right = Box::new(child);
                } else {

                }
            }
            _ => todo!(),
        };
    }
    */
}
*/

/*
/// Set `child` to `parent`'s right.
///
/// This method helps to detect bugs of `Builder` implementation through panics, so it assumes that
/// `parent` and its children, and `child` are of certain variants.
fn set_right_child(mut parent: Node, child: Node) -> Node {
    if let Node::Empty = child {
        panic!("BUG: setting a `Node::Empty` as a parent's right node")
    }
    match &mut parent {
        Node::And { left, right } | Node::Or { left, right } => {
            if let Node::Empty = **left {
                panic!("BUG: setting a right child node to a parent node of a variant `Node::And` or  `Node::Or` whose left node isn't `Node::Empty`");
            } else if let Node::Empty = **right {
                *right = Box::new(child);
            } else {
                panic!("BUG: setting a right child node to a parent node of a variant `Node::And` or  `Node::Or` whose right node isn't `Node::Empty`");
            }
        }
        _ => panic!(
            "BUG: setting a node to a parent node that isn't a variant `Node:And`, nor `Node:Or`"
        ),
    };

    parent
}

/// Set `child` to `parent`'s left.
///
/// This method helps to detect bugs of `Builder` implementation through panics, so it assumes that
/// `parent` and its children, and `child` are of certain variants.
fn set_left_child(mut parent: Node, child: Node) -> Node {
    if let Node::Empty = child {
        panic!("BUG: setting a `Node::Empty` as a parent's left node")
    }
    match &mut parent {
        Node::And { left, right } | Node::Or { left, right } => {
            if let Node::Empty = **right {
                if let Node::Empty = **left {
                    *left = Box::new(child);
                } else {
                    panic!("BUG: adding a left child node to a parent node of a variant `Node::And` or  `Node::Or` whose left node isn't `Node::Empty`");
                }
            } else {
                panic!("BUG: adding a left child node to a parent node of a variant `Node::And` or  `Node::Or` whose right node isn't `Node::Empty`");
            }
        }
        _ => panic!(
            "BUG: adding a node to a parent node that isn't a variant `Node:And`, nor `Node:Or`"
        ),
    };

    parent
}
*/

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

    #[test]
    #[ignore]
    fn build_filter_successfully() {
        let bar = Regex::new("bar").expect("valid regex");
        let filter_simple = Builder::new().regex("foo", bar).end();

        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let _filter_no_nested = Builder::new()
            .regex("foo", bar)
            .and()
            .regex("foo2", bar2)
            .end();

        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let bar3 = Regex::new("bar3").expect("valid regex");
        let _filter_2_ops = Builder::new()
            .regex("foo", bar)
            .and()
            .regex("foo2", bar2)
            .or()
            .regex("foo3", bar3)
            .end();

        let bar = Regex::new("bar").expect("valid regex");
        let _filter_nested = Builder::new()
            .nest(filter_simple)
            .or()
            .regex("foo", bar)
            .end();

        let bar = Regex::new("bar").expect("valid regex");
        let bar2 = Regex::new("bar2").expect("valid regex");
        let bar3 = Regex::new("bar3").expect("valid regex");
        let bar4 = Regex::new("bar3").expect("valid regex");
        let _filter_nested_after = Builder::new()
            .regex("foo", bar)
            .and()
            .regex("foo2", bar2)
            .or()
            .nest(
                Builder::new()
                    .regex("foo3", bar3)
                    .and()
                    .regex("foo4", bar4)
                    .end(),
            )
            .end();
    }

    /*
       #[test]
       fn build_filter_unsuccessfully() {
    // TODO: check errors variant

    let err = Builder::new()
            .regex("foo", "[bar")
            .end()
            .expect_err("one field: invalid regex");

        let err = Builder::new()
            .regex("foo", "bar")
            .and()
            .regex("foo2", "bar2]")
            .end()
            .expect_err("two fields: invalid regex second field");

        let err = Builder::new()
            .regex("foo", "bar")
            .and()
            .regex("foo2", "bar2")
            .or()
            .regex("foo3", "bar3")
            .end()
            .expect_err("three fields with 'and' and 'or': invalid second field");
    }
    */
}
