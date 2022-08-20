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
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
use std::net::SocketAddr;

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
    language: String,
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
    let ApiRequest { keyword, language } = &request;
    let body = reqwest::get(format!("{}/{}/{}", &DICTIONARY_API, language, keyword))
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

#[tokio::main]
async fn main() {
    // build our application and mount the routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World! " }))
        .route("/search", post(search));

    // fire up the server
    let ip_address = SocketAddr::from(([127, 0, 0, 1], 3456));
    println!("Ignition started on http://{}", ip_address);
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
