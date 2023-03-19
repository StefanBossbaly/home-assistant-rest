use chrono::{DateTime, Utc};

fn time_to_str(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    format!("{}", datetime.format("%Y-%m-%mT%H:%M:%S%:z"))
}

pub struct Query {
    pub endpoint: String,
    pub query_params: Vec<(String, String)>,
}

pub trait Queryable {
    fn generate_query(&self) -> Query;
}

#[derive(Default)]
pub struct HistoryQueryParams {
    pub filter_entity_ids: Option<Vec<String>>,
    pub start_time: Option<std::time::SystemTime>,
    pub end_time: Option<std::time::SystemTime>,
    pub minimal_response: bool,
    pub no_attributes: bool,
    pub significant_changes_only: bool
}

impl Queryable for HistoryQueryParams {
    fn generate_query(&self) -> Query {
        let mut query_params = Vec::new();

        if let Some(end_time) = self.end_time {
            query_params.push(("end_time".to_owned(), time_to_str(end_time)));
        }

        if self.minimal_response {
            query_params.push(("minimal_response".to_owned(), "true".to_owned()));
        }

        if self.no_attributes {
            query_params.push(("no_attributes".to_owned(), "true".to_owned()));
        }

        if self.significant_changes_only {
            query_params.push(("significant_changes_only".to_owned(), "true".to_owned()));
        }

        Query {
            endpoint: "api/history/".to_owned(),
            query_params,
        }
    }
}

#[derive(Default)]
pub struct LogbookParams {
    pub entity: Option<String>,
    pub start_time: Option<std::time::SystemTime>,
    pub end_time: Option<std::time::SystemTime>,
}

impl Queryable for LogbookParams {
    fn generate_query(&self) -> Query {
        let mut query_params = Vec::new();

        if let Some(ref entity) = self.entity {
            query_params.push(("entity".to_owned(), entity.to_owned()));
        }

        if let Some(end_time) = self.end_time {
            query_params.push(("end_time".to_owned(), time_to_str(end_time)));
        }

        Query {
            endpoint: "api/logbook".to_owned(),
            query_params,
        }
    }
}