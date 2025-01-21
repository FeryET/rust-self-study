use std::fmt::Debug;

use regex::{Captures, Error, Regex};

use crate::{
    common::{HttpError, HttpMethod},
    message::{HttpRequest, HttpResponse},
};

const ROUTER_IDENT: &'static str = "<ROUTE>";

pub type HttpRouterFunc = fn(&HttpRequest) -> Result<HttpResponse, HttpError>;

pub struct HttpRouterMapBuilder {
    routers: Vec<(HttpMethod, String, HttpRouterFunc)>,
}

impl HttpRouterMapBuilder {
    pub fn add_router(&mut self, method: HttpMethod, path: &str, func: HttpRouterFunc) -> () {
        self.routers.push((method, path.to_string(), func));
    }
    pub fn build(&self) {}

    fn sort_routers() {}
}

fn map_strpath_to_regexpath(path: String) -> Result<Regex, Error> {
    let param_pattern = Regex::new("<(\\w+)>").unwrap();
    Regex::new(
        param_pattern
            .replace_all(&path, |caps: &Captures| format!("(?P<{}>\\w+)", &caps[1]))
            .to_string()
            .as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::map_strpath_to_regexpath;

    #[test]
    fn test_map_strpath_to_regex_compile_one_middle_parameter_pass() {
        let case = "/my/path/<id>/number";
        let expected = "/my/path/(?P<id>\\w+)/number";
        let result = map_strpath_to_regexpath(case.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), expected)
    }

    #[test]
    fn test_map_strpath_to_regex_compile_one_end_parameter_pass() {
        let case = "/my/path/to/<id>";
        let expected = "/my/path/to/(?P<id>\\w+)";
        let result = map_strpath_to_regexpath(case.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), expected)
    }

    #[test]
    fn test_map_strpath_to_regex_compile_two_parameters_pass() {
        let case = "/my/path/to/<id>/of/<item>";
        let expected = "/my/path/to/(?P<id>\\w+)/of/(?P<item>\\w+)";
        let result = map_strpath_to_regexpath(case.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), expected)
    }

    #[test]
    fn test_map_strpath_to_regex_compile_two_consecetuive_parameters_pass() {
        let case = "/my/path/to/<id>/<item>";
        let expected = "/my/path/to/(?P<id>\\w+)/(?P<item>\\w+)";
        let result = map_strpath_to_regexpath(case.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), expected)
    }

    #[test]
    fn test_map_strpath_to_regex_generated_parses_parameters_by_names() {
        let pattern = map_strpath_to_regexpath(String::from("/my/path/to/<id>/<items>")).unwrap();
        let case = "/my/path/to/100/songs";

        let matches = pattern.captures(case);
        assert!(matches.is_some());

        let matches = matches.unwrap();
        assert_eq!(matches.name("id").unwrap().as_str(), "100");
        assert_eq!(matches.name("items").unwrap().as_str(), "songs");
    }
}
