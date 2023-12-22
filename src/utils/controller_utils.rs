use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;

pub trait BaseController {
    fn routes() -> (Vec<Route>, OpenApi);
}