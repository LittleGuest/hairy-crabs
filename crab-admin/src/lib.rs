mod auth;
mod result;
mod tool_gen;

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
