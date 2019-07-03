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
macro_rules! auto_mount_mod_hint {
    ($l:literal) => {
        use $crate::auto_mount::AutoMountModuleHint;
        static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: $l, enabled: true};
    };
    (disabled) => {
        use $crate::auto_mount::AutoMountModuleHint;
        static __ROCKED_MOD_AUTO_MOUNT_INFO : AutoMountModuleHint = AutoMountModuleHint {mount_point: "/", enabled: false};
    }
}

pub trait RoutesCollection {
    fn unfiltred() -> Vec<(crate::Route,&'static AutoMountModuleHint)>;
    fn all_enabled() -> Vec<crate::Route> {
        Self::unfiltred()
        .into_iter()
        .filter(|x| x.1.enabled)
        .map(|x| x.0)
        .collect()
    }
    fn with_hint_mount_point(path: &str) -> Vec<crate::Route> {
        Self::unfiltred()
        .into_iter()
        .filter(|x| x.1.mount_point == path && x.1.enabled)
        .map(|x| x.0)
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
            fn unfiltred() -> Vec<($crate::Route,&'static $crate::auto_mount::AutoMountModuleHint)> {
                let mut v = vec![];
                for route_info in $crate::inventory::iter::<RoutesInventory > {
                    v.push((route_info.route.into(), route_info.mod_hint));
                }
                v
            }
        }
        $crate::inventory::collect!(RoutesInventory);

        #[cfg(not(rocket_codegen_auto_mounting))]
        compile_error!("Auto mouting not configured properly! See documentation of this macro for more information.");
    }
}
