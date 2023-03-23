extern crate reqwest;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse<T> {
    pub total_size: i32,
    pub done: bool,
    pub records: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnonymousApexResponse {
    pub compiled: bool,
    pub success: bool,
    pub body: String
}

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub id: String,
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct UpsertResponse {
    create: Option<CreateResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: String,
    pub fields: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub id: String,
    pub issued_at: String,
    pub access_token: String,
    pub instance_url: String,
    pub signature: String,
    pub token_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TokenErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub token_type: String,
    pub value: String,
    pub issued_at: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct DescribeResponse {
    pub activateable: bool,
    //    pub action_overrides: ActionOverride[],
    pub child_relationships: Vec<ChildRelationship>,
    pub compact_layoutable: bool,
    pub createable: bool,
    pub custom: bool,
    pub custom_setting: bool,
    pub deletable: bool,
    pub deprecated_and_hidden: bool,
    pub feed_enabled: bool,
    pub fields: Vec<Field>,
    pub has_subtypes: bool,
    pub is_subtype: bool,
    pub key_prefix: Option<String>,
    pub label: String,
    pub label_plural: String,
    pub layoutable: bool,
    pub listviewable: Option<bool>,
    pub lookup_layoutable: Option<bool>,
    pub mergeable: bool,
    pub mru_enabled: bool,
    pub name: String,
    //    pub named_layout_infos: [],
    //    pub network_scope_field_name: [],
    pub queryable: bool,
    //    pub record_type_infos: Record_type_info[]
    pub replicateable: bool,
    pub retrieveable: bool,
    pub search_layoutable: bool,
    pub searchable: bool,
    //    pub supported_scopes:  Scope_info
    pub triggerable: bool,
    pub undeletable: bool,
    pub updateable: bool,
    pub urls: Urls,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Field {
    pub aggregatable: bool,
    pub ai_prediction_field: bool,
    pub auto_number: bool,
    pub byte_length: u32,
    pub calculated: bool,
    pub calculated_formula: Option<String>,
    pub cascade_delete: bool,
    pub case_sensitive: bool,
    pub compound_field_name: Option<String>,
    pub controller_name: Option<String>,
    pub createable: bool,
    pub custom: bool,
    //    pub default_value: Option<String>,
    pub default_value_formula: Option<String>,
    pub defaulted_on_create: bool,
    pub dependent_picklist: bool,
    pub deprecated_and_hidden: bool,
    pub digits: u8,
    pub display_location_in_decimal: bool,
    pub encrypted: bool,
    pub external_id: bool,
    pub extra_type_info: Option<String>,
    pub filterable: bool,
    pub filtered_lookup_info: Option<String>,
    pub formula_treat_null_number_as_zero: bool,
    pub groupable: bool,
    pub high_scale_number: bool,
    pub html_formatted: bool,
    pub id_lookup: bool,
    pub inline_help_text: Option<String>,
    pub label: String,
    pub length: u32,
    pub mask: Option<String>,
    pub mask_type: Option<String>,
    pub name: String,
    pub name_field: bool,
    pub name_pointing: bool,
    pub nillable: bool,
    pub permissionable: bool,
    //    pub picklist_values: [],
    pub polymorphic_foreign_key: bool,
    pub precision: u8,
    pub query_by_distance: bool,
    pub reference_target_field: Option<String>,
    //    pub reference_to: [],
    pub relationship_name: Option<String>,
    pub relationship_order: Option<String>,
    pub restricted_delete: bool,
    pub restricted_picklist: bool,
    pub scale: u8,
    pub search_prefilterable: bool,
    pub soap_type: String,
    pub sortable: bool,
    #[serde(rename = "type")]
    pub field_type: String,
    pub unique: bool,
    pub updateable: bool,
    pub write_requires_master_read: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct ChildRelationship {
    pub cascade_delete: bool,
    #[serde(rename = "childSObject")]
    pub child_sobject: Option<String>,
    pub deprecated_and_hidden: bool,
    pub field: String,
    //    pub junction_id_list_names: [],
    //    pub junction_reference_to: [],
    pub relationship_name: Option<String>,
    pub restricted_delete: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Urls {
    pub compact_layouts: String,
    pub row_template: String,
    pub approval_layouts: String,
    pub ui_detail_template: String,
    pub ui_edit_template: String,
    pub default_values: String,
    pub listviews: String,
    pub describe: String,
    pub ui_new_record: String,
    pub quick_actions: String,
    pub layouts: String,
    pub sobject: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct DescribeGlobalResponse {
    pub encoding: String,
    pub max_batch_size: u16,
    pub sobjects: Vec<DescribeGlobalSObjectResponse>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct DescribeGlobalSObjectResponse {
    pub activateable: bool,
    pub createable: bool,
    pub custom: bool,
    pub custom_setting: bool,
    pub deletable: bool,
    pub deprecated_and_hidden: bool,
    pub feed_enabled: bool,
    pub has_subtypes: bool,
    pub is_subtype: bool,
    pub key_prefix: Option<String>,
    pub label: String,
    pub label_plural: String,
    pub layoutable: bool,
    pub mergeable: bool,
    pub mru_enabled: bool,
    pub name: String,
    pub queryable: bool,
    pub replicateable: bool,
    pub retrieveable: bool,
    pub searchable: bool,
    pub triggerable: bool,
    pub undeletable: bool,
    pub updateable: bool,
    pub urls: HashMap<String, String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub search_records: Vec<SearchRecord>,
    //    pub metadata: Metadata,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct SearchRecord {
    #[serde(rename = "Id")]
    pub id: String,
    pub attributes: SObjectAttribute,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct SObjectAttribute {
    #[serde(rename = "type")]
    pub sobject_type: String,
    pub url: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct VersionResponse {
    pub label: String,
    pub url: String,
    pub version: String,
}

#[derive(Deserialize, Clone)]
pub struct OAuthUserInfo {
    pub sub: String,
    pub user_id: String,
    pub organization_id: String,
    pub preferred_username: String,
    pub nickname: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub given_name: String,
    pub family_name: String,
    pub zoneinfo: String,
    pub photos: Photos,
    pub profile: String,
    pub picture: String,
    pub phone_number: String,
    pub phone_number_verified: bool,
    pub address: Address,
    pub urls: InstanceUrls,
    pub active: bool,
    pub user_type: String,
    pub language: String,
    pub locale: String,
    #[serde(rename = "utcOffset")]
    pub utc_offset: i64,
    pub updated_at: String,
    pub is_app_installed: bool,
}

#[derive(Deserialize, Clone)]
pub struct Address {
    pub street_address: String,
    pub locality: String,
    pub region: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Deserialize, Clone)]
pub struct Photos {
    pub picture: String,
    pub thumbnail: String,
}

#[derive(Deserialize, Clone)]
pub struct InstanceUrls {
    pub enterprise: String,
    pub metadata: String,
    pub partner: String,
    pub rest: String,
    pub sobjects: String,
    pub search: String,
    pub query: String,
    pub recent: String,
    pub tooling_soap: String,
    pub tooling_rest: String,
    pub profile: String,
    pub feeds: String,
    pub groups: String,
    pub users: String,
    pub feed_items: String,
    pub feed_elements: String,
    pub custom_domain: String,
}

#[derive(Deserialize, Clone)]
pub struct IdResult {
    pub id: String,
}

#[derive(Deserialize, Clone)]
pub struct ApexLog {
    pub Application: String,
    pub DurationMiliseconds: i32,
    pub Location: String,
    pub LogLength: i32,
    pub Operation: String,
    pub Request: String,
    pub StartTime: String,
    pub Status: String,
    pub SystemModstamp: String,
    pub id: String,
    pub LogUserId: String,
    pub RequestType: String,
}