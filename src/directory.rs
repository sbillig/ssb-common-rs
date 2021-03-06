//! Helpers for working with the default directory (layout) of ssb.

use std::path::PathBuf;
use std::env::{home_dir, var_os};
use std::ffi::{OsStr, OsString};

/// The name of the directory where ssb stores its data by default.
///
/// Note that ssb always prepends a "." to the directory, so the actual path used
/// will be `".ssb"`, not `"ssb"`. The directory is resolved from the home
/// directory of the user. So a full path could for example be
/// `"/home/foobar/.ssb"` (on linux).
pub const DEFAULT_SSB_DIRECTORY_NAME: &'static str = "ssb";

/// Applications should read this environment variable to use an ssb directory
/// other than the default one.
pub const ENV_SSB_DIRECTORY_NAME: &'static str = "ssb_appname";

/// A convenience function that returns the full path to the ssb directory.
///
/// This function checks the `ENV_SSB_DIRECTORY_NAME` environment variable to
/// override the default directory if it is set.
///
/// Returns `None` if `std::env::home_dir` returns `None`.
pub fn ssb_directory() -> Option<PathBuf> {
    let mut dirname = OsString::from(".");
    dirname.push(var_os(OsStr::new(ENV_SSB_DIRECTORY_NAME))
                     .unwrap_or(OsString::from(DEFAULT_SSB_DIRECTORY_NAME)));

    home_dir().map(|mut home| {
                       home.push(dirname);
                       home
                   })
}
