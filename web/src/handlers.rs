use crate::MiniJinjaRenderer;
use actix_web::{
    dev::ServiceResponse, http::header::ContentType, middleware::ErrorHandlerResponse, web,
    FromRequest, HttpResponse, Responder, Result,
};
use std::collections::HashMap;

pub async fn index(
    tmpl_env: MiniJinjaRenderer,
    _query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<impl Responder> {
    tmpl_env.render(
        "40dex.html",
        minijinja::context! {
            families => database::get_merged().await.unwrap()
        },
    )
}

/// Error handler for a 404 Page not found error.
pub fn not_found<B>(svc_res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res = get_error_response(&svc_res, "Page not found");

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        svc_res.into_parts().0,
        res.map_into_right_body(),
    )))
}

/// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let req = res.request();

    let tmpl_env = MiniJinjaRenderer::extract(req).into_inner().unwrap();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let ctx = minijinja::context! {
        error => error,
        status_code => res.status().as_str(),
    };

    tmpl_env.render("error.html", ctx).map_or_else(|_| fallback(error), |body| body
            .customize()
            .with_status(res.status())
            .respond_to(req)
            .map_into_boxed_body())
}
