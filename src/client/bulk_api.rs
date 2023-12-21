use crate::client::client::Client;
use crate::errors::Error;
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default)]
pub struct BulkAPi {
    pub client: Client,
}

impl BulkAPi {
    pub fn new(client: Client) -> Self {
        BulkAPi { client }
    }

    pub fn base_path(&self) -> String {
        let instance_url = match self.client.instance_url {
            Some(ref url) => url,
            None => panic!("No instance url found"),
        };

        let version = &self.client.version[1..];
        format!("{}/services/async/{}", instance_url, version)
    }

    pub async fn create_job<T: Serialize>(&mut self, params: T) -> Result<Response, Error> {
        let resource_url = format!("{}/job", self.base_path());
        let headers = self.get_auth_headers();
        self.client.post(resource_url, params, headers).await
    }

    pub async fn add_batch_job(&mut self, job_id: &str, csv: Vec<u8>) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), "text/csv".to_string()));
        self.client.post(resource_url, csv, headers).await
    }

    pub async fn get_batch_for_job(
        &mut self,
        job_id: &str,
        content_type: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), content_type.to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    pub async fn get_batch_result_list(
        &mut self,
        job_id: &str,
        batch_id: &str,
        content_type: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result",
            self.base_path(),
            job_id,
            batch_id
        );
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), content_type.to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    pub async fn get_batch_result(
        &mut self,
        job_id: &str,
        batch_id: &str,
        result_id: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result/{}",
            self.base_path(),
            job_id,
            batch_id,
            result_id
        );
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), "application/xml".to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    pub async fn abort_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path(), job_id);
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), "application/json".to_string()));

        let mut params = HashMap::new();
        params.insert("state", "Aborted");

        self.client.post(resource_url, params, headers).await
    }

    fn get_auth_headers(&self) -> Vec<(String, String)> {
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
        ];
        return headers;
    }
}
