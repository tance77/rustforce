use crate::client::client::Client;
use crate::errors::Error;
use reqwest::Response;
use serde::Serialize;

#[derive(Default)]
pub struct RestApi {
    pub client: Client,
}

impl RestApi {
    pub fn new(client: Client) -> Self {
        RestApi { client }
    }
    fn base_path(&self) -> String {
        let instance_url = self.client.instance_url.as_ref().unwrap();
        let version = &self.client.version;
        format!("{}/services/data/{}", instance_url, version)
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_query.htm
     **/
    pub async fn query(&mut self, query: &str) -> Result<Response, Error> {
        let query_url = format!("{}/query/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        self.client.get(query_url, params).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_queryall.htm
     **/
    pub async fn query_all(&mut self, query: &str) -> Result<Response, Error> {
        let query_url = format!("{}/queryAll/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        self.client.get(query_url, params).await
    }
    /**
     *https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_queryall_more_results.htm
     **/
    pub async fn query_more(&mut self, next_records_url: &str) -> Result<Response, Error> {
        let instance_url = self.client.instance_url.as_ref().unwrap();
        let query_url = format!("{}/{}", instance_url, next_records_url);
        self.client.get(query_url, vec![]).await
    }

    /**
     * Salesforce Object Search Language (SOSL)
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_search.htm
     **/
    pub async fn search_sosl(&mut self, query: &str) -> Result<Response, Error> {
        let query_url = format!("{}/search/", self.base_path());
        let params = vec![("q".to_string(), query.to_string())];
        self.client.get(query_url, params).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_versions.htm
     **/
    pub async fn versions(&mut self) -> Result<Response, Error> {
        let instance_url = match self.client.instance_url.as_ref() {
            Some(url) => url,
            None => return Err(Error::NotLoggedIn),
        };
        let versions_url = format!("{}/services/data/", instance_url);
        self.client.get(versions_url, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm
     **/
    pub async fn find_by_id(&mut self, sobject_name: &str, id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        self.client.get(resource_url, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_basic_info_post.htm
     **/
    pub async fn create<T: Serialize>(
        &mut self,
        object_name: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects/{}", self.base_path(), object_name);
        self.client.post(resource_url, params, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_get.htm
     **/
    pub async fn update<T: Serialize>(
        &mut self,
        object_name: &str,
        id: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), object_name, id);
        self.client.patch(resource_url, params).await
    }

    /**
     ** https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_upsert_patch.htm
     **/
    pub async fn upsert<T: Serialize>(
        &mut self,
        sobject_name: &str,
        key_name: &str,
        key: &str,
        params: T,
    ) -> Result<Response, Error> {
        let resource_url = format!(
            "{}/sobjects/{}/{}/{}",
            self.base_path(),
            sobject_name,
            key_name,
            key
        );
        self.client.patch(resource_url, params).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_retrieve_delete.htm
     **/
    pub async fn destroy(&mut self, sobject_name: &str, id: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects/{}/{}", self.base_path(), sobject_name, id);
        self.client.delete(resource_url).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_describeGlobal.htm
     **/
    pub async fn describe_global(&mut self) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects", self.base_path());
        self.client.get(resource_url, vec![]).await
    }

    /**
     * https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/resources_sobject_describe.htm
     **/
    pub async fn describe(&mut self, object_name: &str) -> Result<Response, Error> {
        let resource_url = format!("{}/sobjects/{}/describe", self.base_path(), object_name);
        self.client.get(resource_url, vec![]).await
    }
}
