//!  Types for finding access grants inside AGS content.

use crate::error::Error;

use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

use regex::Regex;

/// Filters access grants according to its containing predicates.
/// For building a filter use a [`Builder`] instance.
#[derive(Debug)]
pub struct Filter {
    root: Node,
}

impl Filter {}

/// Builds a filter with the indicated predicates.
pub struct Builder {
    root: Rc<Node>,
    last_parent: Option<Rc<Node>>,
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>,
}

impl Builder {
    /// Creates a builder for building a [`Filter`].
    pub fn new() -> Self {
        Self {
            root: Rc::new(Node::Empty),
            last_parent: None,
            left_child: None,
            right_child: None,
        }
    }

    /// Nests a filter into the current position of the filter that this instances is building.
    pub fn nest(mut self, filter: Filter) -> Connector {
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
        let leaf = Rc::new(Node::Exp {
            field: field.to_string(),
            value: exp,
        });

        if self.root.is_empty() {
            self.root = leaf;
        }

        //        if self.last_parent.is_none() {
        //            self.last_parent = Some(Rc::new(Node::Exp {
        //                field: field.to_string(),
        //                value: exp,
        //            }));
        //        } else {
        //            let last_parent = self.last_parent.as_mut().expect("BUG: accessing to Builder::last_parent with is `None`. This code should be executed after ensuring that isn't `None`");
        //        }

        Connector { builder: self }
    }

    fn build(self) -> Filter {
        //  let root = if let Some(r) = self.root {
        //      Rc::try_unwrap(r).expect(
        //          "BUG: called Builder::build with a root node with more than one strong reference",
        //      )
        //  } else if let Some(p) = self.last_parent {
        //      Rc::try_unwrap(p)
        //          .expect("BUG: called Builder::build without a root, but with a last parent node with more than one strong reference")
        //  } else {
        //      let l = self.left_child.expect(
        //          "BUG: Builder::build without a root and a last parent and without a left child",
        //      );
        //      Rc::try_unwrap(l)
        //          .expect("BUG: called Builder::build without a root and a last parent, but with a left child node with more than one strong reference")
        //  };
        if let Some(p) = self.last_parent {
            drop(p);
        }

        Filter {
            root: Rc::try_unwrap(self.root).expect(
                "BUG: called Builder::build with a root node with more than one strong reference",
            ),
        }
    }
}

/// Chains filter predicates to compose parent predicates when building a filter.
pub struct Connector {
    builder: Builder,
}

impl Connector {
    /// A logical `and` operator between a previous predicate and the next one to append.
    fn and(mut self) -> Builder {
        if let Some(previous) = self.builder.last_parent {
            if Rc::ptr_eq(&previous, &self.builder.root) {}
        } else {
            self.builder.last_parent = Some(Rc::new(Node::And {
                left: Box::new(Node::Empty),
                right: Box::new(Node::Empty),
            }));
        }

        if self.builder.root.is_leaf() {
            self.builder.root =
                Rc::clone(self.builder.last_parent.as_ref().expect(
                    "BUG: accessing to Builder::last_parent with is `None`. This code should be executed after ensuring that isn't `None`",
                ));
        }

        self.builder
        /*
        let leaf = {
            let rc = self
                .builder
                .last_added
                .expect("BUG: Calling Connector::and method with a Builder::last_added = None");
            Rc::try_unwrap(rc).expect("BUG: Calling Connector::and method with a Builder::last_added with more than one strong reference")
        };

        if self.builder.root.is_none() {
            self.builder.root = Some(Rc::new(Node::And {
                left: Box::new(leaf),
                right: Box::new(Node::Empty),
            }));
        } else {
            let rc = self.builder.root.unwrap();
            let root = Rc::try_unwrap(rc).expect("BUG: Calling Connector::end method with a Builder::root with more than one strong reference");
        }
        */
    }

    /// A logical `or` operator between a previous predicate and the next one to append.
    fn or(mut self) -> Builder {
        todo!();
    }

    /// Ends the build process returning the built filter.
    ///
    /// It returns an error if there was an error when building the filter, for example, an
    /// expression that has to be a valid regular expression isn't valid.
    fn end(mut self) -> Filter {
        self.builder.build()
    }
}

/// Represents a node of the tree representation of a filter.
#[derive(Debug)]
enum Node {
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
    Exp { field: String, value: Regex },
    Empty,
}

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
            .and()
            .regex("foo2", bar3)
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
