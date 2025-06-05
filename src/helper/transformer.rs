use serde::{Serialize};
use serde_json::{json, Value};

// Contoh struct untuk data dari database
#[derive(Debug)]
pub struct UserDB {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct TweetDB {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub created_at: String,
}

// Struct untuk response JSON
#[derive(Serialize)]
pub struct UserTransformed {
    pub id: i32,
    pub username: String,
    pub profile: Value,
}

#[derive(Serialize)]
pub struct TweetTransformed {
    pub id: i32,
    pub content: String,
    pub created_at: String,
}

// Generic trait Transform<T> - T bisa jadi UserTransformed atau TweetTransformed
pub trait Transform<T> {
    fn transform(&self) -> T;
}

// Implementasi Transform untuk UserDB
impl Transform<UserTransformed> for UserDB {
    fn transform(&self) -> UserTransformed {
        UserTransformed {
            id: self.id,
            username: self.username.clone(),
            profile: json!({
                "email": self.email,
                "joined_date": self.created_at,
            }),
        }
    }
}

// Implementasi Transform untuk TweetDB
impl Transform<TweetTransformed> for TweetDB {
    fn transform(&self) -> TweetTransformed {
        TweetTransformed {
            id: self.id,
            content: self.content.clone(),
            created_at: self.created_at.clone(),
        }
    }
}

// Fungsi helper untuk transform multiple items
pub fn transform_users(users: Vec<UserDB>) -> Vec<UserTransformed> {
    users.iter().map(|user| user.transform()).collect()
}

pub fn transform_tweets(tweets: Vec<TweetDB>) -> Vec<TweetTransformed> {
    tweets.iter().map(|tweet| tweet.transform()).collect()
}