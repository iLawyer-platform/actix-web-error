

use actix_web::http;
#[error("Error: {0}")]
#[status(transparent)]
struct MyError(MyError2);

impl ::actix_web::ResponseError for MyError {
    fn status_code(&self) -> ::actix_web::http::StatusCode {
        ::actix_web::ResponseError::status_code(&self.0)
    }
    fn error_response(&self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
        ::actix_web::HttpResponseBuilder::new(self.status_code())
            .json(::actix_web_error::__private::JsonErrorSerialize(&self))
    }
}
#[error("Error2")]
#[status(500)]
struct MyError2;
#[automatically_derived]
impl ::core::fmt::Debug for MyError2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "MyError2")
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for MyError2 {}
#[allow(unused_qualifications)]
impl std::fmt::Display for MyError2 {
    #[allow(clippy::used_underscore_binding)]
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated)]
            let Self {} = self;
        __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["Error2"], &[]))
    }
}
impl ::actix_web::ResponseError for MyError2 {
    fn status_code(&self) -> ::actix_web::http::StatusCode {
        ::actix_web::http::StatusCode::from_u16(500u16).unwrap()
    }
    fn error_response(&self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
        ::actix_web::HttpResponseBuilder::new(self.status_code())
            .json(::actix_web_error::__private::JsonErrorSerialize(&self))
    }
}
#[status(400)]
enum MyEnum {
    #[error("a")]
    BadRequest,
    #[error("b")]
    AnotherBadRequest,
    #[error("c")]
    #[status(transparent)]
    Internal(MyError2),
}
#[automatically_derived]
impl ::core::fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MyEnum::BadRequest => ::core::fmt::Formatter::write_str(f, "BadRequest"),
            MyEnum::AnotherBadRequest => ::core::fmt::Formatter::write_str(f, "AnotherBadRequest"),
            MyEnum::Internal(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Internal", &__self_0)
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for MyEnum {}
#[allow(unused_qualifications)]
impl std::fmt::Display for MyEnum {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            MyEnum::BadRequest {} => {
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["a"], &[]))
            }
            MyEnum::AnotherBadRequest {} => {
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["b"], &[]))
            }
            MyEnum::Internal(_0) => {
                __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["c"], &[]))
            }
        }
    }
}
impl ::actix_web::ResponseError for MyEnum {
    fn status_code(&self) -> ::actix_web::http::StatusCode {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match &self {
            MyEnum::BadRequest {} => ::actix_web::http::StatusCode::from_u16(400u16).unwrap(),
            MyEnum::AnotherBadRequest {} => {
                ::actix_web::http::StatusCode::from_u16(400u16).unwrap()
            }
            MyEnum::Internal(_0) => ::actix_web::ResponseError::status_code(_0),
        }
    }
    fn error_response(&self) -> ::actix_web::HttpResponse<::actix_web::body::BoxBody> {
        ::actix_web::HttpResponseBuilder::new(self.status_code())
            .json(::actix_web_error::__private::JsonErrorSerialize(&self))
    }
}

#[test]
fn transparent() {
    use actix_web::{body::MessageBody, ResponseError};

    let e1 = MyError(MyError2);

    assert_eq!(e1.status_code(), http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        e1.error_response().into_body().try_into_bytes().unwrap(),
        r#"{"error":"Error: Error2"}"#
    );
    assert_eq!(
        MyEnum::BadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum::AnotherBadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum::Internal(MyError2).status_code(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        MyEnum::BadRequest
            .error_response()
            .into_body()
            .try_into_bytes()
            .unwrap(),
        r#"{"error":"a"}"#
    );
    assert_eq!(
        MyEnum::AnotherBadRequest
            .error_response()
            .into_body()
            .try_into_bytes()
            .unwrap(),
        r#"{"error":"b"}"#
    );
    assert_eq!(
        MyEnum::Internal(MyError2)
            .error_response()
            .into_body()
            .try_into_bytes()
            .unwrap(),
        r#"{"error":"c"}"#
    );
}
