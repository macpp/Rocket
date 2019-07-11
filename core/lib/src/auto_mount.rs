use std::sync::atomic::{AtomicBool,Ordering};
#[derive(Debug)]
pub struct AutoMountHint {
    pub mount_point: &'static str,
    pub enabled: bool,
}

pub mod __default_auto_mount_hint {
    use super::*;
    pub static __ROCKED_AUTO_MOUNT_HINT : AutoMountHint = AutoMountHint {mount_point: "/", enabled: true};
}
pub struct AutoRoute(pub crate::Route,pub &'static AutoMountHint);

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
        if !AUTO_MOUNING_SUPPORTED.load(Ordering::SeqCst) {
            panic!("Auto mounting not supported on this platform!")
        }
        inventory::iter::<T>.into_iter()
        .map(|x| x.into())
        .collect()
    }
}

static AUTO_MOUNING_SUPPORTED: AtomicBool = AtomicBool::new(false);

#[ctor::ctor]
fn auto_route() {
    AUTO_MOUNING_SUPPORTED.store(true, Ordering::SeqCst);
}

/// Allows to configure behavior of [auto_mount()](Rocket::auto_mount) for all routes in module
///
/// # Example - set base path for all routes in module
///
/// ```rust
/// # #![feature(proc_macro_hygiene, decl_macro)]
/// # #[macro_use] extern crate rocket;
/// use rocket::{auto_mount_hint,get};
///
/// mod secret_routes {
///     auto_mount_hint!("/foo");
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
/// use rocket::{auto_mount_hint,get};
///
/// mod secret_routes {
///     auto_mount_hint!(enabled=false);
///
///     // this route will not be mounted by auto_mount()
///     #[get("/secret")]
///     fn secret() -> &'static str {
///         "secret route"
///     }
/// }
/// ```
#[macro_export] 
macro_rules! auto_mount_hint {
    ($l:literal) => {
        use $crate::auto_mount::AutoMountHint;
        static __ROCKED_AUTO_MOUNT_HINT : AutoMountHint = AutoMountHint {mount_point: $l, enabled: true};
    };
    ($l:literal,enabled=$enab: ident) => {
        use $crate::auto_mount::AutoMountHint;
        static __ROCKED_AUTO_MOUNT_HINT : AutoMountHint = AutoMountHint {mount_point: $l, enabled: $enab};
    };
    (enabled=$enab: ident) => {
        use $crate::auto_mount::AutoMountHint;
        static __ROCKED_AUTO_MOUNT_HINT : AutoMountHint = AutoMountHint {mount_point: "/", enabled: $enab};
    }
}


#[macro_export]
macro_rules! routes_inventory {
    () => {
        routes_inventory!(pub(crate));
    };
    ($x : vis) => {

        $x struct RoutesInventory ( //basicly clone of AutoRoute - each crate must have it's own to destinguish between crates
            &'static $crate::StaticRouteInfo,
            &'static $crate::auto_mount::AutoMountHint,
        );
        impl RoutesInventory {
            pub fn new(route: &'static $crate::StaticRouteInfo,mod_hint: &'static $crate::auto_mount::AutoMountHint) -> Self {
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
