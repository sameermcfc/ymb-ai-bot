use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use axum::{
     http::StatusCode, middleware::Next, response::Response, extract::{State, Request}
};
use axum_extra::headers::HeaderMap;
use std::collections::HashSet;


#[derive(Debug, Deserialize, Clone)]
pub struct FirebaseKey {
    pub kty: String,
    #[serde(rename = "use")]
    pub use_: String, // `use` is a reserved keyword in Rust, so we rename it
    pub alg: String,
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Deserialize)]
pub struct FirebaseKeys {
    pub keys: Vec<FirebaseKey>, // A list of FirebaseKey objects
}

pub async fn fetch_firebase_keys() -> Result<HashMap<String, FirebaseKey>, reqwest::Error> {
    let client = Client::new();
    let url = "https://firebaseappcheck.googleapis.com/v1/jwks";
    let response = client.get(url).send().await?;
    let firebase_keys: FirebaseKeys = response.json().await?;
    // Convert the Vec<FirebaseKey> into a HashMap using the `kid` field as the key
    let key_map = firebase_keys
        .keys
        .into_iter()
        .map(|key| (key.kid.clone(), key)) // Use `kid` as the key
        .collect::<HashMap<String, FirebaseKey>>();

    Ok(key_map)
}

#[derive(Debug, Deserialize)]
pub struct AppCheckClaims {
    pub aud: Vec<String>, // Audience
    pub iss: String, // Issuer
    pub sub: String, // Subject
    pub exp: u64,    // Expiration time
    pub iat: u64,    // Issued at
    // Add other fields if needed
}

pub fn verify_app_check_token(
    token: &str,
    public_keys: &HashMap<String, FirebaseKey>,
) -> Result<AppCheckClaims, String> {
    // Decode the header to get the "kid" field
    println!("Starting to verify App Check token...");
    let header = decode_header(token).map_err(|err| {
        let msg = format!("Failed to decode token header: {}", err);
        println!("{}", msg); // Debugging log
        msg
    })?;
    println!("Token header successfully decoded: {:?}", header);

    let kid = header.kid.ok_or_else(|| {
        let msg = "Token header does not contain 'kid' field".to_string();
        println!("{}", msg); // Debugging log
        msg
    })?;
    println!("Extracted 'kid' from token header: {}", kid);

    // Get the matching public key
    let firebase_key = public_keys.get(&kid).ok_or_else(|| {
        let msg = format!("No matching key found for 'kid': {}", kid);
        println!("{}", msg); // Debugging log
        msg
    })?;
    println!("Found matching public key for 'kid': {}", kid);

    // Construct the DecodingKey using the RSA components (`n` and `e`)
    let decoding_key = DecodingKey::from_rsa_components(&firebase_key.n, &firebase_key.e)
        .map_err(|err| {
            let msg = format!("Failed to construct decoding key: {}", err);
            println!("{}", msg); // Debugging log
            msg
        })?;
    println!("Successfully constructed decoding key.");

    // Set validation parameters
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["projects/853958425291"]); // Replace with your Firebase App's App ID
    let mut issuers = HashSet::new();
    issuers.insert("https://firebaseappcheck.googleapis.com/853958425291".to_string());
    validation.iss = Some(issuers);
    println!("Validation parameters set: {:?}", validation);

    // Decode and validate the token
    let token_data = decode::<AppCheckClaims>(token, &decoding_key, &validation)
        .map_err(|err| {
            let msg = format!("Failed to decode or validate token: {}", err);
            println!("{}", msg); // Debugging log
            msg
        })?;
    println!("Token successfully decoded and validated: {:?}", token_data);

    Ok(token_data.claims)
}




pub async fn app_check_middleware(
    headers: HeaderMap,           // Extract all headers
    state: State<AppState>,         // Shared state containing public keys
    req: Request,                 // Incoming request
    next: Next,                   // Next handler/middleware
) -> Result<Response, StatusCode> {
    // Extract the X-Firebase-AppCheck header
    if let Some(app_check_token) = headers
        .get("X-Firebase-AppCheck")
        .and_then(|value| value.to_str().ok())
    {   
        println!("{}", app_check_token.to_string());
        // Verify the App Check token
        match verify_app_check_token(app_check_token, &state.public_keys) {
            Ok(claims) => {
                // You can log or inspect the claims here
                println!("Verified App Check claims: {:?}", claims);

                // Pass the request to the next middleware or handler
                Ok(next.run(req).await)
            }
            Err(err) => {
                println!("Invalid App Check token: {}", err);
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    } else {
        // If the header is missing, return 401 Unauthorized
        println!("Missing X-Firebase-AppCheck header");
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Clone)]
pub struct AppState {
    pub public_keys: HashMap<String, FirebaseKey>,
}
