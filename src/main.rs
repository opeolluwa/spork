/// A wrapper around dictionary API <https://api.dictionaryapi.dev/api/v2/entries/en/>
/// the request accept incoming request and parse it with the ApiRequest struct
/// the ApiRequest object can contain the following fields:
/// - keyword : the search term
/// - language : the target language of the key word
/// example : example
/// ```json
/// {"keyword": "axe", "language": "en"}
///```
//import axum and other dependencies
use axum::{
    extract::Json as Request,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get_service, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;
///the base url of the dictionary API
const DICTIONARY_API: &str = "https://api.dictionaryapi.dev/api/v2/entries";
#[derive(Debug, Serialize, Deserialize)]
struct License {
    name: String,
    url: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
    examples: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Phonetics {
    text: String,
    audio: String,
    #[serde(rename = "sourceUrl")]
    source_url: String,
    license: License,
}
#[derive(Debug, Serialize, Deserialize)]
struct DictionaryApiResponse {
    word: String,
    phonetic: String,
    phonetics: Vec<Phonetics>,
    meanings: Vec<Meaning>,
    license: License,
    #[serde(rename = "sourceUrls")]
    source_urls: Vec<String>,
}
///Api Request structure
#[derive(Debug, Serialize, Deserialize)]
struct ApiRequest {
    // language: String,
    keyword: String,
}

///API response data structure
#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    search_term: String,
    language: String,
    transcription: String,
}

///Api Response structure
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Vec<DictionaryApiResponse>,
}

//an handler to receive incoming request
async fn search(Request(request): Request<ApiRequest>) -> impl IntoResponse {
    //destructure the request
    let ApiRequest { keyword } = &request;
    let body = reqwest::get(format!("{}/{}/{}", &DICTIONARY_API, "en", keyword))
        .await
        .unwrap();
    //error handling
    let data = match body.json::<Vec<DictionaryApiResponse>>().await {
        Ok(val) => Some(val),
        _ => None,
    };
    //try to destructure the response
    let data: std::option::Option<Vec<DictionaryApiResponse>> = data;
    Json(data)
}

//the index route
// Include utf-8 file at **compile** time.
async fn _index() -> Html<&'static str> {
    Html(std::include_str!("../views/index.html"))
}

#[tokio::main]
async fn main() {
    //static file mounting
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service = get_service(
        ServeDir::new(assets_dir).append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    // build our application and mount the routes
    let app = Router::new()
        .fallback(static_files_service)
        // .route("/", get(index))
        .route("/api/search/", post(search));
    // fire up the server
       let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3456);

    let ip_address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Ignition started on http://{}", ip_address);
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
