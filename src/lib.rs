/*
Copyright 2021 Google LLC

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use tree_sitter::{Language, Parser, Query, Tree};

#[macro_use]
extern crate log;

pub mod builder;
mod capture;
mod util;

#[cfg(feature = "python")]
pub mod python;
pub mod query;
pub mod result;

extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
}

/// Helper function to parse an input string
/// into a tree-sitter tree, using our own slightly modified
/// C grammar. This function won't fail but the returned
/// Tree might be invalid and contain errors.
pub fn parse(source: &str, cpp: bool) -> Tree {
    let language = if !cpp {
        unsafe { tree_sitter_c() }
    } else {
        unsafe { tree_sitter_cpp() }
    };
    let mut parser = Parser::new();
    if let Err(e) = parser.set_language(language) {
        // 有错误直接打印错误然后 panic, 不往外传递
        eprintln!("{}", e);
        panic!();
    }

    parser.parse(source, None).unwrap()
}

// Internal helper function to create a new tree-sitter query.
fn ts_query(sexpr: &str, cpp: bool) -> tree_sitter::Query {
    let language = if !cpp {
        unsafe { tree_sitter_c() }
    } else {
        unsafe { tree_sitter_cpp() }
    };
    // dbg!(&sexpr);

    match Query::new(language, &sexpr) {
        Ok(q) => q,
        Err(e) => {
            eprintln!(
                "Tree sitter query generation failed: {:?}\n {}",
                e.kind, e.message
            );
            eprintln!("sexpr: {}", sexpr);
            eprintln!("This is a bug! Can't recover :/");
            std::process::exit(1);
        }
    }
}

