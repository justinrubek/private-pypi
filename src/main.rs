use axum::{
    extract::Path,
    handler::{get, post},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tera::{Context, Result, Tera, try_get_value};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                panic!("Parsing error: {}", e);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

/*
pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}
*/

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/simple/", get(root).post(create_package)) // GET POST /simple/
        .route("/simple/:distrib", get(get_distribution)) // GET /simple/<distrib>/
        .route("/simple/:distrib/:filename", get(get_package)); // GET /simple/<distrib>/<filename>

    let addr = SocketAddr::from(([127, 0, 0, 1], 2000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(Serialize)]
struct Package {
    name: String,
    href: String,
}

#[derive(Serialize)]
struct ListPackages {
    packages: Vec<Package>,
}

async fn root() -> Html<String> {
    let all_packages = Vec::new();

    // TODO: Load all packages from s3

    let packages = ListPackages {
        packages: all_packages,
    };
    let page = TEMPLATES.render("simple.html", &Context::from_serialize(&packages).unwrap()).unwrap();
    println!("{}", page);

    Html(page)
}

async fn get_distribution(Path(distrib): Path<String>) -> Html<String> {
    let all_packages = Vec::new();

    // TODO: Load distrib packages from s3

    let packages = ListPackages {
        packages: all_packages,
    };
    let page = TEMPLATES.render("simple.html", &Context::from_serialize(&packages).unwrap()).unwrap();
    println!("{}", page);

    Html(page)
}

async fn get_package() -> impl IntoResponse {
    (StatusCode::OK, "package")
}

async fn create_package(Json(payload): Json<CreatePackage>) -> impl IntoResponse {
    (StatusCode::CREATED, Json("created"))
}

#[derive(Deserialize)]
struct CreatePackage {
    name: String,
}
