pub mod gateway {

    use serde::{Deserialize, Serialize};
    use serde_json::error::Result;
    use std::collections::HashMap;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Debug)]
    pub struct Request {
        pub resource: String,
        pub path: String,
        pub httpMethod: String,
        pub headers: Option<HashMap<String, String>>,
        pub body: String,
    }

    impl Request {
        pub fn body<'a, T: Deserialize<'a>>(&'a self) -> Result<T> {
            serde_json::from_str(self.body.as_str())
        }
    }

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    pub struct Response {
        pub isBase64Encoded: bool,
        pub statusCode: u16,
        pub headers: Option<HashMap<String, String>>,
        pub body: String,
    }

    impl From<crate::io::Response> for Response {
        fn from(s: crate::io::Response) -> Self {
            Self {
                body: serde_json::to_string(&s).unwrap(),
                headers: Some(HashMap::from([("test".into(), "plop".into())])),
                isBase64Encoded: false,
                statusCode: 200,
            }
        }
    }
}
