use super::rocket;
use rocket::local::Client;
use rocket::http::Status;
//TODO: compile fail on routes_inventory!() misuse?
//TODO: tests for other mounting styles

#[test]
fn auto_mount_all() {
    let rocket = rocket::ignite()
    .auto_mount_all::<crate::RoutesInventory>(); 
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, world!".into()));

    let mut response = client.get("/info").dispatch();
    assert_eq!(response.body_string(), Some("this is test web server".into()));

    let mut response = client.get("/user/about").dispatch();
    assert_eq!(response.body_string(), Some("current user is unnown".into()));

    let mut response = client.get("/user/logout").dispatch();
    assert_eq!(response.body_string(), Some("logged out!".into()));

    check_unmounted(&client,vec!["/about","/logout","/secret","/user/info","/lib_route"]);
}

#[test]
fn auto_mount_with_base() {
    let rocket = rocket::ignite()
    .auto_mount_with_base::<crate::RoutesInventory>("/user");
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/user/about").dispatch();
    assert_eq!(response.body_string(), Some("current user is unnown".into()));

    let mut response = client.get("/user/logout").dispatch();
    assert_eq!(response.body_string(), Some("logged out!".into()));

    check_unmounted(&client,vec!["/","/info","/about","/logout","/secret","/user/info","/lib_route"])
}

#[test]
fn repeated_mounting() {
    use rocket::auto_mount::RoutesCollection;

    let rocket = rocket::ignite()
    .auto_mount_all::<crate::RoutesInventory>()
    .mount("/legacy_user_api", crate::RoutesInventory::with_hint_mount_point("/user"));

    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/user/about").dispatch();
    assert_eq!(response.body_string(), Some("current user is unnown".into()));

    let mut response = client.get("/user/logout").dispatch();
    assert_eq!(response.body_string(), Some("logged out!".into()));

    let mut response = client.get("/legacy_user_api/about").dispatch();
    assert_eq!(response.body_string(), Some("current user is unnown".into()));

    let mut response = client.get("/legacy_user_api/logout").dispatch();
    assert_eq!(response.body_string(), Some("logged out!".into()));

    check_unmounted(&client,vec!["/about","/logout","/secret","/user/info","/legacy_user_api/info","/lib_route"]);
}

#[test]
fn routes_from_lib() {
    let rocket = rocket::ignite()
    .auto_mount_all::<crate::RoutesInventory>()
    .auto_mount_all::<auto_mount_lib::RoutesInventory>();
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, world!".into()));

    let mut response = client.get("/info").dispatch();
    assert_eq!(response.body_string(), Some("this is test web server".into()));

    let mut response = client.get("/lib_route").dispatch();
    assert_eq!(response.body_string(), Some("Hello from liblary!".into()));

    check_unmounted(&client,vec!["/about","/logout","/secret","/user/info"]);
}

#[test]
fn custom_filtering() {
    use rocket::auto_mount::RoutesCollection;

    let secret_routes: Vec<_> = crate::RoutesInventory::unfiltred()
    .into_iter()
    .filter(|x| !x.1.enabled)
    .map(|x| x.0)
    .collect();

    let rocket = rocket::ignite()
    .mount("/secret_routes", secret_routes);
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/secret_routes/secret").dispatch();
    assert_eq!(response.body_string(), Some("secret route!".into()));

    check_unmounted(&client,vec!["/secret_routes/","/secret_routes/info","/secret_routes/about","/secret_routes/logout","/about","/logout","/secret","/user/info","/lib_route"]);
}

fn check_unmounted(client: &Client, paths: Vec<&'static str>) {
    for path in paths.into_iter() {
        let response = client.get(path).dispatch();
        if response.status() != Status::NotFound {
            panic!("Route is mounted but should not! : {}", path)
        }
    }
}