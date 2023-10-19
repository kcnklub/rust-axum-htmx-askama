use askama::Template;
use axum::{
    extract::Form,
    response::Html,
    routing::{get, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/contact/1/edit", get(edit_contact))
        .route("/contact/1", put(update_contact))
        .route("/contact/1", get(root));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let root = Root {
        first_name: "default",
        last_name: "default",
        email: "default",
    };
    Html(root.render().unwrap())
}

async fn edit_contact() -> Html<String> {
    let edit = Edit {};
    Html(edit.render().unwrap())
}

#[derive(Debug, serde::Deserialize)]
struct UpdateContact {
    first_name: String,
    last_name: String,
    email: String,
}

async fn update_contact(input: Form<UpdateContact>) -> Html<String> {
    let edit = Root {
        first_name: input.first_name.as_str(),
        last_name: input.last_name.as_str(),
        email: input.email.as_str(),
    };
    Html(edit.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Root<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
}

#[derive(Template)]
#[template(path = "edit.html")]
struct Edit;
