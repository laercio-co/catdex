// use actix_files::{Files, NamedFile};
use actix_files::Files;
// use actix_web::{web, App, HttpServer, Result};
use actix_web::{web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;
use serde_json::json;

// async fn index() -> Result<NamedFile> {
//     Ok(NamedFile::open("../static/index.html")?)
// }

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
    "project_name": "Catdex",
    "cats": [
        {
            "name": "Gato Um",
            "image_path": "/static/image/01.jpg"
        },
        {
            "name": "Gato Dois",
            "image_path": "/static/image/02.jpg"
        },
        {
            "name": "Gato Três",
            "image_path": "/static/image/03.jpg"
        }
    ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        // This function is guarded by a dir_source feature, which we enabled in Cargo.toml
        .register_templates_directory(".html", "../static/")
        .unwrap();
    /* To avoid recompiling the templates in each thread, we need a way to
    share this Handlebars instance across threads. To share states between
    threads, you can use the web::Data provided by Actix */
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "../static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
