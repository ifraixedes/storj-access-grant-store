//! Errors returned by this crate.

use std::error as stderr;
use std::fmt;

/// Convenient type alias to shorten the signature on every usage.
pub(crate) type BoxError = Box<dyn stderr::Error + Send + Sync>;

/// The error type that this crate use for wrapping errors.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Identifies errors produced by the internal implementation that aren't expected to happen.
    /// For example, some of these may be due to bugs in dependencies or unexpected operative system
    /// behaviors.
    /// Read the [`Internal`] documentation to have a more detailed explanation.
    Internal(Internal),
    /// Identifies invalid arguments passed to a function or method.
    InvalidArguments(Args),
}

impl Error {
    /// Creates an [`Internal` variant](Self::Internal) from the provided context message and the
    /// error that originated it.
    pub(crate) fn new_internal(ctx_msg: &str, err: BoxError) -> Self {
        Error::Internal(Internal {
            ctx_msg: String::from(ctx_msg),
            inner: err,
        })
    }

    /// Convenient constructor for creating an [`InvalidArguments` variant](Self::InvalidArguments)
    /// Error.
    ///
    /// See [`Args`] documentation to know about the convention for the value of the `names`
    /// parameter because this constructor will panic in the future when the constraints will be
    /// implemented by [`Args::new`] constructor.
    pub(crate) fn new_invalid_arguments(names: &str, msg: &str) -> Self {
        Self::InvalidArguments(Args::new(names, msg))
    }
}

impl stderr::Error for Error {
    fn source(&self) -> Option<&(dyn stderr::Error + 'static)> {
        match self {
            Error::InvalidArguments { .. } => None,
            Error::Internal(Internal { inner, .. }) => Some(inner.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::InvalidArguments(args) => {
                write!(f, "{}", args)
            }
            Error::Internal(details) => {
                write!(f, "{}", details)
            }
        }
    }
}

/// Represents invalid arguments error regarding the business domain.
///
/// # Example
///
/// ```ignore
/// // This example is ignored because it shows how to return an `InvalidArguments` error through
/// // the constructor methods that aren't exported outside of this crate.
///
/// use storj_uplink_lib::{Error, Result};
///
/// fn positive_non_zero_div_and_mul(a: i64, b: i64, div: i64) ->Result<i64> {
///     if div == 0 {
///         return Err(Error::new_invalid_arguments("div", "div cannot be 0"));
///     }
///
///     if (a == 0 && b != 0) || (a != 0 && b == 0) {
///         return Err(Error::new_invalid_arguments(
///             "(a,b)", "a and b can only be 0 if both are 0",
///         ));
///     }
///
///     if (a >= 0 && b >= 0 && div > 0) || (a <= 0 && b <= 0 && div < 0 ) {
///         return Ok((a/div) * (b/div));
///     }
///
///     Err(Error::new_invalid_arguments(
///         "<all>", "all the arguments must be positive or negative, they cannot be mixed",
///     ))
/// }
/// ```
#[derive(Debug)]
// TODO: delete this type if we don't use it, otherwise map into the Error enum.
pub struct Args {
    /// One or several parameters names; it has several conventions for expressing the involved
    /// parameters.
    ///
    /// * When a specific parameter is invalid its value is the exact parameter name.
    /// * When the parameter is a list (vector, array, etc.), the invalid items can be
    ///   __optionally__ indicated using square brackets (e.g. `l[3,5,7]`).
    /// * when the parameter is struct, the invalid fields or method return return values can be
    ///    __optionally__ indicated using curly brackets (e.g invalid field: `person{name}`, invalid
    ///    method return value: `person{full_name()}`, invalid fields/methods:
    ///   `employee{name, position()}`).
    /// * When several parameters are invalid, its values is the parameters names wrapped in round
    ///   brackets (e.g. `(p1,p3)`); it also accepts any above combination of parameters types
    ///   (e.g. `(p1, l[2,10], person{name})`).
    /// * When all the function parameters are invalid, `<all>` is used.
    ///
    // For enforcing these constrains internally use [`Self::new`].
    pub names: String,
    /// A human friendly message that explains why the argument(s) are invalid.
    pub msg: String,
}

impl Args {
    /// Creates a new instance or panic if [`Self::names`] field's constrains mentioned in the
    /// fields' documentation are violated.
    ///
    /// It panics because it makes easier to find a BUG on the `name`'s passed value.
    ///
    /// TODO(https://github.com/storj-thirdparty/uplink-rust/issues/52): Implement the `name`'s
    /// constraints validation and panic if the validation fails.
    fn new(names: &str, msg: &str) -> Self {
        Args {
            names: String::from(names),
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} arguments have invalid values. {}",
            self.names, self.msg
        )
    }
}

/// Represents an error that happen because of the violation of an internal assumption.
///
/// An assumption can be violated by the use of a function that returns an error when it should
/// never return it or because it's validated explicitly by the implementation or because we know
/// that the function return an error in some cases that we know that aren't happening in the
/// implementation.
///
/// An assumption examples is:
/// After parsing a file with a Pest grammar and the file parsed correctly, we don't find a pair in
/// the position that's expected. Despite Pest documentation  indicates that using `unwrap` and
/// `unreachable` is idiomatic when using it
/// (see: https://pest.rs/book/parser_api.html#using-pair-and-pairs-with-a-grammar), we prefer to
/// handle the error an return this kind.
/// On the other hand, we may return an internal error and it may be a bug in our side, so whenever
/// you get one of this, open an issue if it isn't clear the reason.
#[derive(Debug)]
pub struct Internal {
    /// A human friendly message to provide context of the error.
    pub ctx_msg: String,
    /// The inner error that caused this internal error
    inner: BoxError,
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.ctx_msg)
    }
}

impl stderr::Error for Internal {
    fn source(&self) -> Option<&(dyn stderr::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}
