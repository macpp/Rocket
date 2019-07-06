pub struct AutoMountModuleHint {
    pub mount_point: &'static str,
    pub enabled: bool,
}

pub mod __default_auto_mount_info {
    use super::*;
    pub static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: "/", enabled: true};
}

/// Allows to configure behavior of [auto_mount()](Rocket::auto_mount) for all routes in module
///
/// # Example - set base path for all routes in module
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket::{auto_mount_mod_hint,get};
///
/// mod secret_routes {
///     auto_mount_mod_hint!("/foo");
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
/// use rocket::{auto_mount_mod_hint,get};
///
/// mod secret_routes {
///     auto_mount_mod_hint!(disabled);
///
///     // this route will not be mounted by auto_mount()
///     #[get("/secret")]
///     fn secret() -> &'static str {
///         "secret route"
///     }
/// }
/// ```
#[macro_export] 
// TODO: since its hint, it should be possible to set mounting point for disabled module
// TODO: additional field (&'static Any or something) for user data
//TODO: rename to auto_mount_hint ?
macro_rules! auto_mount_mod_hint {
    ($l:literal) => {
        use $crate::auto_mount::AutoMountModuleHint;
        static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: $l, enabled: true};
    };
    ($l:literal,disabled) => {
        use $crate::auto_mount::AutoMountModuleHint;
        static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: $l, enabled: false};
    };
    (disabled) => {
        use $crate::auto_mount::AutoMountModuleHint;
        static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: "/", enabled: false};
    }
}

pub struct AutoRoute(pub crate::Route,pub &'static AutoMountModuleHint);

pub trait RoutesCollection {
    //TODO: maybe it should be hashmap of something? -> nope, what if two moduls with auto_mount_hint("/asdf")? 
    fn unfiltred() -> &'static[(AutoRoute)];
    fn all_enabled() -> Vec<crate::Route> {
        Self::unfiltred()
        .iter()
        .filter(|x| x.1.enabled)
        .map(|x| x.0.clone())
        .collect()
    }
    fn with_hint_mount_point(path: &str) -> Vec<crate::Route> {
        Self::unfiltred()
        .iter()
        .filter(|x| x.1.mount_point == path && x.1.enabled)
        .map(|x| x.0.clone())
        .collect()
    }
}

#[macro_export]
macro_rules! routes_inventory {
    () => {
        routes_inventory!(pub(crate));
    };
    ($x : vis) => {
        $x struct RoutesInventory {
            pub mod_hint: &'static $crate::auto_mount::AutoMountModuleHint,
            pub route: &'static $crate::StaticRouteInfo,
        }
        impl $crate::auto_mount::RoutesCollection for crate::RoutesInventory {
            fn unfiltred() -> &'static[($crate::auto_mount::AutoRoute)] {
                $crate::lazy_static::lazy_static! {
                    static ref ALL_ROUTES: Vec<($crate::auto_mount::AutoRoute)> = {
                        let mut v = vec![];
                            for route_info in $crate::inventory::iter::<RoutesInventory > {
                                v.push($crate::auto_mount::AutoRoute(route_info.route.into(), route_info.mod_hint));
                            }
                        v
                    };
                }
                ALL_ROUTES.as_slice()
                
            }
        }
        $crate::inventory::collect!(RoutesInventory);

        #[cfg(not(rocket_codegen_auto_mounting))]
        compile_error!("Auto mouting not configured properly! See documentation of this macro for more information.");
    }
}
