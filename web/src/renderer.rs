use actix_utils::future::{ready, Ready};
use actix_web::{dev, error, web, FromRequest, HttpRequest};
use actix_web_lab::respond::Html;
pub struct MiniJinjaRenderer {
    pub tmpl_env: web::Data<minijinja_autoreload::AutoReloader>,
}

impl MiniJinjaRenderer {
    pub fn render(
        &self,
        tmpl: &str,
        ctx: impl Into<minijinja::value::Value>,
    ) -> actix_web::Result<Html> {
        self.tmpl_env
            .acquire_env()
            .map_err(|e| {
                error::ErrorInternalServerError(format!(
                    "could not acquire template env: {}",
                    e.to_string()
                ))
            })?
            .get_template(tmpl)
            .map_err(|e| {
                error::ErrorInternalServerError(format!(
                    "could not find template {}",
                    e.to_string()
                ))
            })?
            .render(ctx.into())
            .map(Html)
            .map_err(|err| {
                log::error!("{err}");
                error::ErrorInternalServerError("template error")
            })
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let tmpl_env = <web::Data<minijinja_autoreload::AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { tmpl_env }))
    }
}
