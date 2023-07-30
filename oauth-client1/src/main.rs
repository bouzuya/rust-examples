use std::{borrow::Cow, collections::HashMap, env, io};

use anyhow::Context;
use oauth_client::Token;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
};

fn main() -> anyhow::Result<()> {
    // <https://oauth.net/core/1.0a/>
    // <https://docs.rs/oauth-client/0.8.0/oauth_client/index.html>

    // provider
    let request_token_url = env::var("REQUEST_TOKEN_URL")?;
    let user_authorization_url = env::var("USER_AUTHORIZATION_URL")?;
    let access_token_url = env::var("ACCESS_TOKEN_URL")?;

    // consumer
    let oauth_callback_url = env::var("OAUTH_CALLBACK_URL")?;
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;

    // get_request_token
    let consumer = Token::new(consumer_key, consumer_secret);
    let other_params = {
        let mut params = HashMap::new();
        params.insert(
            Cow::Borrowed("oauth_callback"),
            Cow::Borrowed(oauth_callback_url.as_str()),
        );
        params
    };
    let (header, _body) = oauth_client::authorization_header(
        "GET",
        request_token_url.as_str(),
        &consumer,
        None,
        Some(&other_params),
    );
    let client = Client::new();
    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&header)?);
        headers
    };
    let response = client.get(request_token_url).headers(headers).send()?;
    println!("{:?}", response.headers());
    let body = response.text()?;
    println!("{:?}", body);
    let mut parsed = HashMap::new();
    for p in body.split('&') {
        let (k, v) = p.split_once('=').context("key=value")?;
        parsed.insert(k.to_owned(), v.to_owned());
    }
    println!("{:?}", parsed);
    let oauth_token = parsed["oauth_token"].to_owned();
    let oauth_token_secret = parsed["oauth_token_secret"].to_owned();

    println!(
        "{user_authorization_url}?oauth_token={oauth_token}&oauth_callback={oauth_callback_url}"
    );

    let mut oauth_verifier = String::new();
    io::stdin().read_line(&mut oauth_verifier)?;
    let oauth_verifier = oauth_verifier.trim_end();

    // get_access_token
    let oauth_token = &oauth_token;
    let oauth_token_secret = &oauth_token_secret;
    let token = Token::new(oauth_token, oauth_token_secret);
    let other_params = {
        let mut params = HashMap::new();
        params.insert(
            Cow::Borrowed("oauth_verifier"),
            Cow::Borrowed(oauth_verifier),
        );
        params
    };
    let (header, _body) = oauth_client::authorization_header(
        "GET",
        access_token_url.as_str(),
        &consumer,
        Some(&token),
        Some(&other_params),
    );
    let client = Client::new();
    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&header)?);
        headers
    };
    let response = client.get(access_token_url).headers(headers).send()?;
    let body = response.text()?;
    println!("{:?}", body);
    let mut parsed = HashMap::new();
    for p in body.split('&') {
        let (k, v) = p.split_once('=').context("key=value")?;
        parsed.insert(k.to_owned(), v.to_owned());
    }
    println!("{:?}", parsed);
    Ok(())
}
