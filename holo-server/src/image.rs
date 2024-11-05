//! Image fs grabbing utility methods

use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use rand::{seq::SliceRandom, RngCore};

/// Method for grabbing a random file from a subdirectory
pub fn choose_random_file<P: AsRef<OsStr>>(path: P, rng: &mut impl RngCore) -> Option<PathBuf> {
    let path = Path::new(&path);
    let files: Vec<_> = fs::read_dir(path)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    files.choose(rng).cloned()
}
