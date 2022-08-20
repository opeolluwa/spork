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
use axum::{routing::get, Router};
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

///Api Response structure
struct ApiResponse {
    search_term: String,
    transcription: String,
    antonyms: Vec<String>,
    synonyms: Vec<String>,
}

//an handler to receive incoming request
async fn accept_search_term() {}
#[tokio::main]
async fn main() {
    // build our application with a single route
    let sample_keyword: SearchTerm = SearchTerm::new("rusty".to_string());
    println!("{:?}", sample_keyword);
    let app = Router::new().route("/", get(|| async { "Hello, World! " }));

    // fire up the server
    let ip_address = SocketAddr::from(([127, 0, 0, 1], 3456));
    println!("Ignition started on http://{}", ip_address);
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
