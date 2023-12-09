use rocket::Route;
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub fn swagger_doc_config() -> impl Into<Vec<Route>> {
    return make_rapidoc(&RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../api/openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn swagger_ui() -> impl Into<Vec<Route>> {
    return make_swagger_ui(&SwaggerUIConfig {
        url: "../api/openapi.json".to_owned(),
        ..Default::default()
    });
}