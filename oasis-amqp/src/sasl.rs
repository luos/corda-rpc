use oasis_amqp_macros::amqp;
use serde::{self, Deserialize, Serialize};
use serde_bytes::Bytes;
use serde_repr::*;
use crate::Described;

#[amqp]
#[derive(Debug, PartialEq, Serialize)]
pub enum Frame<'a> {
    Mechanisms(Mechanisms),
    Init(Init<'a>),
    Outcome(Outcome<'a>),
}

#[amqp(descriptor("amqp:sasl-mechanisms:list", 0x0000_0000_0000_0040))]
#[derive(Debug, PartialEq, Serialize)]
pub struct Mechanisms {
    pub sasl_server_mechanisms: Vec<Mechanism>,
}

#[amqp(descriptor("amqp:sasl-init:list", 0x0000_0000_0000_0041))]
#[derive(Debug, PartialEq, Serialize)]
pub struct Init<'a> {
    pub mechanism: Mechanism,
    #[serde(borrow)]
    pub initial_response: Option<&'a Bytes>,
    pub hostname: Option<&'a str>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mechanism {
    Anonymous,
    Plain,
    ScramSha1,
}

#[amqp(descriptor("amqp:sasl-outcome:list", 0x0000_0000_0000_0044))]
#[derive(Debug, PartialEq, Serialize)]
pub struct Outcome<'a> {
    pub code: Code,
    #[serde(borrow)]
    pub additional_data: Option<&'a Bytes>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum Code {
    Ok = 0,
    Auth = 1,
    Sys = 2,
    SysPerm = 3,
    SysTemp = 4,
}
