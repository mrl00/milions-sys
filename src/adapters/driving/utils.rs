use actix_web::FromRequest;
use actix_web::dev::Payload;
use actix_web::web;
use futures::future::LocalBoxFuture;
use garde::Validate;
use serde::de::DeserializeOwned;

pub struct ValidatedJson<T>(pub T);

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate<Context = ()> + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = web::Json::<T>::from_request(req, payload);

        Box::pin(async move {
            // desserialização — erro tratado pelo JsonConfig no startup
            let value = fut.await?;

            // validação garde
            value.validate().map_err(|err| {
                let messages: Vec<String> = err
                    .iter()
                    .map(|(path, error)| format!("{path}: {error}"))
                    .collect();

                actix_web::error::InternalError::from_response(
                    "validation_error",
                    actix_web::HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(serde_json::json!({
                            "error": "validation_error",
                            "message": messages.join(", ")
                        })),
                )
            })?;

            Ok(ValidatedJson(value.into_inner()))
        })
    }
}

