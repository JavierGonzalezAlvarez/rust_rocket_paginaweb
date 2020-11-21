#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::io;
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
NamedFile::open("public/index.html")
}

#[get("/sobrerust")]
fn rust() -> io::Result<NamedFile> {
NamedFile::open("public/sobrerust.html")
}

#[get("/contacto")]
fn contacto() -> io::Result<NamedFile> {
NamedFile::open("public/contacto.html")
}

fn main() {
// crear una ruta "/"
rocket::ignite()
// "/" es un namespace
.mount("/", routes![index, rust, contacto]).launch();
}