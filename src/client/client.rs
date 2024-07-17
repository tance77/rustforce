use crate::access_token::AccessToken;
use crate::errors::Error;
use crate::responses::error_response::ErrorResponse;
use crate::responses::token_response::TokenResponse;
use crate::xml::Xml;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use reqwest::{Response, Url};
use serde::Serialize;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Default)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub login_endpoint: String,
    pub instance_url: Option<String>,
    pub access_token: Option<AccessToken>,
    pub refresh_token: Option<String>,
    pub version: String,
    pub secret_required: bool,
}

impl Client {
    pub fn default() -> Self {
        Client::new()
    }
    pub fn new() -> Client {
        let http_client = reqwest::Client::new();
        Client {
            http_client,
            client_id: None,
            client_secret: None,
            login_endpoint: "https://login.salesforce.com".to_string(),
            access_token: None,
            instance_url: None,
            refresh_token: None,
            secret_required: true,
            version: "v60.0".to_string(),
        }
    }
    pub fn set_login_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.login_endpoint = endpoint.to_string();
        self
    }

    pub fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    pub fn set_instance_url(&mut self, instance_url: &str) -> &mut Self {
        self.instance_url = Some(instance_url.to_string());
        self
    }

    pub fn get_instance_url(&mut self) -> String {
        self.instance_url.clone().unwrap_or_default()
    }

    pub fn set_refresh_token(&mut self, refresh_token: &str) -> &mut Self {
        self.refresh_token = Some(refresh_token.to_string());
        self
    }

    pub fn set_secret_required(&mut self, secret_required: bool) -> &mut Self {
        self.secret_required = secret_required;
        self
    }

    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    pub fn set_client_secret(&mut self, client_secret: &str) -> &mut Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }

    /// Set Access token if you've already obtained one via one of the OAuth2 flows
    pub fn set_access_token(
        &mut self,
        access_token: String,
        issued_at: String,
        token_type: String,
    ) -> &mut Self {
        self.access_token = Some(AccessToken {
            token_type,
            value: access_token,
            issued_at,
        });
        self
    }

    pub fn get_access_token(&mut self) -> String {
        return match &self.access_token {
            Some(token) => {
                format!("{}", token.value)
            }
            None => "".to_string(),
        };
    }

    pub async fn get_identity(&mut self, identity_url: String) -> Result<String, Error> {
        let res = self.get(identity_url, vec![]).await?;
        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn ensure_refresh(&mut self) -> Result<&mut Self, Error> {
        if self.access_token.is_none() {
            return Ok(self);
        }

        let timestamp_ms = self
            .access_token
            .clone()
            .unwrap()
            .issued_at
            .parse::<u64>()
            .unwrap();
        let seconds = timestamp_ms / 1000;
        let nanos = (timestamp_ms % 1000) * 1_000_000; // Convert remainder to nanoseconds

        let given_time = UNIX_EPOCH + Duration::new(seconds, nanos as u32);

        let two_hours = Duration::from_secs(2 * 60 * 60); // 2 hours in seconds
        let modified_time = given_time + two_hours;

        let current_time = SystemTime::now();

        if current_time > modified_time {
            println!("Access Token Expired Refreshing.");
            Ok(self.refresh().await?)
        } else {
            Ok(self)
        }
    }

    fn get_refresh_params(&self) -> Vec<(String, String)> {
        let refresh_token = self.refresh_token.clone().unwrap_or_else(|| "".to_string());
        let client_id = self.client_id.clone().unwrap_or_else(|| "".to_string());
        let client_secret = self.client_secret.clone().unwrap_or_else(|| "".to_string());

        let mut params = vec![
            ("grant_type".to_string(), "refresh_token".to_string()),
            ("client_secret".to_string(), client_secret),
            ("refresh_token".to_string(), refresh_token),
            ("client_id".to_string(), client_id),
        ];

        if self.secret_required {
            params.push((
                "client_secret".to_string(),
                self.client_secret.clone().unwrap_or_else(|| "".to_string()),
            ));
        }
        params
    }

    /// This will fetch an access token when provided with a refresh token
    pub async fn refresh(&mut self) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = self.get_refresh_params();

        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(e) => return Err(e.into()),
        };

        if !res.status().is_success() {
            let error_response = res.json().await?;
            return Err(Error::TokenError(error_response));
        }

        let response: TokenResponse = res.json().await?;
        let token_type = response.token_type.unwrap_or_else(|| "".to_string());
        self.set_access_token(response.access_token, response.issued_at, token_type);
        self.instance_url = Some(response.instance_url);
        Ok(self)
    }

    /// Login to Salesforce with username and password
    pub async fn login_with_credential(
        &mut self,
        username: String,
        password: String,
    ) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/oauth2/token", self.login_endpoint);
        let params = [
            ("grant_type", "password"),
            ("client_id", self.client_id.as_ref().unwrap()),
            ("client_secret", self.client_secret.as_ref().unwrap()),
            ("username", username.as_str()),
            ("password", password.as_str()),
        ];
        let res = self
            .http_client
            .post(token_url.as_str())
            .form(&params)
            .send()
            .await?;

        if !(res.status().is_success()) {
            let error_response = res.json().await?;
            return Err(Error::TokenError(error_response));
        }

        let response: TokenResponse = res.json().await?;
        let token_type = response.token_type.unwrap_or_else(|| "".to_string());
        self.set_access_token(response.access_token, response.issued_at, token_type);
        self.instance_url = Some(response.instance_url);
        Ok(self)
    }

    pub async fn login_by_soap(
        &mut self,
        username: String,
        password: String,
    ) -> Result<&mut Self, Error> {
        let token_url = format!("{}/services/Soap/u/{}", self.login_endpoint, self.version);
        let body = Xml::create_login_envelope(&username, &password);
        let res = self
            .http_client
            .post(token_url.as_str())
            .body(body)
            .header("Content-Type", "text/xml")
            .header("SOAPAction", "\"\"")
            .send()
            .await?;
        if res.status().is_success() {
            let body_response = res.text().await?;
            self.access_token = match Xml::extract("sessionId", body_response.as_str()) {
                Some(t) => {
                    let issued_at =
                        Xml::extract("serverTimestamp", body_response.as_str()).unwrap_or_default();
                    Some(AccessToken {
                        value: t,
                        issued_at,
                        token_type: "Bearer".to_string(),
                    })
                }
                None => None,
            };
            self.instance_url = Xml::extract("serverUrl", body_response.as_str());
            Ok(self)
        } else {
            let body_response = res.text().await?;
            let error_message =
                Xml::extract("faultstring", body_response.as_str()).unwrap_or_default();
            let error_code = Xml::extract("faultcode", body_response.as_str()).unwrap_or_default();

            Err(Error::LoginError(ErrorResponse {
                message: error_message,
                error_code,
                fields: None,
            }))
        }
    }

    pub async fn rest_get_fulluri(&mut self, uri: &str) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/services/apexrest/{}",
            self.instance_url.as_ref().unwrap(),
            uri
        );
        let parsed = Url::parse(&resource_url).unwrap();
        // Some ownership absurdity for string refs accessed through iterators with collect
        let hash_query: HashMap<_, _> = parsed.query_pairs().into_owned().collect();
        let params_string: Vec<(String, String)> = hash_query
            .keys()
            .map(|k| (String::from(k), String::from(&hash_query[k])))
            .collect();
        let params: Vec<(&str, &str)> = params_string
            .iter()
            .map(|&(ref x, ref y)| (&x[..], &y[..]))
            .collect();
        let path: String = parsed.path().to_string();
        self.rest_get(path, params).await
    }

    pub async fn rest_get(
        &mut self,
        path: String,
        params: Vec<(&str, &str)>,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header(vec![])?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_post<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_patch<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_put<T: Serialize>(
        &mut self,
        path: String,
        params: T,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .put(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn rest_delete(&mut self, path: String) -> Result<Response, Error> {
        self.refresh().await?;

        let url = format!("{}{}", self.instance_url.as_ref().unwrap(), path);
        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header(vec![])?)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn get(
        &mut self,
        url: String,
        params: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header(vec![])?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn get_raw(
        &mut self,
        url: &str,
        additional_headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let mut headers = self.create_header(additional_headers)?;
        headers.remove("Accept");
        let res = self.http_client.get(url).headers(headers).send().await?;
        Ok(res)
    }

    pub async fn post<T: Serialize>(
        &mut self,
        url: String,
        params: T,
        headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(headers)?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn post_raw_buffer(
        &mut self,
        url: String,
        body: Vec<u8>,
        headers: Vec<(String, String)>,
    ) -> Result<Response, Error> {
        self.refresh().await?;

        let res = self
            .http_client
            .post(url)
            .headers(self.create_header(headers)?)
            .body(body)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn put(&mut self, url: String, buffer: Vec<u8>) -> Result<Response, Error> {
        self.refresh().await?;

        let mut headers = self.create_header(vec![])?;
        headers.insert("Content-Type", "text/csv".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        let res = self
            .http_client
            .put(url.as_str())
            .headers(headers)
            .body(buffer)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn patch<T: Serialize>(&mut self, url: String, params: T) -> Result<Response, Error> {
        self.refresh().await?;

        let res = self
            .http_client
            .patch(url.as_str())
            .headers(self.create_header(vec![])?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn delete(&mut self, url: String) -> Result<Response, Error> {
        self.refresh().await?;

        let res = self
            .http_client
            .delete(url.as_str())
            .headers(self.create_header(vec![])?)
            .send()
            .await?;
        Ok(res)
    }

    fn create_header(&self, additional_headers: Vec<(String, String)>) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        let auth_value = format!(
            "Bearer {}",
            self.access_token.as_ref().ok_or(Error::NotLoggedIn)?.value
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);

        //Default header
        headers.insert("Accept", HeaderValue::from_str("application/json")?);

        for (key, value) in additional_headers {
            let header_name = match key.parse::<HeaderName>() {
                Ok(name) => name,
                Err(_) => return Err(Error::DeserializeError("Invalid Header Name".to_string())), // Replace with appropriate error handling
            };

            let header_value = match HeaderValue::from_str(&value) {
                Ok(value) => value,
                Err(_) => return Err(Error::DeserializeError("Invalid Header Value".to_string())), // Replace with appropriate error handling
            };

            //delete duplicates
            headers.remove(key);

            headers.insert(header_name, header_value);
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_instance_url() {
        let mut client = Client::new();
        let url = "https://example.com";
        client.set_instance_url(url);
        assert_eq!(Some(url.to_string()), client.instance_url);
    }

    #[tokio::test]
    async fn test_get_instance_url() {
        let mut client = Client::new();
        let url = "https://example.com";
        client.set_instance_url(url);
        assert_eq!(url, client.get_instance_url());
    }

    #[tokio::test]
    async fn test_set_access_token() {
        let mut client = Client::new();
        let token_value = "MySuperSecretToken";
        client.set_access_token(token_value.to_string(), "".to_string(), "".to_string());
        assert_eq!(token_value, client.get_access_token());
    }

    #[tokio::test]
    async fn test_create_refresh_token_params_no_client_secret() {
        // Attempting to refresh without a token should return without error
        let mut client = Client::new();
        client.set_secret_required(false);
        client.set_client_id("Bulma");
        client.set_refresh_token("Goku");

        client.get_refresh_params();

        assert_eq!(client.get_refresh_params().len(), 3);
    }

    #[tokio::test]
    async fn test_create_refresh_token_params() {
        // Attempting to refresh without a token should return without error
        let mut client = Client::new();
        client.set_client_id("Bulma");
        client.set_refresh_token("Goku");

        client.get_refresh_params();

        assert_eq!(client.get_refresh_params().len(), 4);
    }

    // #[tokio::test]
    // async fn can_set_token_on_successful_credential_login() {
    //     let mut client = Client::new();
    //     let token_response = TokenResponse {
    //         access_token: "PowerLevel9000".to_string(),
    //         token_type: Some("Saiyan".to_string()),
    //         issued_at: "2019-10-01 00:00:00".to_string(),
    //         instance_url: "https://dbz.salesforce.com".to_string(),
    //         id: "Goku".to_string(),
    //         signature: "Kamehameha".to_string(),
    //     };
    //
    //     client.set_credentials_from_token_response(token_response);
    //
    //     let token = client.access_token.unwrap();
    //
    //     assert_eq!("PowerLevel9000", token.value);
    //     assert_eq!("2019-10-01 00:00:00", token.issued_at);
    //     assert_eq!("Saiyan", token.token_type);
    //     assert_eq!("https://dbz.salesforce.com", client.instance_url.unwrap());
    // }
}
