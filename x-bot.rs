// A tool to share a new blog post from Ghost CMS to Twitter
// Disclaimer: This is a generated code and may not work as intended. Use it at your own risk.

// Import the required crates
use ghost_api::Client as GhostClient; // A crate for interacting with the Ghost Content API
use egg_mode::tweet::DraftTweet; // A crate for interacting with the Twitter API
use tokio; // A crate for asynchronous programming

// Define the main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Ghost client with your API key and URL
    let ghost_client = GhostClient::new(
        "https://your-ghost-site.com",
        "your-ghost-api-key",
    )?;

    // Get the latest post from your Ghost site
    let post = ghost_client.posts().get_latest().await?;

    // Create a draft tweet with the post title and URL
    let tweet = DraftTweet::new(format!(
        "{} {}",
        post.title,
        post.url
    ));

    // Create a Twitter token with your consumer key, consumer secret, access token, and access secret
    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(
            "your-twitter-consumer-key",
            "your-twitter-consumer-secret",
        ),
        access: egg_mode::KeyPair::new(
            "your-twitter-access-token",
            "your-twitter-access-secret",
        ),
    };

    // Send the tweet to Twitter
    tweet.send(&token).await?;

    // Print a success message
    println!("Tweet sent successfully!");

    Ok(())
}
