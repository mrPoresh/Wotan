use std::convert::From;
use color_eyre::Report;
use actix_web::error::ResponseError;
use serde::{Serialize, Serializer};


#[derive(Debug, PartialEq, Eq)] //  Equivalence relation
pub struct AppErrorCode(i32);

#[derive(Debug, Serialize)]
pub struct AppError {

    message: String,
    code: AppErrorCode,

}

impl AppErrorCode {}

impl Serialize for AppErrorCode {}

impl AppError {}

impl From<Report> for AppError {}

impl From<AppErrorCode> for AppError {}

impl ResponseError for AppError {}