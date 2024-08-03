use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwitterUserTweets {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub result: Option<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub timeline_v2: Option<TimelineV2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineV2 {
    pub timeline: Timeline,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub instructions: Vec<Instruction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub type_field: String,
    // pub entry: Option<Entry>,
    #[serde(default)]
    pub entries: Vec<Entry2>,
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
    pub item_content: ItemContent,
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: ClientEventInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    pub tweet_results: TweetResults,
    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: String,
    #[serde(rename = "socialContext")]
    pub social_context: SocialContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TweetResults {
    pub result: Result2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result2 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub rest_id: String,
    pub core: Core,

    pub is_translatable: bool,
    pub source: String,
    pub legacy: Option<Legacy2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core {
    pub user_results: UserResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults {
    pub result: Result3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result3 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Option<Legacy>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel {}

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
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub description: Description,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url2 {
    pub display_url: String,
    pub expanded_url: String,
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
pub struct Legacy2 {
    pub bookmark_count: i64,
    pub bookmarked: bool,
    pub created_at: String,
    pub conversation_id_str: String,
    pub display_text_range: Vec<i64>,
    pub entities: Entities2,
    pub favorite_count: i64,
    pub favorited: bool,
    pub full_text: String,
    pub is_quote_status: bool,
    pub lang: String,
    pub possibly_sensitive: bool,
    pub possibly_sensitive_editable: bool,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
    pub retweeted: bool,
    pub user_id_str: String,
    pub id_str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities2 {
    pub media: Vec<Medum>,
    pub user_mentions: Vec<Value>,
    pub urls: Vec<Value>,
    pub hashtags: Vec<Value>,
    pub symbols: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "contextType")]
    pub context_type: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientEventInfo {
    pub component: String,
    pub element: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry2 {
    #[serde(rename = "entryId")]
    pub entry_id: String,
    #[serde(rename = "sortIndex")]
    pub sort_index: String,
    pub content: Content2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content2 {
    #[serde(rename = "entryType")]
    pub entry_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent2>,
    // #[serde(rename = "clientEventInfo")]
    // pub client_event_info: Option<ClientEventInfo2>,
    // pub items: Option<Vec<Item>>,
    // #[serde(rename = "displayType")]
    // pub display_type: Option<String>,
    // pub header: Option<Header>,
    // pub footer: Option<Footer>,
    // pub value: Option<String>,
    // #[serde(rename = "cursorType")]
    // pub cursor_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemContent2 {
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    pub tweet_results: TweetResults2,
    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TweetResults2 {
    pub result: Option<Result4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result4 {
    // #[serde(rename = "__typename")]
    // pub typename: String,
    // pub rest_id: String,
    pub core: Core2,
    // pub edit_control: EditControl2,
    // pub is_translatable: bool,
    // pub views: Views2,
    // pub source: String,
    pub legacy: Option<Legacy4>,
    //pub quick_promote_eligibility: QuickPromoteEligibility2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core2 {
    pub user_results: UserResults2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults2 {
    pub result: Result5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result5 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel2,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Option<Legacy3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy3 {
    //pub following: bool,
    // pub can_dm: bool,
    // pub can_media_tag: bool,
    // pub created_at: String,
    // pub default_profile: bool,
    // pub default_profile_image: bool,
    // pub description: String,
    // pub entities: Entities3,
    // pub fast_followers_count: i64,
    // pub favourites_count: i64,
    // pub followers_count: i64,
    // pub friends_count: i64,
    // pub has_custom_timelines: bool,
    // pub is_translator: bool,
    // pub listed_count: i64,
    // pub location: String,
    // pub media_count: i64,
    // pub name: String,
    // pub normal_followers_count: i64,
    // pub pinned_tweet_ids_str: Vec<String>,
    // pub possibly_sensitive: bool,
    // pub profile_banner_url: String,
    // pub profile_image_url_https: String,
    // pub profile_interstitial_type: String,
    pub screen_name: String,
    //     pub statuses_count: i64,
    //     pub translator_type: String,
    //     pub verified: bool,
    //     pub want_retweets: bool,
    //     pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities3 {
    pub description: Description2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description2 {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url3 {
    pub urls: Vec<Url4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url4 {
    pub display_url: String,
    pub expanded_url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Professional2 {
    pub rest_id: String,
    pub professional_type: String,
    pub category: Vec<Category2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category2 {
    pub id: i64,
    pub name: String,
    pub icon_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditControl2 {
    pub edit_tweet_ids: Vec<String>,
    pub editable_until_msecs: String,
    pub is_edit_eligible: bool,
    pub edits_remaining: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Views2 {
    pub count: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy4 {
    pub bookmark_count: i64,
    //pub bookmarked: bool,
    pub created_at: String,
    // pub conversation_id_str: String,
    // pub display_text_range: Vec<i64>,
    // pub entities: Entities4,
    // pub extended_entities: ExtendedEntities2,
    pub favorite_count: i64,
    // pub favorited: bool,
    pub full_text: String,
    // pub is_quote_status: bool,
    pub lang: String,
    // pub possibly_sensitive: bool,
    // pub possibly_sensitive_editable: bool,
    // pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
    // pub retweeted: bool,
    // pub user_id_str: String,
    pub id_str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities4 {
    pub media: Vec<Medum3>,
    pub user_mentions: Vec<Value>,
    pub urls: Vec<Value>,
    pub hashtags: Vec<Value>,
    pub symbols: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum3 {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub features: Features3,
    pub sizes: Sizes3,
    pub original_info: OriginalInfo3,
    pub media_key: Option<String>,
    pub ext_media_availability: Option<ExtMediaAvailability2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features3 {
    pub large: Large5,
    pub medium: Medium5,
    pub small: Small5,
    pub orig: Orig3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large5 {
    pub faces: Vec<Face9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face9 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium5 {
    pub faces: Vec<Face10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face10 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small5 {
    pub faces: Vec<Face11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face11 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig3 {
    pub faces: Vec<Face12>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face12 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes3 {
    pub large: Large6,
    pub medium: Medium6,
    pub small: Small6,
    pub thumb: Thumb3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large6 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium6 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small6 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumb3 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalInfo3 {
    pub height: i64,
    pub width: i64,
    pub focus_rects: Vec<FocusRect3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusRect3 {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability2 {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedEntities2 {
    pub media: Vec<Medum4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum4 {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_key: String,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ext_media_availability: ExtMediaAvailability3,
    pub features: Features4,
    pub sizes: Sizes4,
    pub original_info: OriginalInfo4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability3 {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features4 {
    pub large: Large7,
    pub medium: Medium7,
    pub small: Small7,
    pub orig: Orig4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large7 {
    pub faces: Vec<Face13>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face13 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium7 {
    pub faces: Vec<Face14>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face14 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small7 {
    pub faces: Vec<Face15>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face15 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig4 {
    pub faces: Vec<Face16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face16 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes4 {
    pub large: Large8,
    pub medium: Medium8,
    pub small: Small8,
    pub thumb: Thumb4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large8 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium8 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small8 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumb4 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalInfo4 {
    pub height: i64,
    pub width: i64,
    pub focus_rects: Vec<FocusRect4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusRect4 {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuickPromoteEligibility2 {
    pub eligibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientEventInfo2 {
    pub component: String,
    pub element: Option<String>,
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    #[serde(rename = "timelinesDetails")]
    pub timelines_details: TimelinesDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelinesDetails {
    #[serde(rename = "injectionType")]
    pub injection_type: String,
    #[serde(rename = "controllerData")]
    pub controller_data: String,
    #[serde(rename = "sourceData")]
    pub source_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "entryId")]
    pub entry_id: String,
    pub item: Item2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item2 {
    #[serde(rename = "itemContent")]
    pub item_content: ItemContent3,
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: ClientEventInfo3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemContent3 {
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(rename = "userDisplayType")]
    pub user_display_type: String,
    #[serde(rename = "socialContext")]
    pub social_context: Option<SocialContext2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel3 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy5 {
    pub can_dm: bool,
    pub can_media_tag: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities5,
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
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities5 {
    pub description: Description3,
    pub url: Option<Url5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description3 {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url5 {
    pub urls: Vec<Url6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url6 {
    pub display_url: String,
    pub expanded_url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Professional3 {
    pub rest_id: String,
    pub professional_type: String,
    pub category: Vec<Category3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category3 {
    pub id: i64,
    pub name: String,
    pub icon_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext2 {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "contextType")]
    pub context_type: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientEventInfo3 {
    pub component: String,
    pub element: String,
    pub details: Details2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details2 {
    #[serde(rename = "timelinesDetails")]
    pub timelines_details: TimelinesDetails2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelinesDetails2 {
    #[serde(rename = "injectionType")]
    pub injection_type: String,
    #[serde(rename = "controllerData")]
    pub controller_data: String,
    #[serde(rename = "sourceData")]
    pub source_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "displayType")]
    pub display_type: String,
    pub text: String,
    pub sticky: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Footer {
    #[serde(rename = "displayType")]
    pub display_type: String,
    pub text: String,
    #[serde(rename = "landingUrl")]
    pub landing_url: LandingUrl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandingUrl {
    #[serde(rename = "urlType")]
    pub url_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "scribeConfig")]
    pub scribe_config: ScribeConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScribeConfig {
    pub page: String,
}
