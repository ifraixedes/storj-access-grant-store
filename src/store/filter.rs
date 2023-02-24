//!  Types for finding access grants inside AGS content.

use crate::error::Error;

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
    root: Option<Rc<Node>>,
    cursor: Option<Rc<Node>>,
}

impl Builder {
    /// Creates a builder for building a [`Filter`].
    pub fn new() -> Self {
        Self {
            root: None,
            cursor: None,
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
        match self.root {
            None => {
                self.root = Some(Rc::new(Node::Exp {
                    field: field.to_string(),
                    value: exp,
                }));
                self.cursor = Some(Rc::clone(
                    self.root.as_ref().expect("BUG: root cannot be None"),
                ));
                Connector { builder: self }
            }
            Some(_) => {
                let mut node = *self.cursor.expect("BUG: cursor cannot be None ");
                match &mut node {
                    Node::And { left, right } => {
                        *right = Box::new(Node::Exp {
                            field: field.to_string(),
                            value: exp,
                        });

                        Connector { builder: self }
                    }
                }
            }
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

/// Represents a node of the tree representation of a filter.
#[derive(Debug)]
enum Node {
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
    Exp { field: String, value: Regex },
    Empty,
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
