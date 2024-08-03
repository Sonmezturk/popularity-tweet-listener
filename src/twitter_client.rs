use crate::models::{
    twitter_followers::TwitterFollowersResponse, twitter_user_tweets::TwitterUserTweets,
};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::internal_utils::date_utils;

use hyper::body::Bytes;
use reqwest::{Client, Method};

use reqwest::header::{HeaderMap, HeaderValue};

struct TwitterClient {
    client: Client,
    headers: HeaderMap,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTweetResponse {
    pub tweet_id: String,
    pub tweet_url: String,
    pub tweet: String,
    pub favorite_count: i64,
    pub retweet_count: i64,
    pub reply_count: i64,
}

impl TwitterClient {
    // Constructor
    pub async fn new(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        let mut headers = HeaderMap::new();
        headers.insert("authority", "twitter.com".parse()?);
        headers.insert("authorization", HeaderValue::from_str(token)?);
        headers.insert("accept", "*/*".parse()?);
        headers.insert("accept-language", "en-US,en;q=0.9".parse()?);
        headers.insert("content-type", "application/json".parse()?);
        headers.insert("referer", "https://twitter.com/decryptmedia".parse()?);
        headers.insert(
            "sec-ch-ua",
            "\"Chromium\";v=\"116\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"116\""
                .parse()?,
        );
        headers.insert("sec-ch-ua-mobile", "?0".parse()?);
        headers.insert("sec-ch-ua-platform", "\"macOS\"".parse()?);
        headers.insert("sec-fetch-dest", "empty".parse()?);
        headers.insert("sec-fetch-mode", "cors".parse()?);
        headers.insert("sec-fetch-site", "same-origin".parse()?);
        headers.insert("x-twitter-active-user", "yes".parse()?);
        headers.insert("x-twitter-auth-type", "OAuth2Session".parse()?);
        headers.insert("x-twitter-client-language", "en".parse()?);

        Ok(Self { client, headers })
    }

