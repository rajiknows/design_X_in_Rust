use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
struct Tweet {
    user_id: i32,
    tweet_id: i32,
    timestamp: u64, // Using u64 for easier comparison
}

impl Tweet {
    fn new(user_id: i32, tweet_id: i32) -> Self {
        // Get current time as milliseconds since UNIX epoch
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            user_id,
            tweet_id,
            timestamp,
        }
    }
}

// For proper ordering in BinaryHeap (newest tweets first)
impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordering by timestamp (descending) and then by tweet_id (descending)
        self.timestamp
            .cmp(&other.timestamp)
            .reverse()
            .then_with(|| self.tweet_id.cmp(&other.tweet_id).reverse())
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.tweet_id == other.tweet_id && self.user_id == other.user_id
    }
}

impl Eq for Tweet {}

type UserId = i32;

struct Twitter {
    tweets: HashMap<UserId, Vec<Tweet>>,
    followees: HashMap<UserId, Vec<UserId>>,
    max_news_feed_size: usize,
}

impl Twitter {
    fn new() -> Self {
        Self {
            tweets: HashMap::new(),
            followees: HashMap::new(),
            max_news_feed_size: 10, // News feed size limit is maintained
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let new_tweet = Tweet::new(user_id, tweet_id);

        // Get or create the user's tweet list
        let user_tweets = self.tweets.entry(user_id).or_insert_with(Vec::new);

        // Add the new tweet
        user_tweets.push(new_tweet);

        // Sort by timestamp (newest first)
        user_tweets.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut all_tweets = BinaryHeap::new();

        // Add user's own tweets
        if let Some(user_tweets) = self.tweets.get(&user_id) {
            for tweet in user_tweets {
                all_tweets.push(tweet.clone());
            }
        }

        // Add followees' tweets
        if let Some(followees) = self.followees.get(&user_id) {
            for &followee_id in followees {
                if let Some(followee_tweets) = self.tweets.get(&followee_id) {
                    for tweet in followee_tweets {
                        all_tweets.push(tweet.clone());
                    }
                }
            }
        }

        // Extract top tweets (most recent)
        let mut news_feed = Vec::new();
        while let Some(tweet) = all_tweets.pop() {
            news_feed.push(tweet.tweet_id);
            if news_feed.len() >= self.max_news_feed_size {
                break;
            }
        }

        news_feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        // Prevent users from following themselves
        if follower_id == followee_id {
            return;
        }

        // Get or create the user's followee list
        let followees = self.followees.entry(follower_id).or_insert_with(Vec::new);

        // Add followee if not already following
        if !followees.contains(&followee_id) {
            followees.push(followee_id);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.followees.get_mut(&follower_id) {
            // Remove the followee
            if let Some(pos) = followees.iter().position(|&id| id == followee_id) {
                followees.remove(pos);
            }
        }
    }

    // Additional utility functions

    fn is_following(&self, follower_id: i32, followee_id: i32) -> bool {
        if let Some(followees) = self.followees.get(&follower_id) {
            followees.contains(&followee_id)
        } else {
            false
        }
    }

    fn get_followers_count(&self, user_id: i32) -> usize {
        self.followees
            .iter()
            .filter(|(_, followees)| followees.contains(&user_id))
            .count()
    }

    fn get_user_tweets(&self, user_id: i32, limit: Option<usize>) -> Vec<i32> {
        if let Some(tweets) = self.tweets.get(&user_id) {
            let limit = limit.unwrap_or(tweets.len());
            tweets
                .iter()
                .take(limit)
                .map(|tweet| tweet.tweet_id)
                .collect()
        } else {
            Vec::new()
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_post_and_get_news_feed() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 101);
        twitter.post_tweet(1, 102);
        let feed = twitter.get_news_feed(1);
        assert_eq!(
            feed,
            vec![102, 101],
            "Most recent tweets should appear first"
        );
    }

    #[test]
    fn test_follow_and_get_news_feed() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 101);
        sleep(Duration::from_millis(10));
        twitter.post_tweet(2, 201);
        twitter.follow(1, 2);
        let feed = twitter.get_news_feed(1);
        assert_eq!(
            feed,
            vec![201, 101],
            "Tweets should be ordered by time, newest first"
        );
    }

