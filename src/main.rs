use axum::{
    extract::Path,
    handler::{get, post},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use rusoto_s3::{ListObjectsV2Request, ListObjectsV2Output, S3Client, S3};
use rusoto_signature::region::Region;
use serde_json::{to_value, Value};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                panic!("Parsing error: {}", e);
            }
        }
    };
}

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

fn filter_s3_folders(data: &ListObjectsV2Output) -> Vec<String> {
    let mut folders = HashSet::new();

    match &data.contents {
        Some(contents) => {
            contents.iter().for_each(|obj| {
                match &obj.key {
                    Some(key) => {
                        let mut base = key.split('/');
                        folders.insert(base.next().expect("s3 bucket formatted improperly"));
                        // println!("{}", key);
                    }
                    _ => {},
                }
            });
        }
        _ => {}
    }

    folders.iter().map(|key| key.to_string()).collect()
}

async fn root() -> Result<Html<String>, impl IntoResponse> {
    // TODO: Load all packages from s3
    let client = S3Client::new(Region::UsEast2);
    let request = ListObjectsV2Request {
        bucket: "koloni-pypi".into(),
        // delimiter: Some("/".into()),
        ..Default::default()
    };

    match client.list_objects_v2(request).await {
        Ok(result) => {
            println!("{:?}", filter_s3_folders(&result));
            let packages = filter_s3_folders(&result);
            let packages = ListPackages {
                packages: packages.iter().map(|name| {
                    Package {
                        name: name.into(),
                        href: format!("/simple/{}/", name)
                    }
                }).collect(),
            };
            let page = TEMPLATES.render("simple.html", &Context::from_serialize(&packages).unwrap()).unwrap();
            // println!("{}", page);

            Ok(Html(page))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{:?}", err))))
    }
}

async fn get_distribution(Path(distrib): Path<String>) -> Html<String> {
    let all_packages = Vec::new();

    // TODO: Load distrib packages from s3

    let packages = ListPackages {
        packages: all_packages,
    };
    let page = TEMPLATES.render("simple.html", &Context::from_serialize(&packages).unwrap()).unwrap();

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
