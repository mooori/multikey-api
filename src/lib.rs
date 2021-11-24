use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(try_from = "String", bound = "T: FromStr")]
pub struct QueryVec<T: FromStr>(pub Vec<T>);

impl<T: FromStr> TryFrom<String> for QueryVec<T> {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        println!("string: {}", string);

        if string.is_empty() {
            return Ok(Self(vec![]));
        }

        string
            .split(',')
            .map(|s| s.parse().map_err(|_| "unable to parse".to_string()))
            .collect::<Result<Vec<T>, String>>()
            .map(Self)
    }
}

impl<T: FromStr> TryFrom<Vec<String>> for QueryVec<T> {
    type Error = String;

    fn try_from(x: Vec<String>) -> Result<Self, Self::Error> {
        println!("Vec<String>: {:?}", x);
        todo!();
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FooQuery {
    pub id: u64,
    pub pets: QueryVec<String>,
}
