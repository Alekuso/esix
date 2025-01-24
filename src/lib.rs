//! **You are responsible for any possible form of abuse with the e621 API.**
//!
//! Esix is an e621 API client for Rust.
//!
//! "Esix" is a mascot name of e621.
//!
//! # Example
//!
//! This example shows an example action of getting the post with the highest score that has the tag "tic_tac".
//! ```rust
//! use esix::{Esix, error::Error};
//!
//! fn main() -> Result<(), Error> {
//!     let mut client = Esix::new(
//!             "API_KEY",
//!             "USERNAME",
//!             "project_name".to_string(),
//!             "project_version".to_string()
//!     );
//!     
//!     let posts = client.list("rating:safe", 1)?;
//!     
//!     for post in posts {
//!         println!("{:?}", post);
//!     }
//!     
//!    Ok(())
//! }
//! ```

pub mod error;
pub mod post;

use crate::error::Error;
use crate::post::{Post, Posts};
use std::fmt::Display;

/// The client that will interact with the e621 API.
pub struct Esix {
    pub username: String,
    pub key: String,
    useragent: String,
}

impl Esix {
    /// The base URL of e621 and its API.
    const URL: &'static str = "https://e621.net";

    /// Creates a new instance of the Esix client.
    ///
    /// The client requires the project name and version to be passed in order to
    /// properly identify itself to the e621 API.
    ///
    /// # Arguments
    ///
    /// * `key` - The API key to use for authentication.
    /// * `username` - The username of the e621 account.
    /// * `project_name` - The name of the project using the client.
    /// * `project_version` - The version of the project using the client.
    ///
    /// # Returns
    ///
    /// A new instance of the Esix client.
    pub fn new<S: AsRef<str> + Display>(
        key: S,
        username: S,
        project_name: String,
        project_version: String,
    ) -> Self {
        let useragent = format!("{project_name}/{project_version} (by {username} on e621)");

        Esix {
            username: username.to_string(),
            key: key.as_ref().to_string(),
            useragent,
        }
    }

    /// Lists posts from e621 based on the tags provided delimited with spaces.
    ///
    /// # Arguments
    ///
    /// * `tags` - The tags to search for.
    /// * `limit` - The maximum number of posts to return.
    ///
    /// # Returns
    ///
    /// A vector of posts that match the tags provided.
    pub fn list(&mut self, tags: impl Into<String>, limit: usize) -> Result<Vec<Post>, Error> {
        let reqwest = reqwest::blocking::Client::new();

        let tags = tags.into().replace(" ", "+");

        let url = format!("{}/posts.json", Self::URL);

        let params: Vec<(&str, String)> = vec![
            ("login", self.username.clone()),
            ("api_key", self.key.clone()),
            ("tags", tags),
            ("limit", limit.to_string()),
        ];

        let response = reqwest
            .get(url)
            .query(&params)
            .header("User-Agent", self.useragent.clone())
            .send();

        match response {
            Ok(response) => match response.json::<Posts>() {
                Ok(posts) => Ok(posts.posts),
                Err(e) => Err(Error::from(&e.to_string())),
            },
            Err(e) => Err(Error::from(&e.to_string())),
        }
    }
}
