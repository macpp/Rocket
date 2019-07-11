#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/info")]
fn x() -> &'static str {
    "this is test web server"
}

mod user {
    auto_mount_hint!("/user");

    #[get("/about")] // will be mounted to /test/y
    fn y() -> &'static str {
        "current user is unnown"
    }

    #[get("/logout")] // will be mounted to /test/z
    fn z() -> &'static str {
        "logged out!"
    }
}

mod secret_routes {
    auto_mount_hint!(enabled=false);

    #[get("/secret")] // will not be mounted
    fn w() -> &'static str {
        "secret route!"
    }
}

fn main() {
   rocket::ignite()
   .auto_mount_all::<RoutesInventory>() // see tests for more automounting options
   .launch();
}

routes_inventory!();
