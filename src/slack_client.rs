use std::env;

use reqwest::Client;
use serde_json::json;

pub async fn send_message_to_slack(
    title: &str,
    desc: &str,
    category: &str,
    tweet_urls: &Vec<String>,
    favorite_count: &i64,
    retweet_count: &i64,
    reply_count: &i64,
    score: &str,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let slack_api_url = "https://slack.com/api/chat.postMessage";
    let token = env::var("SLACK_TOKEN").unwrap_or_else(|_| panic!("slack token is not set."));
    let channel_id =
        env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| panic!("channel id is not set."));

    let mut blocks = vec![
        json!({
            "type": "header",
            "text": {
                "type": "plain_text",
                "text": title,  // Assuming title is defined elsewhere
                "emoji": true
            }
        }),
        json!({
            "type": "section",
            "block_id": "sectionBlockOnlyPlainText",
            "text": {
                "type": "plain_text",
                "text": format!("{}", desc)  // Assuming desc is defined elsewhere
            }
        }),
        json!({
            "type": "divider"
        }),
    ];

    if !tweet_urls.is_empty() {
        blocks.push({
            json!({
                    "type": "section",
                    "block_id": "sectionBlockWithLinkButton",
                    "text": {
                        "type": "mrkdwn",
                        "text": format!("Favorite Count: {}, Retweet Count: {}, Reply Count: {} ", favorite_count, retweet_count, reply_count),
                    },
                    "accessory": {
                        "type": "button",
                        "text": {
                            "type": "plain_text",
                            "text": "Tweet link",
                            "emoji": true
                        },
                        "value": "click_me_123",
                        "url": format!("{}", tweet_urls[0]),
                        "action_id": "button-action"
                    }
            })
        });
    }

    blocks.extend(vec![
        json!({
            "type": "divider"
        }),
        json!({
            "type": "section",
            "block_id": "sectionBlockOnlyFields",
            "fields": [
                {
                    "type": "plain_text",
                    "text": format!("Score of pinecone: {:}", score),
                    "emoji": true
                }
            ]
        }),
        json!({
            "type": "divider"
        }),
    ]);
    println!("blocks : {:?}", serde_json::to_string(&blocks));
    let req = client.post(slack_api_url).bearer_auth(token).json(&json!({
        "blocks": blocks,
        "channel": channel_id,
    }));

    let response = req.send().await?;

    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Failed to send message: {}", response.status());
    }

    Ok(())
}
