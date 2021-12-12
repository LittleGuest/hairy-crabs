use std::{error::Error, fmt::Display};

use poem::http::StatusCode;
use serde::{Deserialize, Serialize};

/// 包装数据
#[derive(Serialize, Deserialize)]
pub struct Res<T>
where
    T: Send + Sync,
    T: Serialize,
{
    /// 状态码
    pub code: u32,
    /// 内容
    pub msg: String,
    /// 返回数据
    pub data: Option<T>,
}

impl<T> Default for Res<T>
where
    T: Send + Sync,
    T: Serialize,
{
    fn default() -> Self {
        Self {
            code: Default::default(),
            msg: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> Display for Res<T>
where
    T: Send + Sync,
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "{}", e.to_string()),
        }
    }
}

pub struct ResBuilder<T>(pub Res<T>)
where
    T: Send + Sync,
    T: Serialize;

impl<T> ResBuilder<T>
where
    T: Send + Sync,
    T: Serialize,
{
    pub fn new() -> Self {
        Self {
            0: Res {
                code: 200,
                msg: "成功".to_string(),
                data: None,
            },
        }
    }

    pub fn ok() -> Self {
        Self {
            0: Res {
                code: 200,
                msg: "成功".to_string(),
                data: None,
            },
        }
    }

    pub fn fail() -> Self {
        Self {
            0: Res {
                code: 500,
                msg: "服务器异常".to_string(),
                data: None,
            },
        }
    }

    pub fn with_code(mut self, code: u32) -> Self {
        self.0.code = code;
        self
    }

    pub fn with_msg(mut self, msg: String) -> Self {
        self.0.msg = msg;
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.0.data = Some(data);
        self
    }

    pub fn build(self) -> Res<T> {
        self.0
    }
}

impl<T> Into<poem::Body> for Res<T>
where
    T: Send + Sync,
    T: Serialize,
{
    fn into(self) -> poem::Body {
        poem::Body::from_json(serde_json::to_string(&self).unwrap()).unwrap()
    }
}

impl<T> poem::IntoResponse for Res<T>
where
    T: Send + Sync,
    T: Serialize,
{
    fn into_response(self) -> poem::Response {
        poem::Response::builder().status(StatusCode::OK).body(self)
    }
}

#[cfg(test)]
mod test {
    use super::ResBuilder;

    #[test]
    fn test_res_builder() {
        let res = ResBuilder::<&str>::ok().build();
        println!("{}", res.code);
        println!("{}", res.msg);
        println!("{:?}", res.data);
    }

    // #[test]
    // fn  test_res(){
    //     fn test_into_response() -> impl poem::IntoResponse {
    //         let res = ResBuilder::<&str>::ok().build();
    //         res
    //     }
    //     let res = test_into_response();
    // }
}
