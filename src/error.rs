#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    // Http Errors
    UnathorizedError,
    ForbiddenError,
    BadRequestError(String),
    NotFoundError,
    InternalServerError,
    FailedRequestError,
    //
    BytesConvertError,
    DeserializeError,
    //Catch all error
    UnexpectedError,
}

