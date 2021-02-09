mod router;
mod routing_module;
#[cfg(test)]
mod test {

    extern crate router_derive;
    extern crate seed_routing;
    use router_derive::*;
    use seed::prelude::{IndexMap, *};
    use seed_routing::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, Clone, ParseUrl, WithDefaultRoute)]
    pub enum ExampleRoutes {
        Other {
            id: String,
            children: Settings,
        },
        Admin {
            query: IndexMap<String, String>,
        },

        Dashboard(DashboardRoutes),
        Profile {
            id: String,
        },
        #[default_route]
        NotFound,
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum DashboardRoutes {
        #[as_path = "my_stuff"]
        Stuff { id: String },
        #[as_path = ""]
        Root,
    }
    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum AsPathCheck {
        #[as_path = "foobar"]
        Things,
        FizzBuzz,
    }

    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum Settings {
        Api(Apis),
        Projects {
            id: String,
            query: IndexMap<String, String>,
            children: Apis,
        },
    }

    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum Apis {
        Facebook,
        Google,
        Microsoft,
    }
    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum Routes {
        Stuff,
        Olives { children: ProcessRoutes },
    }

    #[derive(Debug, PartialEq, Clone, ParseUrl)]
    pub enum ProcessRoutes {
        DoingProcessN1,
        Flowering {
            id: String,                      /* 2019/units/Camor  or
                                              * 2019/units/Camor/farms/Cacalsoh */
            query: IndexMap<String, String>, // from-year=2018&kpi=Average
        },
    }

    #[wasm_bindgen_test]
    fn test_url_for_tatrix() {
        let url: Url = "http://localhost/olives/flowering/2019/units/Camor/farms/Cacalsoh?from-year=2018&kpi=Average"
            .parse()
            .unwrap();

        let mut query: IndexMap<String, String> = IndexMap::new();
        query.insert("from-year".to_string(), "2018".to_string());
        query.insert("kpi".to_string(), "Average".to_string());
        let url_to_compare = Routes::Olives {
            children: ProcessRoutes::Flowering {
                id: "2019/units/Camor/farms/Cacalsoh".to_string(),
                query,
            },
        }
        .to_url();
        assert_eq!(url, url_to_compare);

        let url: Url = "http://localhost/olives/flowering/2019/units/Camor"
            .parse()
            .unwrap();

        let query: IndexMap<String, String> = IndexMap::new();

        let url_to_compare = Routes::Olives {
            children: ProcessRoutes::Flowering {
                id: "2019/units/Camor".to_string(),
                query,
            },
        }
        .to_url();
        assert_eq!(url, url_to_compare);

        let url: Url = "http://localhost/olives/flowering".parse().unwrap();

        let query: IndexMap<String, String> = IndexMap::new();

        let url_to_compare = Routes::Olives {
            children: ProcessRoutes::Flowering {
                id: "".to_string(),
                query,
            },
        }
        .to_url();
        assert_eq!(url, url_to_compare);
    }

    #[wasm_bindgen_test]
    fn test_to_url() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Admin {
            query: query_search.clone(),
        }
        .to_url();
        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        assert_eq!(url, url_to_compare);

        let query_search: IndexMap<String, String> = IndexMap::new();

        let url = ExampleRoutes::Admin {
            query: query_search.clone(),
        }
        .to_url();
        let url_to_compare: Url = "/admin".parse().unwrap();
        assert_eq!(url, url_to_compare);

        let url: Url = ExampleRoutes::Profile {
            id: "1".to_string(),
        }
        .to_url();

        let url_to_compare: Url = "/profile/1".parse().unwrap();
        assert_eq!(url, url_to_compare);
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url: Url = ExampleRoutes::Other {
            id: "2".to_string(),
            children: Settings::Projects {
                id: "14".to_string(),
                query: query_search.clone(),
                children: Apis::Facebook,
            },
        }
        .to_url();

        let url_to_compare: Url =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway"
                .parse()
                .unwrap();
        assert_eq!(url, url_to_compare);

        let url: Url = ExampleRoutes::Other {
            id: "2".to_string(),
            children: Settings::Api(Apis::Facebook),
        }
        .to_url();

        let url_to_compare: Url = "/other/2/api/facebook".parse().unwrap();
        assert_eq!(url, url_to_compare);
    }

    #[wasm_bindgen_test]
    fn test_from_path_to_enum() {
        let string = "/admin?user=arn&role=baby_programmer&location=norway";

        let route = ExampleRoutes::parse_path(string).unwrap();
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        assert_eq!(
            route,
            ExampleRoutes::Admin {
                query: query_search
            }
        );

        let string = "/admin?";

        let route = ExampleRoutes::parse_path(string).unwrap();
        let query_search: IndexMap<String, String> = IndexMap::new();
        assert_eq!(
            route,
            ExampleRoutes::Admin {
                query: query_search
            }
        );

        let string = "/admin?";

        let route = ExampleRoutes::parse_path(string).unwrap();
        let query_search: IndexMap<String, String> = IndexMap::new();
        assert_eq!(
            route,
            ExampleRoutes::Admin {
                query: query_search
            }
        );

        let string = "/profile/1/repos";

        let route = ExampleRoutes::parse_path(string).unwrap();
        assert_eq!(
            route,
            ExampleRoutes::Profile {
                id: "1".to_string(),
            }
        );

        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());

        let string_to_compare =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway";
        assert_eq!(
            ExampleRoutes::parse_path(string_to_compare).unwrap(),
            ExampleRoutes::Other {
                id: "2".to_string(),
                children: Settings::Projects {
                    id: "14".to_string(),
                    query: query.clone(),
                    children: Apis::Facebook
                },
            }
        );
    }
    #[wasm_bindgen_test]
    fn test_convert_to_url() {
        let mut query_search: IndexMap<String, String> = IndexMap::new();

        query_search.insert("user".to_string(), "arn".to_string());
        query_search.insert("role".to_string(), "baby_programmer".to_string());
        query_search.insert("location".to_string(), "norway".to_string());
        let url = ExampleRoutes::Dashboard(DashboardRoutes::Root).to_url();
        let url_to_compare: Url = "/dashboard/".parse().unwrap();

        assert_eq!(url, url_to_compare);
        let url = ExampleRoutes::Admin {
            query: query_search,
        }
        .to_url();
        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        assert_eq!(url, url_to_compare);

        let url = ExampleRoutes::Admin {
            query: IndexMap::new(),
        }
        .to_url();
        let url_to_compare: Url = "/admin".parse().unwrap();
        assert_eq!(url, url_to_compare);

        let url = ExampleRoutes::Admin {
            query: IndexMap::new(),
        }
        .to_url();
        let url_to_compare: Url = "/admin?".parse().unwrap();

        let string_url = url.to_string();
        assert_eq!(string_url, "/admin");
        assert_eq!(url, url_to_compare);
    }
    #[wasm_bindgen_test]
    fn test_convert_from_url() {
        let url_to_compare: Url = "/dashboard/".parse().unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Root);
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());
        let route = ExampleRoutes::Admin { query };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/admin".parse().unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();

        let query: IndexMap<String, String> = IndexMap::new();
        let route = ExampleRoutes::Admin { query };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/profile/1".parse().unwrap();

        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Profile {
            id: "1".to_string(),
        };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/dashboard/my_stuff/123".parse().unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Stuff {
            id: "123".to_string(),
        });
        assert_eq!(route.to_url(), url_to_compare);
        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());

        let url_to_compare: Url =
            "/other/2/projects/14/facebook?user=arn&role=baby_programmer&location=norway"
                .parse()
                .unwrap();
        assert_eq!(
            ExampleRoutes::from_url(url_to_compare).unwrap(),
            ExampleRoutes::Other {
                id: "2".to_string(),
                children: Settings::Projects {
                    id: "14".to_string(),
                    query: query.clone(),
                    children: Apis::Facebook
                },
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_from_url() {
        let stuff_ref: Url = "/dashboard/my_stuff/123".parse().unwrap();
        let things_ref: Url = "/foobar".parse().unwrap();
        let fizz_ref: Url = "/fizz_buzz".parse().unwrap();
        let root_ref: Url = "/dashboard/".parse().unwrap();

        let stuff_ref_url = ExampleRoutes::from_url(stuff_ref).unwrap();
        let things_ref_url = AsPathCheck::from_url(things_ref).unwrap();
        let fizz_ref_url = AsPathCheck::from_url(fizz_ref).unwrap();
        let root_ref_url = ExampleRoutes::from_url(root_ref).unwrap();

        assert_eq!(
            stuff_ref_url,
            ExampleRoutes::Dashboard(DashboardRoutes::Stuff {
                id: "123".to_string()
            })
        );
        assert_eq!(things_ref_url, AsPathCheck::Things);
        assert_eq!(fizz_ref_url, AsPathCheck::FizzBuzz);
        assert_eq!(
            root_ref_url,
            ExampleRoutes::Dashboard(DashboardRoutes::Root)
        );
    }

    #[wasm_bindgen_test]
    fn test_convert_from_url_with_children() {
        let url_to_compare: Url = "/dashboard/".parse().unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Root);
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/admin?user=arn&role=baby_programmer&location=norway"
            .parse()
            .unwrap();
        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let mut query: IndexMap<String, String> = IndexMap::new();

        query.insert("user".to_string(), "arn".to_string());
        query.insert("role".to_string(), "baby_programmer".to_string());
        query.insert("location".to_string(), "norway".to_string());
        let route = ExampleRoutes::Admin { query };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/profile/1".parse().unwrap();

        let route_to_compare = ExampleRoutes::from_url(url_to_compare).unwrap();
        let route = ExampleRoutes::Profile {
            id: "1".to_string(),
        };
        assert_eq!(route, route_to_compare);

        let url_to_compare: Url = "/dashboard/my_stuff/123".parse().unwrap();
        let route = ExampleRoutes::Dashboard(DashboardRoutes::Stuff {
            id: "123".to_string(),
        });

        assert_eq!(url_to_compare, route.to_url());
    }

    #[wasm_bindgen_test]
    fn test_default_route() {
        assert_eq!(ExampleRoutes::default(), ExampleRoutes::NotFound);
    }
}
