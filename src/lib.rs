use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::fmt::Write;
use std::cmp;
// use list_comprehension_macro::comp;
// use std::iter::zip;

#[macro_use(concat_string)]
extern crate concat_string;

#[pyfunction]
pub fn format(array: PyReadonlyArray1<f64>, precision: usize) -> PyResult<Vec<String>> {
    
    let result = array
    .iter()?
    .map(|v| {
        let val = v.and_then(PyAny::extract::<f64>)
                   .expect("Could not parse float values f64");
        let mut string = String::new();
        match val.partial_cmp(&0.0) {
            Some(cmp::Ordering::Greater) => write!(string, " {:.precision$}", val, precision=precision),
            Some(cmp::Ordering::Equal) => write!(string, "{:.precision$}", val, precision=precision),
            Some(cmp::Ordering::Less) => write!(string, "{:.precision$}", val, precision=precision),
            None => Ok(()),
        }.unwrap();
        string
    })
    .collect::<Vec<_>>();
Ok(result)
}


#[pyfunction]
fn concat2(a: Vec<String>, b: Vec<String>) -> PyResult<Vec<String>> {
    // let result = comp![concat_string!(x, y) for (x, y) in zip(a, b)];

    let mut result: Vec<String> = Vec::with_capacity(a.len());

    a.par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)| concat_string!(a_val,b_val))
        .collect_into_vec(&mut result);

    Ok(result)
}

#[pyfunction]
fn concat3(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> PyResult<Vec<String>> {
    let mut result: Vec<String> = Vec::with_capacity(a.len());

    a.par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)| {
            let mut concat_bytes = Vec::with_capacity(a_val.len() + b_val.len());
            concat_bytes.extend_from_slice(a_val);
            concat_bytes.extend_from_slice(b_val);
            String::from_utf8(concat_bytes).unwrap()
        })
        .collect_into_vec(&mut result);

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn npconcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat2, m)?)?;
    m.add_function(wrap_pyfunction!(concat3, m)?)?;
    m.add_function(wrap_pyfunction!(format, m)?)?;
    Ok(())
}