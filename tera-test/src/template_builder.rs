use std::error::Error;

use include_dir::{
    include_dir, Dir,
    DirEntry::{Dir as SubDir, File},
};
use lazy_static::lazy_static;
use tera::Tera;

// This file handles the processing of bundled templates, abstracting the details to provide a single constant to refer to the templates.
// You can pretty much substitute the "tera" variable or "TEMPLATES" constant in the Tera docs with the exported value from this file.

// Tera::default() is used with tera.add_raw_templates() instead of Tera::new() because Tera::new() requires the use of a glob pattern even if it won't be used.
// https://stackoverflow.com/a/57760754 - Details the basic steps needed to include the entire template tree in the binary.

static TEMPLATES_DIR: Dir<'_> = include_dir!("pages");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let result = walk_and_collect_templates(&TEMPLATES_DIR);
        let mut tera = Tera::default();

        // Error handling appears in "error.kind" and "error.source()".
        // Extract the actual error to allow pretty printing of template errors (instead of control characters like \n).
        match tera.add_raw_templates(result) {
            Ok(result) => result,
            Err(error) => {
                print!("[Template Builder] {error}");

                if let Some(b) = error.source() {
                    println!("{b}");
                } else {
                    println!();
                }

                panic!("Error parsing templates.");
            }
        };

        tera
    };
}

fn walk_and_collect_templates<'a>(directory: &'a Dir) -> Vec<(&'a str, &'a str)> {
    let mut result = Vec::new();

    for entry in directory.entries() {
        match entry {
            SubDir(subdirectory) => {
                let mut inner_result = walk_and_collect_templates(&subdirectory);
                result.append(&mut inner_result);
            }
            File(file) => {
                result.push((
                    file.path().to_str().expect(&format!(
                        "[Template Builder] Failed to convert file path {:?} to a string.",
                        file.path()
                    )),
                    file.contents_utf8().expect(&format!(
                        "[Template Builder] The contents of file {:?} could not resolve to a proper UTF-8 file.",
                        file.path()
                    )),
                ));
            }
        }
    }

    result
}
