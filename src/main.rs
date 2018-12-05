#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(&Path::new("static/index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(&Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}

