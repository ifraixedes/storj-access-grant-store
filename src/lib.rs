//! # Access Grant Store
//!
//! Library for storing
//! [Storj Access Grants](https://docs.storj.io/dcs/concepts/access/access-grants/) in a text file
//! with some specific format, offering the following capabilities:
//!
//! - Store user's metadata associated to them for facilitating the management.
//! - Add / Read / Update / Delete  an access grant.
//! - Search them by tags and/or fields defined by the user.
//!
//! The access grant is encrypted before storing it in the file.

#![deny(missing_docs)]

mod error;
mod parser;
mod store;
