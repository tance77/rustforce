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

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_create.htm
     **/
    pub async fn create_job<T: Serialize>(&mut self, params: T) -> Result<Response, Error> {
        let resource_url = format!("{}/job", self.base_path());
        let headers = self.get_auth_headers();
        self.client.post(resource_url, params, headers).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_quickstart_add_batch.htm
     */
    pub async fn add_batch_job(&mut self, job_id: &str, csv: Vec<u8>) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), "text/csv".to_string()));
        self.client
            .post_raw_buffer(resource_url, csv, headers)
            .await
    }
    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_quickstart_check_status.htm
     */

    pub async fn get_batch(&mut self, job_id: &str, batch_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch/{}/", self.base_path(), job_id, batch_id);
        let headers = self.get_auth_headers();
        self.client.get_raw(&resource_url, headers).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_close.htm
     **/
    pub async fn close_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path(), job_id);
        let headers = self.get_auth_headers();
        let mut params = HashMap::new();
        params.insert("state", "Closed");
        self.client.post(resource_url, params, headers).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_get_details.htm
     **/
    pub async fn get_job_details(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}", self.base_path(), job_id);
        let headers = self.get_auth_headers();
        self.client.get_raw(&resource_url, headers).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_batches_get_info_all.htm
     **/
    pub async fn get_batches(
        &mut self,
        job_id: &str,
        content_type: &str,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let mut headers = self.get_auth_headers();
        headers.push(("Content-Type".to_string(), content_type.to_string()));
        self.client.get_raw(&resource_url, headers).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_batches_get_results.htm
     */
    pub async fn get_result_list(
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

    /**
     * Scroll down to "Retrieve the Results".
     * I don't really know why this endpoint isn't documented better
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_code_curl_walkthrough_pk_chunking.htm
     */

    pub async fn get_result(
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

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/asynch_api_jobs_abort.htm
     **/
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
