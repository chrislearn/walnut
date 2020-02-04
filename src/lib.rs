pub mod error;

#[macro_use(lazy_static)]
extern crate lazy_static;

use url::Url;
use multimap::MultiMap;

use error::Error;

pub struct Walnut {
    param_names: ParamNames,
}

pub struct ParamNames {
    pub filter: String,
    pub sort: String,
    pub limit: String,
    pub offset: String,
    pub count: String,
}
impl Default for ParamNames{
    fn default() -> Self {
        ParamNames {
            filter: "filter".to_owned(),
            sort: "sort".to_owned(),
            limit: "limit".to_owned(),
            offset: "offset".to_owned(),
            count: "count".to_owned(),
        }
    }
}

pub struct ParamValues {
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub count: Option<bool>,
}
impl Default for ParamValues{
    fn default() -> Self {
        ParamValues {
            filter: None,
            sort: None,
            limit: None,
            offset: None,
            count: None,
        }
    }
}

impl Walnut {
    pub fn new(pnames: ParamNames) -> Walnut {
        Walnut {
            param_names: pnames,
        }
    }
    pub fn parse_query(&self, query: &str) -> Result<ParamValues, Error> {
        let url = format!("https://www.kenorld.com?{}", query);
        let pairs: MultiMap<String, String> = Url::parse(&url)?.query_pairs().into_owned().collect();
        self.parse_query_pairs(&pairs)
    }
    
    pub fn parse_url(&self, url: &str) -> Result<ParamValues, Error> {
        let pairs: MultiMap<String, String> = Url::parse(url)?.query_pairs().into_owned().collect();
        self.parse_query_pairs(&pairs)
    }
    
    pub fn parse_query_pairs(&self, pairs: &MultiMap<String, String>) -> Result<ParamValues, Error> {
        let mut param = ParamValues {
            filter: None,
            sort: None,
            limit: None,
            offset: None,
            count: None,
        };
        
        if let Some(filter) = pairs.get(&self.param_names.filter) {
            param.filter = Some(parse_filter(filter)?);
        }
        if let Some(sort) = pairs.get(&self.param_names.sort) {
            param.sort = Some(parse_sort(sort)?);
        }
        if let Some(limit) = pairs.get(&self.param_names.limit) {
            param.limit = Some(limit.parse::<i64>()?);
        }
        if let Some(offset) = pairs.get(&self.param_names.offset) {
            param.offset = Some(offset.parse::<i64>()?);
        }
        if let Some(count) = pairs.get(&self.param_names.count) {
            param.count = Some(str_to_bool(count));
        }
        Ok(param)
    }
}

    
fn parse_filter(raw: &str) -> Result<String, Error> {
    // enum Token{
    //     Ident(val: String),
    //     Literal(val: String),
    //     Bool(val: bool),
    //     String(val: String),
    //     Number(val: String),
    //     Null,
    //     Binary_Exp(val: String),
    //     Logical_Exp(val: String),
    //     Period_Code,
    //     DQUOTE_CODE,
    //     OPAREN_CODE,
    // }
    // lazy_static! {
    //     static ref LITERALS: Vec<String> = {
    //         "true|false|null".split('|').map(|s|s.to_string()).collect::<Vec<String>>()
    //     };
    // }
    return Ok(raw.to_owned())
}
fn parse_sort(raw: &str) -> Result<String, Error> {
    let exps: Vec<&str> = raw.split(',').collect();
    let mut result = vec![];
    for exp in exps {
        result.push(parse_sort_column(exp)?);
    }
    Ok(result.join(" "))
}
fn parse_sort_column(exp: &str) -> Result<String, Error> {
    lazy_static! {
        static ref SORTS: Vec<String> = {
            "asc|dsc|desc".split('|').map(|s|s.to_string()).collect::<Vec<String>>()
        };
    }
    let mut parts: Vec<&str> = exp.split('.').collect();
    if parts.len() == 1 {
        let first = parts[0];
        if first.starts_with('-') {
            std::mem::replace(&mut parts[0], &first[1..]);
            parts.push("desc");
        } else {
            parts.push("asc");
        }
    }
    if parts.len() != 2 {
        return Err(Error::General("wrong sort format".into()));
    }
    if !is_ident(parts[0]) {
        return Err(Error::General("wrong sort format".into()));
    }
    if !SORTS.contains(&parts[1].to_owned()) {
        return Err(Error::General("wrong sort format".into()));
    }
    Ok(parts.join(" "))
}
fn is_ident(_raw: &str) -> bool {
    true
}

pub fn str_to_bool(v: &str) -> bool {
    lazy_static! {
        static ref TURE_VALUES: Vec<&'static str> = {
            vec!["true", "1", "yes", "on", "t", "y", "✓"]
        };
    }
    TURE_VALUES.contains(&v)
}