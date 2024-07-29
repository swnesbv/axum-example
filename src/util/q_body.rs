use axum::{
    async_trait,
    extract::{Request, FromRequest},
    response::{Response, IntoResponse},
    body::{Bytes},
};

pub struct InputBody(pub Bytes);

#[async_trait]
impl<S> FromRequest<S> for InputBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(Self(body))
    }
}