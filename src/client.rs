extern crate reqwest;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use reqwest::header::AUTHORIZATION;
use serde::export::fmt::Debug;
use serde::de::DeserializeOwned;
use reqwest::{Response, Error};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse<T> {
    total_size: i32,
    done: bool,
    records: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    id: String,
    success: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    message: String,
    error_code: String,
    fields: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    id: String,
    issued_at: String,
    access_token: String,
    instance_url: String,
    signature: String,
    token_type: String,
}

#[derive(Debug)]
pub struct AccessToken {
    token_type: String,
    value: String,
    issued_at: String,
}

#[derive(Debug)]
pub struct Client {
    http_client: reqwest::Client,
    client_id: String,
    client_secret: String,
    login_endpoint: String,
    instance_url: Option<String>,
    access_token: Option<AccessToken>,
    reflesh_token: Option<String>,
    version: String,
}

impl Client {
    pub fn new(client_id: String, client_secret: String) -> Client {
        let http_client = reqwest::Client::new();
        return Client {
            http_client,
            client_id,
            client_secret,
            login_endpoint: "https://login.salesforce.com".to_string(),
            access_token: None,
            reflesh_token: None,
            instance_url: None,
            version: "v44.0".to_string(),
        }
    }

    pub fn login_with_credential(&mut self, username: String, password: String) {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = [
            ("grant_type", "password"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("username", username.as_str()),
            ("password", password.as_str()),
        ];
        let res: TokenResponse = self.http_client.post(token_url.as_str())
            .form(&params)
            .send()
            .unwrap()
            .json()
            .unwrap();

        self.access_token = Some(AccessToken {
            value: res.access_token,
            issued_at: res.issued_at,
            token_type: res.token_type,
        });
        self.instance_url = Some(res.instance_url);
    }

    pub fn query<A: Debug + DeserializeOwned>(&self, query: String) -> Result<QueryResponse<A>, Vec<ErrorResponse>> {
        let query_url = format!("{}/services/data/{}/query/", self.instance_url.as_ref().unwrap(), self.version);
        let params = vec![("q", query)];
        let mut res = self.get(query_url, params).unwrap();
        if res.status().is_success() {
            return Ok(res.json().unwrap());
        }
        return Err(res.json().unwrap());
    }

    pub fn create<A: Serialize>(&self, sobject_name: &str, params: A) -> Result<CreateResponse, Vec<ErrorResponse>> {
        let resource_url = format!("{}/services/data/{}/sobjects/{}", self.instance_url.as_ref().unwrap(), self.version, sobject_name);
        let mut res = self.post(resource_url, params).unwrap();

        if res.status().is_success() {
            return Ok(res.json().unwrap());
        }
        return Err(res.json().unwrap());
    }

    pub fn update<A: Serialize>(&self, sobject_name: &str, id: &str, params: A) -> Result<(), Vec<ErrorResponse>> {
        let resource_url = format!("{}/services/data/{}/sobjects/{}/{}", self.instance_url.as_ref().unwrap(), self.version, sobject_name, id);
        let mut res = self.patch(resource_url, params).unwrap();

        if res.status().is_success() {
            return Ok(());
        }
        return Err(res.json().unwrap());
    }

    pub fn destroy(&self, sobject_name: &str, id: &str) -> Result<(), Vec<ErrorResponse>> {
        let resource_url = format!("{}/services/data/{}/sobjects/{}/{}", self.instance_url.as_ref().unwrap(), self.version, sobject_name, id);
        let mut res = self.delete(resource_url).unwrap();

        if res.status().is_success() {
            return Ok(());
        }
        return Err(res.json().unwrap());
    }

    fn get(&self, url: String, params: Vec<(&str, String)>) -> Result<Response, Error> {
        return self.http_client.get(url.as_str())
            .headers(self.create_header())
            .query(&params)
            .send();
    }

    fn post<T: Serialize>(&self, url: String, params: T) -> Result<Response, Error> {
        return self.http_client.post(url.as_str())
            .headers(self.create_header())
            .json(&params)
            .send();
    }

    fn patch<T: Serialize>(&self, url: String, params: T) -> Result<Response, Error> {
        return self.http_client.patch(url.as_str())
            .headers(self.create_header())
            .json(&params)
            .send();
    }

    fn delete(&self, url: String) -> Result<Response, Error> {
        return self.http_client.delete(url.as_str())
            .headers(self.create_header())
            .send();
    }

    fn create_header(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.access_token.as_ref().unwrap().value).parse().unwrap());
        return headers;
    }
}