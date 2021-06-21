pub mod object;

// use fs::FileSystem;
use object::blob::Blob;
use object::commit;
use object::commit::Commit;
use object::tree;
use object::tree::Tree;
use object::GitObject;
use std::io;
use std::io::prelude::*;

// #[derive(Debug)]
// pub struct Git<F: FileSystem> {
//   pub file_system: F,
// }
//
// impl<F: FileSystem> Git<F> {}
