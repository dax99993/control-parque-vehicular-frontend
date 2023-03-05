


#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    // Http Errors
    UnathorizedError,
    ForbiddenError,
    BadRequestError,
    NotFoundError,
    InternalServerError,
    FailedRequestError,
    //
    DeserializeError,
    //Catch all error
    UnexpectedError,
}

