#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::auto_mount::RoutesCollection;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/x")]
fn x() -> &'static str {
    "This is x route"
}

mod test {
    auto_mount_mod_hint!("/test");

    #[get("/y")] // will be mounted to /test/y
    fn y() -> &'static str {
        "This is y route in test module"
    }

    #[get("/z")] // will be mounted to /test/z
    fn z() -> &'static str {
        "This is z route in test module"
    }
}

mod disabled {
    auto_mount_mod_hint!(disabled);

    #[get("/w")] // will not be mounted
    fn w() -> &'static str {
        "this route should be disabled"
    }
}

fn main() {
    prepare_rocket().launch();
}

pub fn prepare_rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", RoutesInventory::with_hint_mount_point("/"))
    .mount("/test", RoutesInventory::with_hint_mount_point("/test"))
}

routes_inventory!();
