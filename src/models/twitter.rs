use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwitterResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub home: Home,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub home_timeline_urt: HomeTimelineUrt,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeTimelineUrt {
    pub instructions: Vec<Instruction>,
    #[serde(rename = "responseObjects")]
    pub response_objects: ResponseObjects,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
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
    #[serde(rename = "feedbackInfo")]
    pub feedback_info: Option<FeedbackInfo>,
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
    pub tweet_results: TweetResults,
    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: String,
//     #[serde(rename = "socialContext")]
//     pub social_context: Option<SocialContext>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TweetResults {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "__typename")]
    pub typename: String,
    // pub rest_id: String,
    // pub core: Core,
    // pub edit_control: EditControl,
    // pub is_translatable: bool,
    // pub views: Views,
    // pub source: String,
    pub legacy: Legacy2,
    // pub previous_counts: Option<PreviousCounts>,
    // pub quoted_status_result: Option<QuotedStatusResult>,
    // pub unified_card: Option<UnifiedCard2>,
    // pub card: Option<Card>,
    // pub note_tweet: Option<NoteTweet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core {
    pub user_results: UserResults,
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
    pub super_follow_eligible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel {
    pub label: Option<Label>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub url: Url,
    pub badge: Badge,
    pub description: String,
    #[serde(rename = "userLabelType")]
    pub user_label_type: String,
    #[serde(rename = "userLabelDisplayType")]
    pub user_label_display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url {
    pub url: Option<String>,
    #[serde(rename = "urlType")]
    pub url_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub url: Option<String>,
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
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub url: Option<String>,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
    pub following: Option<bool>,
    pub verified_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub description: Description,
    pub url: Option<Url3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub urls: Vec<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url2 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url3 {
    pub urls: Vec<Url4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url4 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
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
pub struct EditControl {
    #[serde(default)]
    pub edit_tweet_ids: Vec<String>,
    pub editable_until_msecs: Option<String>,
    pub is_edit_eligible: Option<bool>,
    pub edits_remaining: Option<String>,
    pub initial_tweet_id: Option<String>,
    pub edit_control_initial: Option<EditControlInitial>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditControlInitial {
    pub edit_tweet_ids: Vec<String>,
    pub editable_until_msecs: String,
    pub is_edit_eligible: bool,
    pub edits_remaining: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Views {
    pub count: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy2 {
    // pub bookmark_count: i64,
    // pub bookmarked: bool,
    // pub created_at: String,
    // pub conversation_id_str: String,
    // pub display_text_range: Vec<i64>,
    // pub entities: Entities2,
    pub favorite_count: i64,
    // pub favorited: bool,
    pub full_text: String,
    // pub is_quote_status: bool,
    // pub lang: String,
    // pub quote_count: i64,
    // pub reply_count: i64,
    pub retweet_count: i64,
    // pub retweeted: bool,
    // pub user_id_str: String,
    // pub id_str: String,
    // pub extended_entities: Option<ExtendedEntities>,
    // pub possibly_sensitive: Option<bool>,
    // pub possibly_sensitive_editable: Option<bool>,
    // pub quoted_status_id_str: Option<String>,
    // pub quoted_status_permalink: Option<QuotedStatusPermalink>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities2 {
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<Url5>,
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
    #[serde(default)]
    pub media: Vec<Medum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMention {
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url5 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashtag {
    pub indices: Vec<i64>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub indices: Vec<i64>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_key: Option<String>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub url: Option<String>,
    pub ext_media_availability: Option<ExtMediaAvailability>,
    pub features: Option<Features>,
    pub sizes: Sizes,
    pub original_info: OriginalInfo,
    pub additional_media_info: Option<AdditionalMediaInfo>,
    pub video_info: Option<VideoInfo>,
    pub source_status_id_str: Option<String>,
    pub source_user_id_str: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    pub large: Option<Large>,
    pub medium: Medium,
    pub small: Small,
    pub orig: Orig,
    pub all: Option<All>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large {
    pub faces: Vec<Face>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium {
    pub faces: Vec<Face2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face2 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small {
    pub faces: Vec<Face3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face3 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig {
    pub faces: Vec<Face4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face4 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct All {
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub user_id: String,
    pub name: String,
    pub screen_name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes {
    pub large: Option<Large2>,
    pub medium: Medium2,
    pub small: Small2,
    pub thumb: Thumb,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large2 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium2 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small2 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumb {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalInfo {
    pub height: i64,
    pub width: i64,
    pub focus_rects: Vec<FocusRect>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusRect {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalMediaInfo {
    pub monetizable: bool,
    pub source_user: Option<SourceUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceUser {
    pub user_results: UserResults2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults2 {
    pub result: Result3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result3 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel2,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Legacy3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel2 {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy3 {
    pub can_dm: bool,
    pub can_media_tag: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities3,
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
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub url: Option<String>,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities3 {
    pub description: Description2,
    pub url: Url6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description2 {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url6 {
    pub urls: Vec<Url7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url7 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<i64>,
    pub duration_millis: i64,
    pub variants: Vec<Variant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub bitrate: Option<i64>,
    pub content_type: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedEntities {
    pub media: Vec<Medum2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum2 {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_key: Option<String>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub url: Option<String>,
    pub ext_media_availability: Option<ExtMediaAvailability2>,
    pub features: Option<Features2>,
    pub sizes: Sizes2,
    pub original_info: OriginalInfo2,
    pub additional_media_info: Option<AdditionalMediaInfo2>,
    pub video_info: Option<VideoInfo2>,
    pub source_status_id_str: Option<String>,
    pub source_user_id_str: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability2 {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features2 {
    pub large: Option<Large3>,
    pub medium: Medium3,
    pub small: Small3,
    pub orig: Orig2,
    pub all: Option<All2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large3 {
    pub faces: Vec<Face5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face5 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium3 {
    pub faces: Vec<Face6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face6 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small3 {
    pub faces: Vec<Face7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face7 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig2 {
    pub faces: Vec<Face8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Face8 {
    pub x: i64,
    pub y: i64,
    pub h: i64,
    pub w: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct All2 {
    pub tags: Vec<Tag2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag2 {
    pub user_id: String,
    pub name: String,
    pub screen_name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes2 {
    pub large: Option<Large4>,
    pub medium: Medium4,
    pub small: Small4,
    pub thumb: Thumb2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large4 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium4 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small4 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thumb2 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalInfo2 {
    pub height: i64,
    pub width: i64,
    pub focus_rects: Vec<FocusRect2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusRect2 {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalMediaInfo2 {
    pub monetizable: bool,
    pub source_user: Option<SourceUser2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceUser2 {
    pub user_results: UserResults3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults3 {
    pub result: Result4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result4 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel3,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Legacy4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel3 {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy4 {
    pub can_dm: bool,
    pub can_media_tag: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities4,
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
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub url: Option<String>,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities4 {
    pub description: Description3,
    pub url: Url8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description3 {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url8 {
    pub urls: Vec<Url9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url9 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoInfo2 {
    pub aspect_ratio: Vec<i64>,
    pub duration_millis: i64,
    pub variants: Vec<Variant2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant2 {
    pub bitrate: Option<i64>,
    pub content_type: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotedStatusPermalink {
    pub url: Option<String>,
    pub expanded: String,
    pub display: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCounts {
    pub bookmark_count: i64,
    pub favorite_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotedStatusResult {
    pub result: Result5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result5 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub rest_id: String,
    pub core: Core2,
    pub unified_card: Option<UnifiedCard>,
    pub edit_control: EditControl2,
    pub is_translatable: bool,
    pub views: Views2,
    pub source: String,
    pub legacy: Legacy6,
    pub previous_counts: Option<PreviousCounts2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Core2 {
    pub user_results: UserResults4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResults4 {
    pub result: Result6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result6 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel4,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Legacy5,
    pub professional: Option<Professional2>,
    pub super_follow_eligible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel4 {
}

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
    pub url: Option<String>,
    pub verified: bool,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
    pub verified_type: Option<String>,
    pub following: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities5 {
    pub description: Description4,
    pub url: Option<Url11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description4 {
    pub urls: Vec<Url10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url10 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url11 {
    pub urls: Vec<Url12>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url12 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
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
pub struct UnifiedCard {
    pub card_fetch_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditControl2 {
    #[serde(default)]
    pub edit_tweet_ids: Vec<String>,
    pub editable_until_msecs: Option<String>,
    pub is_edit_eligible: Option<bool>,
    pub edits_remaining: Option<String>,
    pub initial_tweet_id: Option<String>,
    pub edit_control_initial: Option<EditControlInitial2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditControlInitial2 {
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
pub struct Legacy6 {
    pub bookmark_count: i64,
    pub bookmarked: bool,
    pub created_at: String,
    pub conversation_id_str: String,
    pub display_text_range: Vec<i64>,
    pub entities: Entities6,
    pub extended_entities: Option<ExtendedEntities2>,
    pub favorite_count: i64,
    pub favorited: bool,
    pub full_text: String,
    pub is_quote_status: bool,
    pub lang: String,
    pub possibly_sensitive: Option<bool>,
    pub possibly_sensitive_editable: Option<bool>,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
    pub retweeted: bool,
    pub user_id_str: String,
    pub id_str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities6 {
    #[serde(default)]
    pub media: Vec<Medum3>,
    pub user_mentions: Vec<UserMention2>,
    pub urls: Vec<Url13>,
    pub hashtags: Vec<Value>,
    pub symbols: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum3 {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: Vec<i64>,
    pub media_key: Option<String>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub url: Option<String>,
    pub additional_media_info: Option<AdditionalMediaInfo3>,
    pub ext_media_availability: Option<ExtMediaAvailability3>,
    pub sizes: Sizes3,
    pub original_info: OriginalInfo3,
    pub video_info: Option<VideoInfo3>,
    pub features: Option<Features3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalMediaInfo3 {
    pub monetizable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability3 {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes3 {
    pub large: Option<Large5>,
    pub medium: Medium5,
    pub small: Small5,
    pub thumb: Thumb3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large5 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium5 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small5 {
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
pub struct VideoInfo3 {
    pub aspect_ratio: Vec<i64>,
    pub duration_millis: i64,
    pub variants: Vec<Variant3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant3 {
    pub bitrate: Option<i64>,
    pub content_type: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features3 {
    pub large: Option<Large6>,
    pub medium: Medium6,
    pub small: Small6,
    pub orig: Orig3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large6 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium6 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small6 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig3 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMention2 {
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url13 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
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
    pub media_key: Option<String>,
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub url: Option<String>,
    pub additional_media_info: Option<AdditionalMediaInfo4>,
    pub ext_media_availability: Option<ExtMediaAvailability4>,
    pub sizes: Sizes4,
    pub original_info: OriginalInfo4,
    pub video_info: Option<VideoInfo4>,
    pub features: Option<Features4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalMediaInfo4 {
    pub monetizable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtMediaAvailability4 {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sizes4 {
    pub large: Option<Large7>,
    pub medium: Medium7,
    pub small: Small7,
    pub thumb: Thumb4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large7 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium7 {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small7 {
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
pub struct VideoInfo4 {
    pub aspect_ratio: Vec<i64>,
    pub duration_millis: i64,
    pub variants: Vec<Variant4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant4 {
    pub bitrate: Option<i64>,
    pub content_type: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features4 {
    pub large: Option<Large8>,
    pub medium: Medium8,
    pub small: Small8,
    pub orig: Orig4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Large8 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medium8 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Small8 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orig4 {
    pub faces: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCounts2 {
    pub bookmark_count: i64,
    pub favorite_count: i64,
    pub quote_count: i64,
    pub reply_count: i64,
    pub retweet_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnifiedCard2 {
    pub card_fetch_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub rest_id: String,
    pub legacy: Legacy7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy7 {
    pub binding_values: Vec<BindingValue>,
    pub card_platform: CardPlatform,
    pub name: String,
    pub url: Option<String>,
    pub user_refs_results: Vec<UserRefsResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingValue {
    pub key: String,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    pub image_value: Option<ImageValue>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub string_value: Option<String>,
    pub scribe_key: Option<String>,
    pub user_value: Option<UserValue>,
    pub image_color_value: Option<ImageColorValue>,
    pub boolean_value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageValue {
    pub height: i64,
    pub width: i64,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserValue {
    pub id_str: String,
    pub path: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageColorValue {
    pub palette: Vec<Palette>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
    pub rgb: Rgb,
    pub percentage: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rgb {
    pub blue: i64,
    pub green: i64,
    pub red: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardPlatform {
    pub platform: Platform,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    pub audience: Audience,
    pub device: Device,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Audience {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRefsResult {
    pub result: Result7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result7 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel5,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: Legacy8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel5 {
    pub label: Label2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label2 {
    pub url: Url14,
    pub badge: Badge2,
    pub description: String,
    #[serde(rename = "userLabelType")]
    pub user_label_type: String,
    #[serde(rename = "userLabelDisplayType")]
    pub user_label_display_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url14 {
    pub url: Option<String>,
    #[serde(rename = "urlType")]
    pub url_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Badge2 {
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legacy8 {
    pub following: bool,
    pub can_dm: bool,
    pub can_media_tag: bool,
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities7,
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
    pub pinned_tweet_ids_str: Vec<Value>,
    pub possibly_sensitive: bool,
    pub profile_banner_url: String,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: i64,
    pub translator_type: String,
    pub url: Option<String>,
    pub verified: bool,
    pub verified_type: String,
    pub want_retweets: bool,
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities7 {
    pub description: Description5,
    pub url: Url16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description5 {
    pub urls: Vec<Url15>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url15 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url16 {
    pub urls: Vec<Url17>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url17 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteTweet {
    pub is_expandable: bool,
    pub note_tweet_results: NoteTweetResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteTweetResults {
    pub result: Result8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result8 {
    pub id: String,
    pub text: String,
    pub entity_set: EntitySet,
    pub richtext: Option<Richtext>,
    pub media: Option<Media>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySet {
    pub user_mentions: Vec<Value>,
    pub urls: Vec<Url18>,
    pub hashtags: Vec<Value>,
    pub symbols: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url18 {
    pub display_url: String,
    pub expanded_url: String,
    pub url: Option<String>,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Richtext {
    pub richtext_tags: Vec<RichtextTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichtextTag {
    pub from_index: i64,
    pub to_index: i64,
    pub richtext_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    pub inline_media: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub topic: Topic,
    #[serde(rename = "functionalityType")]
    pub functionality_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub description: String,
    pub following: bool,
    pub icon_url: String,
    pub id: String,
    pub topic_id: String,
    pub name: String,
    pub not_interested: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackInfo {
    #[serde(rename = "feedbackKeys")]
    pub feedback_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientEventInfo {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseObjects {
    #[serde(rename = "feedbackActions")]
    pub feedback_actions: Vec<FeedbackAction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackAction {
    pub key: String,
    pub value: Value2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value2 {
    #[serde(rename = "feedbackType")]
    pub feedback_type: String,
    pub prompt: Option<String>,
    pub confirmation: Option<String>,
    #[serde(rename = "feedbackUrl")]
    pub feedback_url: Option<String>,
    #[serde(rename = "hasUndoAction")]
    pub has_undo_action: bool,
    #[serde(rename = "childKeys")]
    pub child_keys: Option<Vec<String>>,
    pub icon: Option<String>,
    #[serde(rename = "richBehavior")]
    pub rich_behavior: Option<RichBehavior>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichBehavior {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub topic: Topic2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic2 {
    pub description: String,
    pub following: bool,
    pub icon_url: String,
    pub id: String,
    pub topic_id: String,
    pub name: String,
    pub not_interested: bool,
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
