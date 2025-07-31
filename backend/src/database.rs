use std::collections::HashMap;

use itertools::Itertools;
use serde_json::json;
use thiserror::Error;

static DB_API_ENDPOINT: &str = "/api/v3/";

#[derive(Error, Debug)]
pub enum DatabaseClientError {
    #[error("Couldn't initialize http client ({0})")]
    HttpClientInit(reqwest::Error),

    #[error("Couldn't parse authorization token into header ({0})")]
    AuthorizationParse(reqwest::header::InvalidHeaderValue),

    #[error("Couldn't parse database url ({0})")]
    UrlParse(url::ParseError),

    #[error("Error with HTTP request ({0})")]
    Http(reqwest::Error),
}

pub struct Client {
    db_name: String,
    db_url: url::Url,
    #[allow(clippy::struct_field_names)]
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(
        database_name: &str,
        database_url: &str,
        auth_token: &str,
    ) -> Result<Self, DatabaseClientError> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {auth_token}")
                .parse()
                .map_err(DatabaseClientError::AuthorizationParse)?,
        );

        Ok(Client {
            db_name: database_name.to_string(),
            db_url: url::Url::parse(database_url).map_err(DatabaseClientError::UrlParse)?,
            http_client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .map_err(DatabaseClientError::HttpClientInit)?,
        })
    }

    fn query_url(&self) -> Result<url::Url, DatabaseClientError> {
        self.db_url
            .join(
                format!(
                    "/{}/query_influxql",
                    DB_API_ENDPOINT
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                )
                .as_str(),
            )
            .map_err(DatabaseClientError::UrlParse)
    }

    fn write_url(&self) -> Result<url::Url, DatabaseClientError> {
        self.db_url
            .join(
                format!(
                    "/{}/write_lp",
                    DB_API_ENDPOINT
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                )
                .as_str(),
            )
            .map_err(DatabaseClientError::UrlParse)
    }

    pub async fn query(&self, influxql: &str) -> Result<reqwest::Response, DatabaseClientError> {
        let query_body = json!({
            "db": self.db_name,
            "q": influxql
        });

        self.http_client
            .post(self.query_url()?)
            .json(&query_body)
            .send()
            .await
            .map_err(DatabaseClientError::Http)
    }

    pub async fn write(
        &self,
        lines: Vec<(HashMap<String, String>, HashMap<String, String>, u64)>,
    ) -> Result<reqwest::Response, DatabaseClientError> {
        let tags_to_str =
            |m: HashMap<String, String>| m.into_iter().map(|(k, v)| format!("{k}={v}")).join(",");

        let fields_to_str = |m: HashMap<String, String>| {
            m.into_iter().map(|(k, v)| format!("{k}=\"{v}\"")).join(",")
        };

        let line_protocol = lines
            .into_iter()
            .map(|(tags, fields, timestamp)| {
                format!(
                    "{},{} {} {}",
                    self.db_name,
                    tags_to_str(tags),
                    fields_to_str(fields),
                    timestamp
                )
            })
            .join("\n");

        self.http_client
            .post(self.write_url()?)
            .query(&[("db", self.db_name.clone())])
            .body(line_protocol)
            .send()
            .await
            .map_err(DatabaseClientError::Http)
    }
}
