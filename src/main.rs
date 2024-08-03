mod chatgpt_client;
mod redis_client;
mod slack_client;
mod telegram_client;
mod twitter_client;
mod models {
    pub mod twitter;
    pub mod twitter_followers;
    pub mod twitter_user_tweets;
}
mod internal_utils;
mod pinecone_client;

use pinecone_client::QueryResponse;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use chatgpt_client::{CompletionRequest, Message, OpenAI};

use pinecone_client::PineConeClient;
use redis_client::{get_from_redis, push_to_redis};
use serde_json::json;
use slack_client::send_message_to_slack;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::{env, error::Error, time::Duration};

use telegram_client::TelegramBot; // Import the TelegramBot struct

use tokio::time::sleep;
use twitter_client::{post_tweet, user_followers, user_tweets, UserTweetResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ChatgptResponse {
    tweet_id: String,
    title: String,
    description: String,
    category: String,
    tweet_urls: Vec<String>,
    favorite_count: i64,
    retweet_count: i64,
    reply_count: i64,
}

#[derive(Debug)]
struct TweetVectorLookup {
    score: f64,
    embeddings: Vec<f64>,
}

#[derive(Deserialize)]
struct LambdaRequest {}

async fn rate_limited_execution(users: &[&str]) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut results = Vec::new();

    for user in users {
        sleep(Duration::from_secs(2)).await;
        let followers = user_followers(user.to_string()).await?;
        results.push(followers);
    }

    Ok(results)
}

async fn rate_limited_execution_for_user_tweets(
    users: &[&str],
    since_hour: i64,
) -> Result<Vec<Vec<UserTweetResponse>>, Box<dyn Error>> {
    let mut results = Vec::new();

    for user in users {
        sleep(Duration::from_secs(2)).await;

        match user_tweets(user.to_string(), since_hour).await {
            Ok(response) => {
                results.push(response);
            }
            Err(e) => {
                eprintln!("Error getting user tweet: {:?}", e);
            }
        }
    }

    Ok(results)
}

fn write_to_log(results: &[String]) {
    println!("{:?}", results);
    // Use OpenOptions to open the file in append mode
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("log.txt") {
        for result in results {
            if let Err(err) = writeln!(file, "{}", result) {
                println!("Failed to write to log file: {}", err);
            }
        }
        println!("Results appended to log file.");
    } else {
        println!("Failed to open log file.");
    }
}

