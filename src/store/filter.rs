//!  Types for finding access grants inside AGS content.

use crate::error::Error;

use std::cell::OnceCell;
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
    root: Option<Rc<BuilderNode>>,
    current: Option<Rc<BuilderNode>>,
    current_parent: Option<Rc<BuilderNode>>,
}

impl Builder {
    /// Creates a builder for building a [`Filter`].
    pub fn new() -> Self {
        Self {
            root: None,
            current: None,
            current_parent: None,
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
        // TODO: continue here!
        todo!();
    }
}

/// Chains filter predicates to compose parent predicates when building a filter.
pub struct Connector {
    builder: Builder,
}

impl Connector {
    /// A logical `and` operator between a previous predicate and the next one to append.
    fn and(mut self) -> Builder {
        todo!();
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
        todo!();
    }
}

struct BuilderNode {
    kind: BuilderNodeKind,
    left: Rc<BuilderNode>,
    right: Rc<BuilderNode>,
}

enum BuilderNodeKind {
    And,
    Or,
    Exp { field: String, value: Regex },
}

/// Represents a node of the tree representation of a filter.
#[derive(Debug)]
enum Node {
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
    Exp { field: String, value: Regex },
    Empty,
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
