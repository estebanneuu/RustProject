#[macro_use] extern crate rocket;

#[get("/")]
fn root() -> String {
    let mut retour = String::new();
    retour.push_str("seems ok");
    retour
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root])

}