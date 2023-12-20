use crate::client::client::{ApiVersion, Client};
use crate::errors::Error;
use crate::responses::all_jobs_response::AllJobsStatusResponse;
use crate::responses::bulk_api_v2_response::BulkApiCreateResponse;
use crate::responses::bulk_api_v2_status_response::BulkApiStatusResponse;
use crate::responses::create_response::CreateResponse;
use crate::responses::describe_global_response::DescribeGlobalResponse;
use crate::responses::job_status_response::JobStatusResponse;
use crate::responses::query_response::QueryResponse;
use crate::responses::search_response::SearchResponse;
use crate::responses::version_response::VersionResponse;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

pub struct BulkApiV2 {
    client: Client,
}

impl BulkApiV2 {
    pub fn new(client: Client) -> Self {
        BulkApiV2 { client }
    }

    fn base_path(&self) -> String {
        format!(
            "{}/services/data/{}",
            self.client.instance_url.as_ref().unwrap(),
            self.client.version
        )
    }

    pub async fn query<T: DeserializeOwned>(
        &mut self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/query/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        let res = self.client.get(query_url, params).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn query_all<T: DeserializeOwned>(
        &mut self,
        query: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!("{}/queryAll/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        let res = self.client.get(query_url, params).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn query_more<T: DeserializeOwned>(
        &mut self,
        next_records_url: &str,
    ) -> Result<QueryResponse<T>, Error> {
        let query_url = format!(
            "{}/{}",
            self.client.instance_url.as_ref().unwrap(),
            next_records_url
        );
        let res = self.client.get(query_url, vec![]).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn search_sosl(&mut self, query: &str) -> Result<SearchResponse, Error> {
        let query_url = format!("{}/search/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        let res = self.client.get(query_url, params).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Get all supported API versions
    pub async fn versions(&mut self) -> Result<Vec<VersionResponse>, Error> {
        let versions_url = format!(
            "{}/services/data/",
            self.client
                .instance_url
                .as_ref()
                .ok_or(Error::NotLoggedIn)?
        );
        let res = self.client.get(versions_url, vec![]).await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn find_by_id<T: DeserializeOwned>(
        &mut self,
        sobject_name: &str,
        id: &str,
    ) -> Result<T, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    /// Creates an SObject
    pub async fn create<T: Serialize>(
        &mut self,
        sobject_name: &str,
        params: T,
    ) -> Result<CreateResponse, Error> {
        let resource_url = format!("{}/sobjects/{}", self.base_path(), sobject_name);
        let res = self.client.post(resource_url, params, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn update<T: Serialize>(
        &mut self,
        sobject_name: &str,
        id: &str,
        params: T,
    ) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.client.patch(resource_url, params).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn upsert<T: Serialize>(
        &mut self,
        sobject_name: &str,
        key_name: &str,
        key: &str,
        params: T,
    ) -> Result<Option<CreateResponse>, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.base_path(),
            sobject_name,
            key_name,
            key
        );
        let res = self.client.patch(resource_url, params).await?;

        if res.status().is_success() {
            match res.status() {
                StatusCode::CREATED => Ok(res.json().await?),
                _ => Ok(None),
            }
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn destroy(&mut self, sobject_name: &str, id: &str) -> Result<(), Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        let res = self.client.delete(resource_url).await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ErrorResponses(res.json().await?))
        }
    }

    pub async fn describe_global(&mut self) -> Result<DescribeGlobalResponse, Error> {
        let resource_url = format!("{}/sobjects/", self.base_path());
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn describe(&mut self, sobject_name: &str) -> Result<serde_json::Value, Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.base_path(), sobject_name);
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(serde_json::from_str(res.text().await?.as_str())?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn create_job<T: Serialize>(
        &mut self,
        params: T,
    ) -> Result<BulkApiCreateResponse, Error> {
        let resource_url = format!("{}/jobs/ingest", self.base_path());
        let res = self.client.post(resource_url, params, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn upload_csv_to_job(&mut self, job_id: &str, csv: Vec<u8>) -> Result<String, Error> {
        let resource_url = format!("{}/jobs/ingest/{}/batches", self.base_path(), job_id);
        let res = self.client.put(resource_url, csv).await?;

        if res.status().is_success() {
            Ok("Created".to_string())
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn get_recent_jobs(&mut self) -> Result<AllJobsStatusResponse, Error> {
        let resource_url = format!("{}/jobs/ingest/", self.base_path());
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn get_job_status(&mut self, job_id: &str) -> Result<JobStatusResponse, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn download_csv_for_job(
        &mut self,
        job_id: &str,
        result_set: &str,
    ) -> Result<String, Error> {
        // NOTE: RESULT_SET IS ONE OF successfulResults, failedResults, unprocessedrecords
        let resource_url = format!("{}/jobs/ingest/{}/{}", self.base_path(), job_id, result_set);
        let res = self.client.get_raw(&resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn abort_job(&mut self, job_id: &str) -> Result<String, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        let mut params = HashMap::new();
        params.insert("state", "Aborted");

        let res = self.client.patch(resource_url, params).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn get_result_for_batch(
        &mut self,
        job_id: &str,
        batch_id: &str,
    ) -> Result<String, Error> {
        let resource_url = format!("{}/job/{}/batch/{}", self.base_path(), job_id, batch_id);

        let headers = vec![("Content-Type".to_string(), "text/csv".to_string())];
        let res = self.client.get_raw(&resource_url, headers).await?;

        if res.status().is_success() {
            Ok(res.text().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn set_upload_state<T: Serialize>(
        &mut self,
        job_id: &str,
        params: T,
    ) -> Result<BulkApiStatusResponse, Error> {
        let resource_url = format!("{}/jobs/ingest/{}", self.base_path(), job_id);
        let res = self.client.patch(resource_url, params).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
    }

    pub async fn check_job_status(&mut self, job_id: &str) -> Result<(), Error> {
        let resource_url = format!("{}/jobs/ingest/{}/", self.base_path(), job_id);
        let res = self.client.get(resource_url, vec![]).await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(Error::DescribeError(res.json().await?))
        }
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
