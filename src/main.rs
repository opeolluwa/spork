#![allow(dead_code, unused_variables)]
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
    response::Json as Response,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// SearchTerm Constructor
#[derive(Serialize, Deserialize, Debug)]
struct SearchTerm {
    keyword: String,
}

impl SearchTerm {
    fn new(keyword: String) -> SearchTerm {
        SearchTerm { keyword }
    }
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
async fn search(Request(request): Request<ApiRequest>) -> Response<ApiResponse> {
    //destructure the request
    let ApiRequest { keyword, language } = request;
    println!("search for {}", keyword);
    let data  = ResponseData{
        search_term: keyword,
        language: language,
        transcription: "some transcription goes here".to_string(),
    };

    let response = ApiResponse{
        success:true,
        message: format!("search result for {}", &keyword),
        data
    };
    Response(json!(response))
}

#[tokio::main]
async fn main() {
    // build our application and mount the routes
    let sample_keyword: SearchTerm = SearchTerm::new("rusty".to_string());
    println!("{:?}", sample_keyword);
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
