use crate::client::client::Client;
use crate::errors::Error;
use crate::responses::bulk_api_v1_response::BulkApiV1CreateResponse;
use crate::responses::error_response::ErrorResponse;
use log::debug;
use serde::Serialize;

pub struct BulkAPi {
    client: Client,
}

impl BulkAPi {
    pub fn new(client: Client) -> Self {
        BulkAPi { client }
    }

    pub fn base_path(&self) -> String {
        format!(
            "{}/services/async/{}",
            self.client.instance_url.as_ref().unwrap(),
            &self.client.version[1..]
        )
    }

    pub async fn create_job<T: Serialize>(
        &mut self,
        params: T,
    ) -> Result<BulkApiV1CreateResponse, Error> {
        let resource_url = format!("{}/job", self.base_path());
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
        ];
        let res = self.client.post(resource_url, params, headers).await?;

        if res.status().is_success() {
            let res_bytes = res.bytes().await?;
            let res_text = std::str::from_utf8(&res_bytes).unwrap();
            debug!("{:?}", res_text);
            let parsed = serde_json::from_slice(&res_bytes).unwrap();
            Ok(parsed)
        } else {
            let res_bytes = res.bytes().await?;
            let parsed: ErrorResponse = serde_json::from_slice(&res_bytes).unwrap();
            Err(Error::DescribeError(parsed))
        }
    }

    pub async fn add_batch_job(&mut self, job_id: &str, csv: Vec<u8>) -> Result<String, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
        ];
        let res = self.client.post(resource_url, csv, headers).await?;

        if res.status().is_success() {
            let res_bytes = res.bytes().await?;
            let res_text = std::str::from_utf8(&res_bytes).unwrap();
            debug!("{:?}", res_text);

            let parsed = serde_json::from_slice(&res_bytes).unwrap();
            Ok(parsed)
        } else {
            let res_bytes = res.bytes().await?;
            let res_text = std::str::from_utf8(&res_bytes).unwrap();
            debug!("{:?}", res_text);

            let parsed: ErrorResponse = serde_json::from_slice(&res_bytes).unwrap();
            Err(Error::DescribeError(parsed))
        }
    }

    pub async fn get_batch_for_job(
        &mut self,
        job_id: &str,
        content_type: &str,
    ) -> Result<String, Error> {
        let resource_url = format!("{}/job/{}/batch", self.base_path(), job_id);
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
            ("Accept".to_string(), content_type.to_string()),
        ];
        let res = self.client.get_raw(&resource_url, headers).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn get_batch_result_list(
        &mut self,
        job_id: &str,
        batch_id: &str,
        content_type: &str,
    ) -> Result<String, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result",
            self.base_path(),
            job_id,
            batch_id
        );
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
            ("Accept".to_string(), content_type.to_string()),
        ];
        let res = self.client.get_raw(&resource_url, headers).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn get_batch_result(
        &mut self,
        job_id: &str,
        batch_id: &str,
        result_id: &str,
    ) -> Result<String, Error> {
        let resource_url = format!(
            "{}/job/{}/batch/{}/result/{}",
            self.base_path(),
            job_id,
            batch_id,
            result_id
        );
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
            ("Accept".to_string(), "application/xml".to_string()),
        ];
        let res = self.client.get_raw(&resource_url, headers).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn abort_job(&mut self, job_id: &str) -> Result<String, Error> {
        let resource_url = format!("{}/job/{}", self.base_path(), job_id);
        let headers = vec![
            //X-SFDC-Session is needed for API v1 we can just pass it our access token
            (
                "X-SFDC-Session".to_string(),
                self.client.access_token.as_ref().unwrap().value.clone(),
            ),
            ("Content-Type".to_string(), "application/json".to_string()),
        ];

        let params: Vec<String> = Vec::new();

        let res = self.client.post(resource_url, params, headers).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }
}