async fn function_handler() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let chatgpt_api_key =
        env::var("CHATGPT_API_KEY").unwrap_or_else(|_| panic!("chatgpt api key is not set."));

    let pinecone_api_key =
        env::var("PINECONE_API_KEY").unwrap_or_else(|_| panic!("pinecone api key is not set."));

    let pinecone_base_url = "https://twitter-<PineCone>.svc.gcp-starter.pinecone.io/";

    let keywords = [
        "Binance", "Bitcoin", "Ethereum", "DeFi", "btc", "eth", "bnb", "hack", "crypto", "USDC",
        "bear", "NFT", "Web3",
    ];

    let since_hour = 2;

    // CoinGecko, Messari , CZ, OKX
    let users = [
        "2412652615",          // CoinGecko
        "412587524",           // Messari
        "902926941413453824",  // CZ
        "867617849208037377",  // OKX
        "864347902029709314",  //  Crypto.com
        "877807935493033984",  // Binance
        "244647486",           // Michael Saylor
        "1387497871751196672", // Watcher.Guru
        "1156037602488639490", // DeBank
        "263017181",           // Hulk Gaming
        "574032254",           // Coinbase
        "3129477561",          // Consensys
        "928759224599040001",  // crypto blomberg
        "1395365515779055616", // The DeFi Investor
        "720487892670410753",  // Coinbase Exchange
        "2207129125",          // Cointelegraph
        "1136776548013334529", // DefiMoon
        "1321585610654687241", // defillama
        "2260491445",          // CoinMarketCap
        "1039833297751302144", // WhaleAlert
        "3367334171",          // Bitcoin News
        "1494146332528885767", // CertiKAlert
    ];

    // let all_followers = rate_limited_execution(&users).await?;
    // println!("{:?}", all_followers);
    let user_tweets = rate_limited_execution_for_user_tweets(&users, since_hour).await?;

    let flat_results: Vec<UserTweetResponse> = user_tweets.into_iter().flatten().collect();
    //println!("flat_results {:?}", flat_results);

    if flat_results.is_empty() {
        return Ok(());
    }

    let openai = OpenAI::new(&chatgpt_api_key);
    let pinecone_client = PineConeClient::new(&pinecone_api_key, pinecone_base_url.to_string())?;
    let mut filtered_results = Vec::new();

    println!("flat_results len {:?}", &flat_results.len());
    let mut tweet_id_score_lookup: HashMap<String, TweetVectorLookup> = HashMap::new();

    for user_tweet in flat_results.into_iter() {
        let embedding_response = openai.get_embedding(&user_tweet.tweet).await?;

        let daum = &embedding_response.data[0];
        let query_response = PineConeClient::query(&pinecone_client, &daum.embedding).await?;
        let score = get_score(&query_response);

        if score < 0.85 {
            filtered_results.push(user_tweet.clone());

            tweet_id_score_lookup.insert(
                user_tweet.tweet_id,
                TweetVectorLookup {
                    score,
                    embeddings: daum.embedding.clone(),
                },
            );
        }
    }

    println!("filtered_results len {:?}", &filtered_results.len());

    if filtered_results.is_empty() {
        return Ok(());
    }

    let tweets_json =
        serde_json::to_string(&filtered_results).expect("Failed to serialize to JSON");

    let mut messages: Vec<Message> = vec![Message {
            role: "system".to_string(),
            content: r#"
            You will be provided with an array of tweets from various users over the past hour. Your task is to identify and summarize up to 5 trending topics related to Digital Assets & Cryptocurrency based on these tweets. 
            
            Avoid considering tweets that ask questions, mention videos, pictures, or provide links without contextual information.

            Prioritize the topics according to the following criteria (1 being the most important):
            1. Cyberattacks impacting the Digital Assets or Crypto community.
            2. Instances of fraud or scams related to Digital Assets or Cryptocurrency.
            3. Newly emerging platforms and decentralized applications (dApps) gaining popularity.
            4. Significant news concerning Digital Assets, Cryptocurrency, or blockchain technology.
            5. Large transaction transfers (txs) within the Digital Assets or Crypto sphere.
            
            Please provide your findings in the following JSON format:
            ```json
            {
                "title": "string representing the topic",
                "description": "string providing a concise summary of the topic.",
                "favorite_count": "It shows how many times the topic has been favorited.",
                "retweet_count": "It shows how many times the topic has been retweeted.",
                "reply_count": "It shows how many times the topic has been replied.",
                "category": "string indicating the category number (e.g., '1 - Cyberattacks')",
                "tweet_urls": "array of strings representing the tweet URLs where the topic is mentioned. If the topic is mentioned in multiple tweets, include all related tweet URLs."
            }
            ```
            Ensure that each topic is unique and only associated with one category from the list above. If a topic could fall under multiple categories, choose the higher-priority category (lower number) according to the criteria provided. In case there are fewer than 5 relevant topics, include only the relevant ones. If a tweet mentions multiple topics, treat each topic separately.
            Example Input:
            ```json
            {
                "tweets": [
                    {"tweet_url": "https://twitter.com/xyz_user/status/1709289016292766016", "tweet_id": 1706304502054240256, "tweet": "New #DeFi platform XYZ is making waves with its innovative approach...", favorite_count: 37, retweet_count: 8, reply_count: 20},
                    {"tweet_url": "https://twitter.com/abc_user/status/1909273016292766020", "tweet_id": 1709645853021949952, "tweet": "Huge transfer of 10,000 BTC reported...", favorite_count: 37, retweet_count: 9, reply_count: 28},
                    // ...more tweets
                ]
            }
            ```
            Example Output:
            ```json
            [
                {
                    "tweet_id": 1706304502054240256,
                    "title": "New DeFi Platform XYZ",
                    "description": "A new decentralized finance platform, XYZ, has been gaining traction due to its innovative approach...",
                    "favorite_count": 37,
                    "retweet_count": 9, 
                    "reply_count": 28,
                    "category": "3 - New popular platform and dapps",
                    "tweet_urls": ["https://twitter.com/xyz_user/status/1709289016292766016"]
                },
                {
                    "tweet_id": 1709645853021949952,
                    "title": "Large Bitcoin Transfer",
                    "description": "A transfer of 10,000 BTC was reported, showcasing significant movement within the crypto market...",
                    "favorite_count": 37,
                    "retweet_count": 9, 
                    "reply_count": 28,
                    "category": "5 - Huge transfer txs",
                    "tweet_urls": ["https://twitter.com/abc_user/status/1909273016292766020"]
                },
                // ...more topics
            ]
            ```"#.to_string(),
        }];

    messages.push(Message {
        role: "user".to_string(),
        content: format!(
            "{:?}",
            json!({
                "tweets": tweets_json
            })
        ),
    });

    let req = CompletionRequest {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.0,
        max_tokens: 1000,
        top_p: 0.1,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
    };

    let bot = TelegramBot::new("<bot Id>");

    let topics: chatgpt_client::CompletionResponse = openai.get_completion(&req).await?;
    println!("total tokens {:?}", topics.usage.total_tokens);
    println!("chatgpt response  {}", topics.choices[0].message.content);

    let json_data = extract_json_from_markdown(&topics.choices[0].message.content)
        .map_or_else(|| topics.choices[0].message.content.as_str(), |data| data);

    let parsed: Vec<ChatgptResponse> = serde_json::from_str(&json_data)?;

    for data in parsed.into_iter() {
        let current_embedding = tweet_id_score_lookup.get(&data.tweet_id);

        let cos_score = current_embedding
            .map(|embedding| embedding.score)
            .unwrap_or(0.0);

        let cos_score_string = cos_score.to_string();

        // println!("{}", cos_score_string);

        // let send_message_to_slack_future = send_message_to_slack(
        //     &data.title,
        //     &data.description,
        //     &data.category,
        //     &data.tweet_urls,
        //     &data.favorite_count,
        //     &data.retweet_count,
        //     &data.reply_count,
        //     &cos_score_string,
        // );

        let formatted_tweet = hashtagify(&data.description, &keywords);

        //println!("formatted_tweet {:}", formatted_tweet + "tkelsk");
        let new_tweet: String = format!(
            r#"{} Get The Latest Updates {}"#,
            formatted_tweet, "https://t.me/crypto_forerunner"
        );

        match post_tweet(&new_tweet).await {
            Ok(_) => println!("Tweet posted successfully!"),
            Err(e) => eprintln!("Failed to post tweet: {}", e),
        }

        // match send_message_to_slack_future.await {
        //     Ok(_) => println!("Sent to Slack!"),
        //     Err(e) => eprintln!("Failed to send to Slack: {}", e),
        // }

        if let Some(embedding) = current_embedding.map(|f| &f.embeddings) {
            PineConeClient::save_vector(&pinecone_client, embedding).await?;
        }

        let telegram_text = format!(
            r#"
*{}*
{}
Favorite Count: {}, Retweet Count: {}, Reply Count {}"#,
            &data.title,
            data.description,
            &data.favorite_count,
            &data.retweet_count,
            &data.reply_count,
        );

        bot.send_message("<channelId>", telegram_text.as_str())
            .await?;

        tokio::time::sleep(tokio::time::Duration::new(5, 0)).await; // Sleep for 5 seconds
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    function_handler().await?;
    Ok(())
}

fn extract_json_from_markdown(input: &str) -> Option<&str> {
    let start_delimiter = "```json";
    let end_delimiter = "```";

    let start = input.find(start_delimiter)? + start_delimiter.len();
    let end = input.rfind(end_delimiter)?;

    Some(&input[start..end].trim())
}

fn get_score(query_response: &QueryResponse) -> f64 {
    query_response
        .matches
        .as_ref() // Convert Option<Vec<Match>> to Option<&Vec<Match>>
        .and_then(|matches| matches.first()) // Get the first match if it exists
        .and_then(|m| m.score) // Get the score if it exists
        .unwrap_or(0.0) // If there's no match or no score, default to 0.0
}

fn hashtagify(text: &str, keywords: &[&str]) -> String {
    let mut result = text.to_owned();

    for &keyword in keywords {
        let pattern = format!(r"(?i)\b{}\b", regex::escape(keyword)); // (?i) makes it case-insensitive, \b ensures word boundaries
        let re = Regex::new(&pattern).unwrap();
        result = re
            .replace_all(&result, |caps: &regex::Captures| format!("#{}", &caps[0]))
            .to_string();
    }

    result
}
