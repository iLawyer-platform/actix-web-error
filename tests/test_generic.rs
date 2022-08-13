use actix_web::{http::StatusCode, ResponseError};
use std::fmt::Display;

trait MyTrait: Display {
    fn status() -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, thiserror::Error)]
#[error("my error")]
struct MyStruct;

impl MyTrait for MyStruct {}

impl ResponseError for MyStruct {}

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[error("Error: {0}")]
#[status(transparent)]
struct MyError<T>(T);

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[status(400)]
enum MyEnum<T: MyTrait> {
    #[error("Bad")]
    Bad,
    #[error("Delegate")]
    #[status(transparent)]
    Delegate(T),
}

#[test]
fn structs() {
    use actix_web::body::MessageBody;

    let e = MyError(MyStruct);
    assert_eq!(e.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        e.error_response().into_body().try_into_bytes().unwrap(),
        r#"{"error":"Error: my error"}"#
    );
}

#[test]
fn enums() {
    use actix_web::body::MessageBody;

    let e1: MyEnum<MyStruct> = MyEnum::Bad;
    let e2 = MyEnum::Delegate(MyStruct);
    assert_eq!(e1.status_code(), StatusCode::BAD_REQUEST);
    assert_eq!(e2.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        e1.error_response().into_body().try_into_bytes().unwrap(),
        r#"{"error":"Bad"}"#
    );
    assert_eq!(
        e2.error_response().into_body().try_into_bytes().unwrap(),
        r#"{"error":"Delegate"}"#
    );
}
