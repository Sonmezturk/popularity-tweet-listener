use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwitterFollowersResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub timeline: Timeline,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub timeline: Timeline2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline2 {
    pub instructions: Vec<Instruction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub direction: Option<String>,
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "entryId")]
    pub entry_id: String,
    #[serde(rename = "sortIndex")]
    pub sort_index: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "entryType")]
    pub entry_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<ClientEventInfo>,
    pub value: Option<String>,
    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    pub user_results: UserResults,
    #[serde(rename = "userDisplayType")]
    pub user_display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults {
    pub result: Result2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result2 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Legacy,
    pub professional: Option<Professional>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy {
    pub can_dm: bool,
    pub can_media_tag: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities,
    pub fast_followers_count: i64,
    pub favourites_count: i64,
    pub followers_count: i64,
    pub friends_count: i64,
    pub has_custom_timelines: bool,
    pub is_translator: bool,
    pub listed_count: i64,
    pub location: String,
    pub media_count: i64,
    pub name: String,
    pub normal_followers_count: i64,
    pub pinned_tweet_ids_str: Vec<String>,
    pub possibly_sensitive: bool,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub description: Description,
    pub url: Option<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub urls: Vec<Url>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url {
    pub display_url: String,
    pub expanded_url: String,
    pub url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url2 {
    pub urls: Vec<Url3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url3 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Professional {
    pub rest_id: String,
    pub professional_type: String,
    pub category: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientEventInfo {
    pub component: String,
    pub element: String,
}
