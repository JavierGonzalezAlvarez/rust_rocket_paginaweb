#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;
//use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
//fn index() -> &'static str {
    // "hola"
fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

fn main() {
    // crear una ruta "/"
    rocket::ignite()
        .mount("/", routes![index]).launch();
}