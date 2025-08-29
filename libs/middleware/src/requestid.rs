use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{extract::Request, response::Response};
use tower::{Layer, Service};
use tracing::{Instrument, Span, error, info, info_span};

pub const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Debug, Clone)]
pub struct LogRequestId<S> {
    inner: S,
    span: Arc<Span>,
}

impl<S> LogRequestId<S> {
    pub fn new(inner: S, span: Arc<Span>) -> Self {
        LogRequestId { inner, span }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LogRequestId<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // Pin and box because instrumentation changes the future's type from S::Future
    // to Instrumented<S::Future>. Boxing erases the concrete type so we can
    // return Pin<Box<dyn Future<...>>> as required by our Service trait.
    // .instrument() attaches the span context to the future so all downstream
    // execution (route handlers, other middleware) happens within this span.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let request_id = req.headers().get(REQUEST_ID_HEADER);
        println!("{:?}", req.headers());

        let child_span = match request_id {
            Some(request_id) => {
                info_span!(parent: self.span.as_ref(), "http_request", request_id = ?request_id)
            }
            None => {
                error!("could not extract request_id");
                info_span!(parent: self.span.as_ref(), "http_request")
            }
        };

        info!("set request id");

        // Box the instrumented future to match our Service::Future type.
        // The span will be active for all downstream execution.
        Box::pin(self.inner.call(req).instrument(child_span))
    }
}

#[derive(Debug, Clone)]
pub struct LogRequestIdLayer {
    span: Arc<Span>,
}

impl LogRequestIdLayer {
    pub fn new(span: Arc<Span>) -> Self {
        LogRequestIdLayer { span }
    }
}

impl<S> Layer<S> for LogRequestIdLayer {
    type Service = LogRequestId<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogRequestId::new(inner, self.span.clone())
    }
}
