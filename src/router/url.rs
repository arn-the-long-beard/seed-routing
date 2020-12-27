use crate::ParseError;
use seed::{prelude::IndexMap, Url};

#[allow(clippy::missing_errors_doc)]
/// Used by `#[derive(ParseUrl)]`
pub trait Navigation {
    fn from_url(url: Url) -> std::result::Result<Self, ParseError>
    where
        Self: Sized;
    fn to_url(&self) -> Url;
}

pub fn convert_to_string(query: &IndexMap<String, String>) -> String {
    let mut query_string = "".to_string();
    for (i, q) in query.iter().enumerate() {
        query_string += format!("{}={}", q.0, q.1).as_str();

        if i != query.len() - 1 {
            query_string += "&".to_string().as_str();
        }
    }
    query_string
}

pub fn extract_url_payload(
    url_string: String,
    with_id_param: bool,
    with_query_parameters: bool,
    with_children: bool,
) -> (
    Option<String>,
    Option<IndexMap<String, String>>,
    Option<String>,
) {
    let param_id = if with_id_param {
        Some(extract_id_parameter(&url_string))
    } else {
        None
    };

    let query_parameters = if with_query_parameters {
        Some(extract_query_params(&url_string))
    } else {
        None
    };

    let children_path = if with_children {
        Some(extract_children_string(url_string, param_id.clone()))
    } else {
        None
    };

    (param_id, query_parameters, children_path)
}

pub fn extract_id_parameter(url_string: &str) -> String {
    let mut single_paths = url_string.split('/');

    let root = single_paths.next();

    if root.is_some() && !root.unwrap().is_empty() {
        eprintln!("root path should be like '' because urls starts with / ");
    }
    // make error if root is not empty
    let mut param_id = single_paths
        .next()
        .map(std::string::ToString::to_string)
        .expect("Should have param id");

    if param_id.contains('?') {
        param_id = param_id
            .split('?')
            .next()
            .map(std::string::ToString::to_string)
            .expect("We should have a id parameter but got empty string")
    }
    param_id
}

pub fn extract_children_string(url_string: String, param_id: Option<String>) -> String {
    let full_query = url_string;
    let children_path: Option<String>;

    if param_id.is_some() {
        println!("We have id param");
        children_path = full_query
            .trim_start_matches('/')
            .to_string()
            .strip_prefix(&param_id.expect("should have id parameter"))
            .map(std::string::ToString::to_string);
    } else {
        println!("No id param");
        children_path = Some(full_query)
    }

    children_path.expect("We should have a children path")
}

pub fn extract_query_params(url_string: &str) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    let url_parts: Vec<&str> = url_string.split('?').collect();
    let mut parts_iter = url_parts.iter();

    let _ = parts_iter.next();
    if let Some(sub_string) = parts_iter.next() {
        if sub_string.is_empty() {
            eprintln!(
                "query parameter is empty because query string only contains
            '?'"
            );
        } else {
            let key_value: Vec<&str> = sub_string.split('&').collect();

            for pair in key_value {
                let mut sub = pair.split('=');
                let key = sub.next().unwrap_or_else(|| {
                    panic!(
                        "we should have a key for the parameter key but got {}",
                        url_string
                    )
                });
                let value = sub.next().unwrap_or_else(|| {
                    panic!("we should have a value for the key but got {}", url_string)
                });
                query.insert(key.to_string(), value.to_string());
            }
        }
    }
    query
}
#[cfg(test)]
mod test {

    extern crate router_macro_derive;

    use super::*;

    #[derive(Debug)]
    struct UserTask {
        id: String,
        query: IndexMap<String, String>,
        children: String,
    }
    #[derive(Debug)]
    struct UserTask2 {
        id: String,
        query: IndexMap<String, String>,
    }
    #[derive(Debug)]
    struct UserTask3 {
        query: IndexMap<String, String>,
        children: String,
    }
    #[derive(Debug)]
    struct UserTask4 {
        id: String,
        children: String,
    }
    #[test]
    fn test_extract_id_param() {
        let url_string = "/12/stuff?user=arn&role=programmer";

        let id_param = extract_id_parameter(&url_string.to_string());

        assert_eq!(id_param, "12");

        let url_string = "/12?user=arn&role=programmer";

        let id_param = extract_id_parameter(&url_string.to_string());

        assert_eq!(id_param, "12");
    }

