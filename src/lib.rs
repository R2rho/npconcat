use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use std::fmt::Write;
use std::cmp;
// use list_comprehension_macro::comp;
// use std::iter::zip;
use flame;
use std::fs::File;

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
fn concat6(list_a: &PyList, list_b: &PyList) -> PyObject {
    flame::start("concat6");
    let a = list_a.iter().map(|x| x.extract::<String>().unwrap());
    let b = list_b.iter().map(|x| x.extract::<String>().unwrap());

    let result: Vec<String> = a.zip(b)
        .collect::<Vec<(String, String)>>()
        .into_par_iter()
        .map(|(a_val, b_val)| concat_string!(a_val, b_val))
        .collect();
    
    flame::end("concat6");
    flame::dump_html(&mut File::create("concat6_flamegraph.html").unwrap()).unwrap();

    Python::with_gil(|py| {
        // list_a.to_object(py)
        PyList::new(py, &result).to_object(py)
    })
}

#[pyfunction]
fn pass_through(list_a: &PyList, _list_b: &PyList) -> PyObject {
    Python::with_gil(|py| {
        list_a.to_object(py)
    })
}
/// A Python module implemented in Rust.
#[pymodule]
fn npconcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat2, m)?)?;
    m.add_function(wrap_pyfunction!(concat6, m)?)?;
    m.add_function(wrap_pyfunction!(format, m)?)?;
    m.add_function(wrap_pyfunction!(pass_through, m)?)?;
    Ok(())
}