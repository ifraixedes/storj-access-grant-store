//! Access the content of an AGS file.

mod filter;

use filter::Filter;

use crate::error::Error;

use std::path::Path as stdPath;

/// Access grant store based on AGS text file.
pub struct AGSFile {}

impl AGSFile {
    pub fn load(content: &str) -> Result<Self, Error> {
        todo!();
    }

    pub fn load_from_file(file: &stdPath) -> Result<Self, Error> {
        todo!();
    }

    pub fn find_grants(&self, f: &Filter) -> Vec<AccessGrant> {
        todo!();
    }
}

pub struct AccessGrant {
    pub name: String,
    pub project: String,
    pub encrypted_grant: String,
}
