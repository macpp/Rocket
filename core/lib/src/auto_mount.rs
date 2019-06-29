pub struct AutoMountModuleInfo {
    pub base: &'static str,
    pub enabled: bool,
}

pub mod __default_auto_mount_info {
    use super::*;
    pub static __rocket_mod_auto_mount_info : AutoMountModuleInfo = AutoMountModuleInfo {base: "/", enabled: true};
}

/// Allows to configure behavior of [auto_mount()](Rocket::auto_mount) for all routes in module
///
/// # Example - set base path for all routes in module
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket::{mod_auto_mount,get};
///
/// mod secret_routes {
///     mod_auto_mount!("/foo");
///
///     // this route will be mounted at /foo/bar when auto_mount() is used
///     #[get("/bar")]
///     fn bar() -> &'static str {
///         "Hello!"
///     }
/// }
///
/// ```
///  # Example - disable automatic mounting for all routes in module
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket::{mod_auto_mount,get};
///
/// mod secret_routes {
///     mod_auto_mount!(disabled);
///
///     // this route will not be mounted by auto_mount()
///     #[get("/secret")]
///     fn secret() -> &'static str {
///         "secret route"
///     }
/// }
/// ```
#[macro_export]
macro_rules! mod_auto_mount {
    ($l:literal) => {
        use $crate::auto_mount::AutoMountModuleInfo;
        static __rocket_mod_auto_mount_info : AutoMountModuleInfo = AutoMountModuleInfo {base: $l, enabled: true};
    };
    (disabled) => {
        use $crate::auto_mount::AutoMountModuleInfo;
        static __rocket_mod_auto_mount_info : AutoMountModuleInfo = AutoMountModuleInfo {base: "/", enabled: false};
    }
}

#[macro_export]
macro_rules! routes_inventory {
    () => {
        pub(crate)struct RoutesInventory {
            pub mod_info: &'static $crate::auto_mount::AutoMountModuleInfo,
            pub route: &'static $crate::StaticRouteInfo,
        }
        impl RoutesInventory {
            pub fn get_all() -> Vec<$crate::Route> {
                println!("getting all the routes!");
                let mut v = vec![];
                for route in $crate::inventory::iter::<RoutesInventory > {
                    /*if route.mod_info.enabled {
                        self = self.mount(route.mod_info.base,vec![route.route.into()]);
                    }*/
                    v.push(route.route.into());
                }
                println!("total routes: {}", v.len());
                v
            }

            pub fn get_all_with_hint_base(hint_base: &str) -> Vec<$crate::Route> {
                println!("getting all the routes with hint!");
                let mut v = vec![];
                for route in $crate::inventory::iter::<RoutesInventory > {
                    /*if route.mod_info.enabled {
                        self = self.mount(route.mod_info.base,vec![route.route.into()]);
                    }*/
                    if hint_base == route.mod_info.base {
                        v.push(route.route.into());
                    }
                }
                println!("total routes: {}", v.len());
                v
            }
        }
        $crate::inventory::collect!(RoutesInventory);
    }
}
