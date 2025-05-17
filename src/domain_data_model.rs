//! This file contains definition of the domain data model used internally by the application.

pub struct EstimateGas {
    pub from: Option<String>,
    pub to: String,
    pub value: Option<String>,
    pub data: Option<String>,
    pub block: Option<String>,
}
