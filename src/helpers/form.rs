use jelly::actix_web::web::Form;

pub trait  IntoInner {
    type Target;

    fn into_inner(self) -> Self::Target;
}

impl<T> IntoInner for Form<T> {
    type Target = T;
    
    fn into_inner(self) -> T {
        self.0
    }
}
