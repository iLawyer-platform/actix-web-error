use actix_web::http;

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[error("Error: {0}")]
#[status(400)]
struct MyError(&'static str);

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[error("Error: {0}")]
#[status(BAD_REQUEST)]
struct MyError2(&'static str);

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
#[status(400)]
enum MyEnum {
    #[error("a")]
    BadRequest,
    #[error("b")]
    AnotherBadRequest,
    #[error("c")]
    #[status(500)]
    Internal,
}

#[derive(Debug, thiserror::Error, actix_web_error::Json)]
enum MyEnum2 {
    #[error("a")]
    #[status(BAD_REQUEST)]
    BadRequest,
    #[error("b")]
    #[status(400)]
    AnotherBadRequest,
    #[error("c")]
    #[status(500)]
    Internal,
}
#[test]
fn basic() {
    use actix_web::{body::MessageBody, ResponseError};

    let e = MyError("xd");
    assert_eq!(e.status_code(), http::StatusCode::BAD_REQUEST);
    let e2 = MyError2("xd");
    assert_eq!(e2.status_code(), http::StatusCode::BAD_REQUEST);
    assert_eq!(
        e.error_response().into_body().try_into_bytes().unwrap(),
        r#"{"error":"Error: xd"}"#
    );
}

#[test]
fn basic_enum() {
    use actix_web::{body::MessageBody, ResponseError};

    assert_eq!(
        MyEnum::BadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum::AnotherBadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum::Internal.status_code(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        MyEnum2::BadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum2::AnotherBadRequest.status_code(),
        http::StatusCode::BAD_REQUEST
    );
    assert_eq!(
        MyEnum2::Internal.status_code(),
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
        MyEnum::Internal
            .error_response()
            .into_body()
            .try_into_bytes()
            .unwrap(),
        r#"{"error":"c"}"#
    );
}
