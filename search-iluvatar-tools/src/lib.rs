#![doc = include_str!("../README.md")]
#![deny(warnings, unsafe_code, missing_docs)]

use std::{env::var_os, path::PathBuf};

/// Returns the path to the iluvatar home directory, if it is set.
#[inline]
pub fn find_ilu_home() -> Option<PathBuf> {
    var_os("ILUVATAR_PATH").map(PathBuf::from)
}

#[test]
fn test() {
    println!("{}", find_ilu_home().unwrap().display());
}
