use crate::client::client::Client;
use crate::errors::Error;
use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default)]
pub struct BulkApiV2 {
    pub client: Client,
}

impl BulkApiV2 {
    pub fn new(client: Client) -> Self {
        BulkApiV2 { client }
    }

    fn base_path(&self) -> String {
        let instance_url = self.client.instance_url.as_ref().unwrap();
        let version = &self.client.version;
        format!("{}/services/data/{}", instance_url, version)
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/create_job.htm
     **/
    pub async fn create_job<T: Serialize>(&mut self, params: T) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest", self.base_path());
        self.client.post(resource_url, params, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/upload_job_data.htm
     **/
    pub async fn upload_job_data(&mut self, job_id: &str, csv: Vec<u8>) -> Result<String, Error> {
        let resource_url = format!("{}/jobs/ingest/{}/batches", self.base_path(), job_id);
        let res = self.client.put(resource_url, csv).await?;

        if res.status().is_success() {
            Ok("Created".to_string())
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_all_jobs.htm
     **/
    pub async fn get_all_jobs(&mut self) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/", self.base_path());
        self.client.get(resource_url, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm
     **/
    pub async fn get_job_info(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        self.client.get(resource_url, vec![]).await
    }

    /**
    * These share the same endpoint, but different result sets. May want to split into two?

    * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_successful_results.htm
    * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_failed_results.htm
    **/

    pub async fn get_job_records(
        &mut self,
        job_id: &str,
        result_set: &str,
    ) -> Result<Response, Error> {
        // NOTE: RESULT_SET IS ONE OF successfulResults, failedResults, unprocessedrecords
        let resource_url = format!("{}/jobs/ingest/{}/{}", self.base_path(), job_id, result_set);
        self.client.get_raw(&resource_url, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/abort_job.htm
     **/
    pub async fn abort_job(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        let mut params = HashMap::new();
        params.insert("state", "Aborted");
        self.client.patch(resource_url, params).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/close_job.htm
     **/
    pub async fn set_upload_state<T: Serialize>(
        &mut self,
        job_id: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        self.client.patch(resource_url, params).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_asynch.meta/api_asynch/get_job_info.htm
     **/
    pub async fn check_job_status(&mut self, job_id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/jobs/ingest/{}/", self.base_path(), job_id);
        self.client.get(resource_url, vec![]).await
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Account {
        id: String,
        name: String,
    }

    #[tokio::test]
    async fn test_a() {}
}
