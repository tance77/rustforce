extern crate reqwest;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryInto;




// #[cfg(test)]
// mod tests {
//     use crate::responses::query_response::QueryResponse;
//     use crate::{errors::Error};
//     use mockito::mock;
//     use serde::{Deserialize, Serialize};
//     use serde_json::json;
//
//     #[derive(Deserialize, Serialize)]
//     #[serde(rename_all = "PascalCase")]
//     struct Account {
//         id: String,
//         name: String,
//     }
//
//     #[tokio::test]
//     async fn login_with_credentials() -> Result<(), Error> {
//         let _m = mock("POST", "/services/oauth2/token")
//             .with_status(200)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!({
//                     "access_token": "this_is_access_token",
//                     "issued_at": "2019-10-01 00:00:00",
//                     "id": "12345",
//                     "instance_url": "https://ap.salesforce.com",
//                     "signature": "abcde",
//                     "token_type": "Bearer",
//                 })
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = super::Client::new(Some("aaa".to_string()), Some("bbb".to_string()));
//         let url = &mockito::server_url();
//         client.set_login_endpoint(url);
//         client
//             .login_with_credential("u".to_string(), "p".to_string())
//             .await?;
//         let token = client.access_token.unwrap();
//         assert_eq!("this_is_access_token", token.value);
//         assert_eq!("Bearer", token.token_type);
//         assert_eq!("2019-10-01 00:00:00", token.issued_at);
//         assert_eq!("https://ap.salesforce.com", client.instance_url.unwrap());
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn can_get_access_token() {
//         let mut client = create_test_client();
//         assert_eq!("this_is_access_token", client.get_access_token());
//     }
//
//     #[tokio::test]
//     async fn query() -> Result<(), Error> {
//         let _m = mock(
//             "GET",
//             "/services/data/v44.0/query/?q=SELECT+Id%2C+Name+FROM+Account",
//         )
//             .with_status(200)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!({
//                 "totalSize": 123,
//                 "done": true,
//                 "records": vec![
//                     Account {
//                         id: "123".to_string(),
//                         name: "foo".to_string(),
//                     },
//                 ]
//             })
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = create_test_client();
//         let r: QueryResponse<Account> = client.query("SELECT Id, Name FROM Account").await?;
//         assert_eq!(123, r.total_size);
//         assert_eq!(true, r.done);
//         assert_eq!("123", r.records[0].id);
//         assert_eq!("foo", r.records[0].name);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn create() -> Result<(), Error> {
//         let _m = mock("POST", "/services/data/v44.0/sobjects/Account")
//             .with_status(201)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!({
//                                 "id": "12345",
//                                 "success": true,
//                 //                "errors": vec![],
//                             })
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = create_test_client();
//         let r = client
//             .create("Account", [("Name", "foo"), ("Abc__c", "123")])
//             .await?;
//         assert_eq!("12345", r.id);
//         assert_eq!(true, r.success);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn update() -> Result<(), Error> {
//         let _m = mock("PATCH", "/services/data/v44.0/sobjects/Account/123")
//             .with_status(204)
//             .with_header("content-type", "application/json")
//             .create();
//
//         let mut client = create_test_client();
//         let r = client
//             .update("Account", "123", [("Name", "foo"), ("Abc__c", "123")])
//             .await;
//         assert_eq!(true, r.is_ok());
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn upsert_201() -> Result<(), Error> {
//         let _m = mock(
//             "PATCH",
//             "/services/data/v44.0/sobjects/Account/ExKey__c/123",
//         )
//             .with_status(201)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!({
//                             "id": "12345",
//                             "success": true,
//             //                "errors": vec![],
//                         })
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = create_test_client();
//         let r = client
//             .upsert(
//                 "Account",
//                 "ExKey__c",
//                 "123",
//                 [("Name", "foo"), ("Abc__c", "123")],
//             )
//             .await
//             .unwrap();
//         assert_eq!(true, r.is_some());
//         let res = r.unwrap();
//         assert_eq!("12345", res.id);
//         assert_eq!(true, res.success);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn upsert_204() -> Result<(), Error> {
//         let _m = mock(
//             "PATCH",
//             "/services/data/v44.0/sobjects/Account/ExKey__c/123",
//         )
//             .with_status(204)
//             .with_header("content-type", "application/json")
//             .create();
//
//         let mut client = create_test_client();
//         let r = client
//             .upsert(
//                 "Account",
//                 "ExKey__c",
//                 "123",
//                 [("Name", "foo"), ("Abc__c", "123")],
//             )
//             .await
//             .unwrap();
//         assert_eq!(true, r.is_none());
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn destroy() -> Result<(), Error> {
//         let _m = mock("DELETE", "/services/data/v44.0/sobjects/Account/123")
//             .with_status(204)
//             .with_header("content-type", "application/json")
//             .create();
//
//         let mut client = create_test_client();
//         let r = client.destroy("Account", "123").await?;
//         println!("{:?}", r);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn versions() -> Result<(), Error> {
//         let _m = mock("GET", "/services/data/")
//             .with_status(200)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!([{
//                     "label": "Winter '19",
//                     "url": "https://ap.salesforce.com/services/data/v44.0/",
//                     "version": "v44.0",
//                 }])
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = create_test_client();
//         let r = client.versions().await?;
//         assert_eq!("Winter '19", r[0].label);
//         assert_eq!("https://ap.salesforce.com/services/data/v44.0/", r[0].url);
//         assert_eq!("v44.0", r[0].version);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn find_by_id() -> Result<(), Error> {
//         let _m = mock("GET", "/services/data/v44.0/sobjects/Account/123")
//             .with_status(200)
//             .with_header("content-type", "application/json")
//             .with_body(
//                 json!({
//                     "Id": "123",
//                     "Name": "foo",
//                 })
//                     .to_string(),
//             )
//             .create();
//
//         let mut client = create_test_client();
//         let r: Account = client.find_by_id("Account", "123").await?;
//         assert_eq!("foo", r.name);
//
//         Ok(())
//     }
//
//     fn create_test_client() -> super::Client {
//         let mut client = super::Client::new(Some("aaa".to_string()), Some("bbb".to_string()));
//         let url = &mockito::server_url();
//         client.set_instance_url(url);
//         client.set_access_token("this_is_access_token");
//         return client;
//     }
//
//     #[tokio::test]
//     async fn test_idk() {
//         #[derive(Default, Deserialize, Serialize)]
//         #[serde(rename_all = "camelCase")]
//         pub struct BatchJob {
//             pub object: String,
//             pub content_type: String,
//             pub operation: String,
//             pub line_ending: String,
//         }
//
//         let mut client = create_test_client();
//
//         let params = BatchJob {
//             operation: "Insert".to_string(),
//             object: "Timecard".to_string(),
//             content_type: "CSV".to_string(),
//             line_ending: "LF".to_string(),
//         };
//
//
//         client.create_job(params);
//     }
// }
