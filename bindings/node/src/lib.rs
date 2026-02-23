#![deny(clippy::all)]
use napi_derive::napi;
use napi::{Error, Result, Status};
use wareki_core::{to_wareki as core_to_wareki, from_wareki as core_from_wareki};

#[napi(object)]
pub struct Wareki {
  pub era_name: String,
  pub year: u32,
}

#[napi]
pub fn to_wareki(year: i32, month: u32, day: u32) -> Result<Wareki> {
    match core_to_wareki(year, month, day) {
        Ok(w) => Ok(Wareki {
            era_name: w.era_name().to_string(),
            year: w.year,
        }),
        Err(e) => Err(Error::new(Status::InvalidArg, e)),
    }
}

// Node.js側には文字列 (ISO 8601) で返し、JS側で Date オブジェクトにするか、
// 日時は `epoch milliseconds` なので f64 で返すことができます。
// ここではJS側の Date のパースに使える `YYYY-MM-DD` フォーマットの文字列を返すことにします。
#[napi]
pub fn from_wareki(era_name: String, year: u32, month: u32, day: u32) -> Result<String> {
    match core_from_wareki(&era_name, year, month, day) {
        Ok(d) => Ok(d.format("%Y-%m-%d").to_string()),
        Err(e) => Err(Error::new(Status::InvalidArg, e)),
    }
}
