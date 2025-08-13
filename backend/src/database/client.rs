use itertools::Itertools;
use reqwest::header::InvalidHeaderValue;
use serde_json::json;
use thiserror::Error;

static DB_API_ENDPOINT: &str = "/api/v3/";

#[derive(Error, Debug)]
pub enum DatabaseClientError {
    #[error("Couldn't initialize http client ({0})")]
    HttpClientInit(reqwest::Error),

    #[error("Couldn't parse authorization token into header ({0})")]
    AuthorizationParse(InvalidHeaderValue),

    #[error("Couldn't parse database url ({0})")]
    UrlParse(url::ParseError),

    #[error("Error with HTTP request ({0})")]
    Http(reqwest::Error),

    #[error("Couldn't write data ({0})")]
    Write(reqwest::Error),

    #[error("Couldn't deserialize data ({0})")]
    Deserialize(String),
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

    pub async fn query(
        &self,
        influxql: &str,
    ) -> Result<Vec<shared::plant::PlantWithReadings>, DatabaseClientError> {
        let query_body = json!({
            "db": self.db_name,
            "q": influxql,
            "time_format": "rfc3339",
        });

        self.http_client
            .post(self.query_url()?)
            .json(&query_body)
            .send()
            .await
            .map_err(DatabaseClientError::Http)?
            .json::<Vec<super::response::InfluxResponse>>()
            .await
            .map_err(|e| DatabaseClientError::Deserialize(e.to_string()))?
            .into_iter()
            .map(std::convert::TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseClientError::Deserialize)
    }

    pub async fn write(
        &self,
        lines: Vec<super::line_protocol::LineProtocol>,
    ) -> Result<(), DatabaseClientError> {
        let serialized_lines = lines.into_iter().map(|l| l.to_influx_string()).join("\n");

        self.http_client
            .post(self.write_url()?)
            .query(&[("db", self.db_name.clone())])
            .body(serialized_lines)
            .send()
            .await
            .map_err(DatabaseClientError::Http)?
            .error_for_status()
            .map_err(DatabaseClientError::Write)?;

        Ok(())
    }
}
