use std::pin::Pin;
use std::fmt::Debug;
use std::future::Future;
use std::task::{Context, Poll};
use jelly::error::error::Error;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};
use jelly::actix_web::{FromRequest, HttpRequest, dev::Payload};


#[derive(Debug, Deserialize, Serialize)]
pub struct MultipartGuard<T>(T);

impl<T> MultipartGuard<T> {
    /// Unwrap into inner `T` value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for MultipartGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MultipartGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromRequest for MultipartGuard<T>
where
    T: FromRequest
{
    type Error = Error;
    type Future = MultipartGuardFuture<T::Future>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        MultipartGuardFuture {
            inner: Box::pin(T::from_request(req, payload)),
        }
    }
}

pub struct MultipartGuardFuture<Fut> {
    inner: Pin<Box<Fut>>
}

impl<F, T, E> Future for MultipartGuardFuture<F>
where
    F: Future<Output = Result<T, E>>
{
    type Output = Result<MultipartGuard<T>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(Ok(out)) => Poll::Ready(Ok(MultipartGuard(out))),
            Poll::Ready(Err(_e)) => {
                Poll::Ready(
                    Err(Error::Generic("Unable to resolve inner type".into()))
                )
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<I, T> CsrfGuarded for MultipartGuard<I> 
where 
    T: CsrfGuarded + 'static,
    I: Deref<Target = T>
{
    fn csrf_token(&self) -> &CsrfToken {
        self.0.csrf_token()
    }
}