    pub async fn request(
        &self,
        url: String,
        method: Method,
        query: Option<Vec<(&str, &str)>>,
        body: Option<Value>,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        let mut request = self
            .client
            .request(method, url.clone())
            .headers(self.headers.clone());

        if let Some(q) = query {
            request = request.query(&q);
        }

        if let Some(b) = body {
            request = request.json(&b);
        }

        let response = request.send().await?;
        println!("url {:?} status  {:?}", url.to_string(), response.status());
        let body = response.bytes().await?;
        Ok(body)
    }
}

pub async fn user_followers(user_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let token = "Bearer <Bearer Token>";
    let twitter_client = TwitterClient::new(token).await?;

    let variables = format!(
        "{{\"userId\":\"{}\",\"count\":20,\"includePromotedContent\":false}}",
        user_id
    );

    let query = vec![
        ("variables", variables.as_str()),
        ("features", "{\"responsive_web_graphql_exclude_directive_enabled\":true,\"verified_phone_label_enabled\":false,\"creator_subscriptions_tweet_preview_api_enabled\":true,\"responsive_web_graphql_timeline_navigation_enabled\":true,\"responsive_web_graphql_skip_user_profile_image_extensions_enabled\":false,\"tweetypie_unmention_optimization_enabled\":true,\"responsive_web_edit_tweet_api_enabled\":true,\"graphql_is_translatable_rweb_tweet_is_translatable_enabled\":true,\"view_counts_everywhere_api_enabled\":true,\"longform_notetweets_consumption_enabled\":true,\"responsive_web_twitter_article_tweet_consumption_enabled\":false,\"tweet_awards_web_tipping_enabled\":false,\"freedom_of_speech_not_reach_fetch_enabled\":true,\"standardized_nudges_misinfo\":true,\"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled\":true,\"longform_notetweets_rich_text_read_enabled\":true,\"longform_notetweets_inline_media_enabled\":true,\"responsive_web_media_download_video_enabled\":false,\"responsive_web_enhance_cards_enabled\":false}"),
    ];
    let body = twitter_client
        .request(
            "https://twitter.com/i/api/graphql/<key>/BlueVerifiedFollowers"
                .to_string(),
            reqwest::Method::GET,
            Some(query),
            None,
        )
        .await?;
    let twitter_followers_response: TwitterFollowersResponse = serde_json::from_slice(&body)?;

    let all_blue_users: Vec<String> = twitter_followers_response
        .data
        .user
        .result
        .timeline
        .timeline
        .instructions
        .iter()
        .filter(|instruction| instruction.type_field == "TimelineAddEntries")
        .flat_map(|f| f.entries.iter())
        .filter_map(|f| f.content.item_content.as_ref())
        .filter(|f| f.user_results.result.legacy.followers_count > 100)
        .map(|f| f.user_results.result.rest_id.clone())
        .collect::<Vec<String>>();

    println!("user_followers_done");
    Ok(all_blue_users)
}

pub async fn user_tweets(
    user_id: String,
    since_in_hour: i64,
) -> Result<Vec<UserTweetResponse>, Box<dyn std::error::Error>> {
    let token = "Bearer <Token>";
    let twitter_client = TwitterClient::new(token).await?;
    let variables = format!("{{\"userId\":\"{}\",\"count\":10,\"includePromotedContent\":false,\"withQuickPromoteEligibilityTweetFields\":true,\"withVoice\":true,\"withV2Timeline\":true}}", user_id);
    let query = vec![
        ("variables", variables.as_str()),
        ("features", "{\"responsive_web_graphql_exclude_directive_enabled\":true,\"verified_phone_label_enabled\":false,\"responsive_web_home_pinned_timelines_enabled\":false,\"creator_subscriptions_tweet_preview_api_enabled\":true,\"responsive_web_graphql_timeline_navigation_enabled\":true,\"responsive_web_graphql_skip_user_profile_image_extensions_enabled\":false,\"tweetypie_unmention_optimization_enabled\":true,\"responsive_web_edit_tweet_api_enabled\":true,\"graphql_is_translatable_rweb_tweet_is_translatable_enabled\":true,\"view_counts_everywhere_api_enabled\":true,\"longform_notetweets_consumption_enabled\":true,\"responsive_web_twitter_article_tweet_consumption_enabled\":false,\"tweet_awards_web_tipping_enabled\":false,\"freedom_of_speech_not_reach_fetch_enabled\":true,\"standardized_nudges_misinfo\":true,\"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled\":true,\"longform_notetweets_rich_text_read_enabled\":true,\"longform_notetweets_inline_media_enabled\":true,\"responsive_web_media_download_video_enabled\":false,\"responsive_web_enhance_cards_enabled\":false}"),
    ];
    let body = twitter_client
        .request(
            "https://twitter.com/i/api/graphql/<Token>/UserTweets".to_string(),
            reqwest::Method::GET,
            Some(query),
            None,
        )
        .await?;
    println!("body {:?} userId {:?}", body.len(), user_id);
    let twitter_followers_response: TwitterUserTweets = serde_json::from_slice(&body)?;

    let filtered_entries: Vec<UserTweetResponse> = twitter_followers_response
        .data
        .user
        .result
        .as_ref()
        .and_then(|f| f.timeline_v2.as_ref())
        .iter()
        .flat_map(|f| &f.timeline.instructions)
        .filter(|instruction| instruction.type_field == "TimelineAddEntries")
        .flat_map(|t| &t.entries)
        .filter(|p| !p.entry_id.contains("promoted"))
        .filter_map(|f| f.content.item_content.as_ref())
        .filter_map(|p| p.tweet_results.result.as_ref())
        .filter(|r| {
            r.legacy.as_ref().map_or(false, |legacy| {
                legacy.lang == "en"
                    && date_utils::check_date(&legacy.created_at, since_in_hour)
                    && (legacy.favorite_count > 50
                        || legacy.reply_count > 20
                        || legacy.retweet_count > 20)
            })
        })
        .filter_map(|t| {
            let tweet = t.legacy.as_ref()?.full_text.clone();
            let tweet_id = t.legacy.as_ref()?.id_str.to_string();
            let favorite_count: i64 = t.legacy.as_ref()?.favorite_count.clone();
            let retweet_count: i64 = t.legacy.as_ref()?.retweet_count.clone();
            let reply_count: i64 = t.legacy.as_ref()?.reply_count.clone();
            let screen_name = t
                .core
                .user_results
                .result
                .legacy
                .as_ref()?
                .screen_name
                .clone();

            let tweet_url = format!("https://twitter.com/{}/status/{}", screen_name, tweet_id);

            Some(UserTweetResponse {
                tweet_id,
                tweet_url,
                tweet,
                favorite_count,
                retweet_count,
                reply_count,
            })
        })
        .collect();

    println!("user_tweets_done");

    Ok(filtered_entries)
}

pub async fn post_tweet(tweet_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = "Bearer <Token>";
    let twitter_client = TwitterClient::new(token).await?;
    let variables = format!(
        "{{\"tweet_text\":\"{}\",\"dark_request\":false,\"media\":{{\"media_entities\":[],\"possibly_sensitive\":false}},\"semantic_annotation_ids\":[]}}",
        tweet_text
    );

    let features = "{\"tweetypie_unmention_optimization_enabled\":true,\"responsive_web_edit_tweet_api_enabled\":true,\"graphql_is_translatable_rweb_tweet_is_translatable_enabled\":true,\"view_counts_everywhere_api_enabled\":true,\"longform_notetweets_consumption_enabled\":true,\"responsive_web_twitter_article_tweet_consumption_enabled\":false,\"tweet_awards_web_tipping_enabled\":false,\"responsive_web_home_pinned_timelines_enabled\":false,\"longform_notetweets_rich_text_read_enabled\":true,\"longform_notetweets_inline_media_enabled\":true,\"responsive_web_graphql_exclude_directive_enabled\":true,\"verified_phone_label_enabled\":false,\"freedom_of_speech_not_reach_fetch_enabled\":true,\"standardized_nudges_misinfo\":true,\"tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled\":true,\"responsive_web_media_download_video_enabled\":false,\"responsive_web_graphql_skip_user_profile_image_extensions_enabled\":false,\"responsive_web_graphql_timeline_navigation_enabled\":true,\"responsive_web_enhance_cards_enabled\":false}";

    let body = json!({"variables": variables.as_str(),
    "features": features,
    "queryId": "Id"});

    let _ = twitter_client
        .request(
            "https://twitter.com/i/api/graphql/<token>/CreateTweet".to_string(),
            reqwest::Method::POST,
            None,
            Some(body),
        )
        .await?;

    Ok(())
}
