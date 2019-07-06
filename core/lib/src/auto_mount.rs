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
// TODO: additional field (&'static Any or something) for user data??
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
    fn unfiltred() -> Vec<AutoRoute>;
    fn with_hint_mount_point(path: &str) -> Vec<crate::Route> {
        Self::unfiltred()
        .into_iter()
        .filter(|x| x.1.mount_point == path && x.1.enabled)
        .map(|x| x.0)
        .collect()
    }
}

impl<T> RoutesCollection for T
where T: inventory::Collect,
AutoRoute: From<&'static T> {
    fn unfiltred() -> Vec<AutoRoute> {
        inventory::iter::<T>.into_iter()
        .map(|x| x.into())
        .collect()
    }
}

#[macro_export]
// TODO: only  pub and pub(crate) should be allowed? 
macro_rules! routes_inventory {
    () => {
        routes_inventory!(pub(crate));
    };
    ($x : vis) => {

        $x struct RoutesInventory ( //basicly clone of AutoRoute - each crate must have it's own to destinguish between crates
            &'static $crate::StaticRouteInfo,
            &'static $crate::auto_mount::AutoMountModuleHint,
        );
        impl RoutesInventory {
            pub fn new(route: &'static $crate::StaticRouteInfo,mod_hint: &'static $crate::auto_mount::AutoMountModuleHint) -> Self {
                RoutesInventory (route, mod_hint)
            }
        }
        impl From<&'static RoutesInventory> for $crate::auto_mount::AutoRoute {
            fn from (x: &RoutesInventory) -> $crate::auto_mount::AutoRoute {
                $crate::auto_mount::AutoRoute(x.0.into(), x.1)
            }
        }
        $crate::inventory::collect!(RoutesInventory);

        #[cfg(not(rocket_codegen_auto_mounting))]
        compile_error!("Auto mouting not configured properly! See documentation of this macro for more information.");
    }
}
