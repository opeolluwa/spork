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
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    examples: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    partOfSpeech: String,
    definition: Vec<Definition>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Phonetics {
    text: String,
    audio: String,
    sourceUrl: String,
    license: License,
}
#[derive(Debug, Serialize, Deserialize)]
struct DictionaryApiResponse {
    word: String,
    phonetic: String,
    phonetics: Vec<Phonetics>,
    meanings: Vec<Meaning>,
    license: License,
    sourceUrls: Vec<String>,
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
    // antonyms: Vec<String>,
    // synonyms: Vec<String>,
}

///Api Response structure
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: ResponseData,
}

//an handler to receive incoming request
async fn search(Request(request): Request<ApiRequest>) -> impl IntoResponse {
    //destructure the request
    let ApiRequest { keyword, language } = &request;
    let client = reqwest::Client::new();
    println!("{}/{}/{}", &DICTIONARY_API, language, keyword);
    let body = client
        .get("https://api.dictionaryapi.dev/api/v2/entries/en/absolute")
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();
    //destructure the response
    println!("{:#?}", body.json::<Vec<DictionaryApiResponse>>().await);

    let data = ResponseData {
        search_term: keyword.clone(),
        language: language.clone(),
        transcription: "some transcription goes here".to_string(),
    };

    let response = ApiResponse {
        success: true,
        message: format!("search result for {}", &keyword),
        data,
    };
    Json(response)
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
