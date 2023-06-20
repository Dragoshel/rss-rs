use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ERROR: could not fetch data")]
    Reqwest(#[from] reqwest::Error),

    #[error("ERROR: could not parse rss document")]
    Rss(#[from] rss::Error),

    #[error("ERROR: could not interact with Mongo Db")]
    MongoDb(#[from] mongodb::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