    #[test]
    fn test_extract_query_params() {
        let url_string = "/12/stuff?user=arn&role=programmer";

        let params = extract_query_params(&url_string.to_string());
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();

        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());
        assert_eq!(params, query_to_compare);

        let url_string = "/12/stuff";

        let params = extract_query_params(&url_string.to_string());
        let query_to_compare: IndexMap<String, String> = IndexMap::new();

        assert_eq!(params, query_to_compare);

        let url_string = "/12/stuff?";

        let params = extract_query_params(&url_string.to_string());
        let query_to_compare: IndexMap<String, String> = IndexMap::new();

        assert_eq!(params, query_to_compare);
    }
    #[test]
    fn test_extract_children() {
        let url_string = "/12/stuff?user=arn&role=programmer";
        let children = extract_children_string(url_string.to_string(), Some("12".to_string()));
        assert_eq!(children, "/stuff?user=arn&role=programmer");

        let url_string = "/12/stuff?user=arn&role=programmer";
        let children = extract_children_string(url_string.to_string(), None);
        assert_eq!(children, "/12/stuff?user=arn&role=programmer");
    }
    #[test]
    fn test_string_to_index_map() {
        let string = "/task?user=arn&role=programmer";

        let query = extract_url_payload(string.to_string(), false, true, false);

        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();

        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(query.1.unwrap(), query_to_compare);
    }

    #[test]
    fn test_strings() {
        let string = "/task/12?user=arn&role=programmer";

        let task: UserTask2 = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string(), true, true, false))
            .map(|(id, query, _)| (id.unwrap(), query.unwrap()))
            .map(|(id, query)| UserTask2 { id, query })
            .unwrap();

        eprintln!("{:?}", task);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(task.id, "12");
        assert_eq!(task.query, query_to_compare);

        let string = "?user=arn&role=programmer";
        let query = extract_url_payload(string.to_string(), false, true, true);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(query.1.unwrap(), query_to_compare);
    }
    #[test]
    fn test_strings_with_id_param_and_children_and_query() {
        let string = "/task/12/stuff?user=arn&role=programmer";

        let task: UserTask = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string(), true, true, true))
            .map(|(id, query, children)| (id.unwrap(), query.unwrap(), children.unwrap()))
            .map(|(id, query, children)| UserTask {
                id,
                query,
                children,
            })
            .unwrap();

        eprintln!("{:?}", task);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(task.id, "12");
        assert_eq!(task.query, query_to_compare);
        assert_eq!(task.children, "/stuff?user=arn&role=programmer")
    }
    #[test]
    fn test_strings_with_id_param_and_children() {
        let string = "/task/12/stuff?user=arn&role=programmer";

        let task: UserTask4 = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string(), true, false, true))
            .map(|(id, query, children)| (id.unwrap(), query, children.unwrap()))
            .map(|(id, _, children)| UserTask4 { id, children })
            .unwrap();

        assert_eq!(task.id, "12");
        assert_eq!(task.children, "/stuff?user=arn&role=programmer")
    }
    #[test]
    fn test_strings_with_children_and_query() {
        let string = "/task/stuff?user=arn&role=programmer";

        let task: UserTask3 = string
            .trim_start_matches('/')
            .strip_prefix("task")
            .map(|rest| extract_url_payload(rest.to_string(), false, true, true))
            .map(|(id, query, children)| (id, query.unwrap(), children.unwrap()))
            .map(|(_, query, children)| UserTask3 { query, children })
            .unwrap();

        eprintln!("{:?}", task);
        let mut query_to_compare: IndexMap<String, String> = IndexMap::new();
        query_to_compare.insert("user".to_string(), "arn".to_string());
        query_to_compare.insert("role".to_string(), "programmer".to_string());

        assert_eq!(task.query, query_to_compare);
        assert_eq!(task.children, "/stuff?user=arn&role=programmer")
    }
}
