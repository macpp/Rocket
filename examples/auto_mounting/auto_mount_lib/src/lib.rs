#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/lib_route")]
pub fn lib_route() -> &'static str {
    "Hello from liblary!"
}

routes_inventory!(pub);