    #[test]
    fn test_unfollow() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 101);
        sleep(Duration::from_millis(10));
        twitter.post_tweet(2, 201);
        twitter.follow(1, 2);

        // Test that following works
        let feed_before = twitter.get_news_feed(1);
        assert_eq!(
            feed_before,
            vec![201, 101],
            "Should see tweets from followed user"
        );

        // Test unfollowing
        twitter.unfollow(1, 2);
        let feed_after = twitter.get_news_feed(1);
        assert_eq!(
            feed_after,
            vec![101],
            "Should not see tweets from unfollowed user"
        );
    }

    #[test]
    fn test_feed_limit() {
        let mut twitter = Twitter::new();
        for i in 1..=15 {
            twitter.post_tweet(1, i);
            // Add small delay to ensure different timestamps
            sleep(Duration::from_millis(1));
        }
        let feed = twitter.get_news_feed(1);
        assert_eq!(feed.len(), 10, "Feed should be limited to 10 items");

        // Check that tweets are in reverse chronological order
        for i in 0..9 {
            assert!(
                feed[i] > feed[i + 1],
                "Tweets should be in descending order by ID"
            );
        }
    }

    #[test]
    fn test_follow_self() {
        let mut twitter = Twitter::new();
        twitter.follow(1, 1);
        assert!(
            !twitter.is_following(1, 1),
            "User should not be able to follow themselves"
        );
    }

    #[test]
    fn test_interleaved_timeline() {
        let mut twitter = Twitter::new();

        // User 1 posts tweet
        twitter.post_tweet(1, 101);
        sleep(Duration::from_millis(10));

        // User 2 posts tweet
        twitter.post_tweet(2, 201);
        sleep(Duration::from_millis(10));

        // User 1 posts another tweet
        twitter.post_tweet(1, 102);
        sleep(Duration::from_millis(10));

        // User 2 posts another tweet
        twitter.post_tweet(2, 202);

        // User 1 follows User 2
        twitter.follow(1, 2);

        // Get User 1's news feed
        let feed = twitter.get_news_feed(1);

        // Check that tweets are interleaved properly by time
        assert_eq!(
            feed,
            vec![202, 102, 201, 101],
            "Feed should contain interleaved tweets in chronological order"
        );
    }

    #[test]
    fn test_multiple_follows_and_unfollows() {
        let mut twitter = Twitter::new();

        // Set up users and tweets
        twitter.post_tweet(1, 101);
        twitter.post_tweet(2, 201);
        twitter.post_tweet(3, 301);

        // User 1 follows User 2 and User 3
        twitter.follow(1, 2);
        twitter.follow(1, 3);

        let feed1 = twitter.get_news_feed(1);
        assert_eq!(feed1.len(), 3, "Should see tweets from all followed users");

        // Unfollow User 2
        twitter.unfollow(1, 2);
        let feed2 = twitter.get_news_feed(1);
        assert_eq!(
            feed2.len(),
            2,
            "Should only see tweets from self and User 3"
        );
        assert!(feed2.contains(&101), "Should contain own tweet");
        assert!(feed2.contains(&301), "Should contain User 3's tweet");
        assert!(!feed2.contains(&201), "Should not contain User 2's tweet");
    }

    #[test]
    fn test_follow_twice() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(2, 201);

        // Follow User 2 twice
        twitter.follow(1, 2);
        twitter.follow(1, 2);

        // Should only show up once in the news feed
        let feed = twitter.get_news_feed(1);
        assert_eq!(
            feed,
            vec![201],
            "Following twice shouldn't duplicate tweets"
        );
    }

    #[test]
    fn test_empty_news_feed() {
        let twitter = Twitter::new();
        let feed = twitter.get_news_feed(1);
        assert_eq!(feed.len(), 0, "News feed should be empty for new users");
    }

    #[test]
    fn test_unfollow_nonexistent() {
        let mut twitter = Twitter::new();
        // This should not cause any errors
        twitter.unfollow(1, 999);
        assert!(
            !twitter.is_following(1, 999),
            "Should not be following non-existent user"
        );
    }

    #[test]
    fn test_large_number_of_tweets() {
        let mut twitter = Twitter::new();

        // Post 100 tweets
        for i in 1..=100 {
            twitter.post_tweet(1, i);
        }

        // All tweets should be stored (no cleanup)
        if let Some(tweets) = twitter.tweets.get(&1) {
            assert_eq!(
                tweets.len(),
                100,
                "All tweets should be stored without cleanup"
            );
        }

        // But news feed should still be limited to 10
        let feed = twitter.get_news_feed(1);
        assert_eq!(
            feed.len(),
            10,
            "News feed should still be limited to 10 items"
        );
        assert_eq!(feed[0], 100, "Most recent tweet should be first");
        assert_eq!(feed[9], 91, "News feed should have most recent 10 tweets");
    }
}
