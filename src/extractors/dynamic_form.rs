use std::pin::Pin;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use jelly::error::error::Error;
use std::ops::{Deref, DerefMut};
use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};
use jelly::actix_web::{FromRequest, HttpRequest, dev::Payload};

use crate::helpers::form::IntoInner;

#[derive(Debug)]
pub struct FormGuard<Ex, O>(O, PhantomData<Ex>);

impl<Ex, O> FormGuard<Ex, O> {
    pub fn into_inner(self) -> O {
        self.0
    }
}

impl<Ex, O> Deref for FormGuard<Ex, O> {
    type Target = O;

    fn deref(&self) -> &O {
        &self.0
    }
}

impl<Ex, O> DerefMut for FormGuard<Ex, O>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Ex, O, M> FromRequest for FormGuard<Ex, O>
where
    Ex: FromRequest + IntoInner<Target = M>, M:Into<O>,
    Ex::Error: std::fmt::Debug
{
    type Error = Error;
    type Future = FormGuardFuture<Ex::Future, Ex, O>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        FormGuardFuture {
            inner: Box::pin(Ex::from_request(req, payload)),
            _marker: Default::default()
        }
    }
}

pub struct FormGuardFuture<Fut, Ex, O> {
    inner: Pin<Box<Fut>>,
    _marker: PhantomData<Box<(Ex, O)>>
    // _marker: PhantomData<Ex>
}

// use this if you decide to remove the box in PhantomData above
// impl<Fut, Ex> Unpin for FormGuardFuture<Fut, Ex> {} 

impl<Fut, Ex, O, M, E> Future for FormGuardFuture<Fut, Ex, O>
where
    E: std::fmt::Debug,
    Ex: IntoInner<Target = M>, M: Into<O>,
    Fut: Future<Output = Result<Ex, E>>
{
    type Output = Result<FormGuard<Ex, O>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(Ok(out)) => {
                let form = out.into_inner().into();
                Poll::Ready(Ok(FormGuard(form, Default::default())))
            }
            Poll::Ready(Err(err)) => {
                dbg!("============== An error occured in FormGuardFuture: =========", err);
                Poll::Ready(
                    Err(Error::Generic("Unable to resolve inner type".into()))
                )
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Ex, O> CsrfGuarded for FormGuard<Ex, O> where O:CsrfGuarded{
    fn csrf_token(&self) -> &CsrfToken {
        self.0.csrf_token()
    }
}
