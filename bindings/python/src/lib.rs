use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use chrono::Datelike;
use wareki_core::{to_wareki as core_to_wareki, from_wareki as core_from_wareki};

/// 和暦情報を保持するPythonクラス
#[pyclass]
pub struct Wareki {
    #[pyo3(get)]
    pub era_name: String,
    #[pyo3(get)]
    pub year: u32,
}

#[pymethods]
impl Wareki {
    fn __repr__(&self) -> String {
        format!("{}{}年", self.era_name, self.year)
    }
}

/// 西暦から和暦への変換
/// to_wareki(2026, 2, 23) -> Wareki
#[pyfunction]
fn to_wareki(year: i32, month: u32, day: u32) -> PyResult<Wareki> {
    match core_to_wareki(year, month, day) {
        Ok(w) => Ok(Wareki {
            era_name: w.era_name().to_string(),
            year: w.year,
        }),
        Err(e) => Err(PyValueError::new_err(e)),
    }
}

/// 和暦から西暦への変換
/// from_wareki("令和", 8, 2, 23) -> datetime.date
#[pyfunction]
fn from_wareki(py: Python<'_>, era_name: &str, year: u32, month: u32, day: u32) -> PyResult<PyObject> {
    match core_from_wareki(era_name, year, month, day) {
        Ok(d) => {
            let datetime = py.import("datetime")?;
            let date_obj = datetime.getattr("date")?.call1((d.year(), d.month(), d.day()))?;
            Ok(date_obj.into())
        }
        Err(e) => Err(PyValueError::new_err(e)),
    }
}

/// Pythonモジュールの定義
#[pymodule]
fn wareki(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Wareki>()?;
    m.add_function(wrap_pyfunction!(to_wareki, m)?)?;
    m.add_function(wrap_pyfunction!(from_wareki, m)?)?;
    Ok(())
}
