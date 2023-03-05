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
fn concat7(list_a: &PyList, list_b: &PyList) -> PyObject {
    flame::start("concat7");

    let a: Vec<String> = list_a
    .iter()
    .map(|x| x.extract::<String>().unwrap())
    .collect();

    let b: Vec<String> = list_b
        .iter()
        .map(|x| x.extract::<String>().unwrap())
        .collect();

    let result: Vec<String> = a
        .par_iter()
        .zip(b.par_iter())
        .map(|(a_val, b_val)| concat_string!(a_val, b_val))
        .collect();

    flame::end("concat7");
    flame::dump_html(&mut File::create("concat7_flamegraph.html").unwrap()).unwrap();

    Python::with_gil(|py| {
        PyList::new(py, &result).to_object(py)
    })
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
        PyList::new(py, &result).to_object(py)
    })
}

fn parse_value(py_val: &PyAny, precision: Option<usize>) -> String {
    if let Ok(val) = py_val.extract::<String>() {
        val
    } else if let Ok(val) = py_val.extract::<f64>() {
        match precision {
            Some(p) => format!("{:.1$}", val, p),
            None => val.to_string()
        }
    } else {
        panic!("Unsupported type")
    }
}

#[pyfunction]
fn concat8(list_a: &PyList, list_b: &PyList, precision: Option<usize>) -> PyObject {
    flame::start("concat8");
    let a: Vec<String> = list_a.iter().map(|x| parse_value(x, precision)).collect();
    let b: Vec<String> = list_b.iter().map(|x| parse_value(x, precision)).collect();

    let result: Vec<String> = a
        .into_par_iter()
        .zip(b.into_par_iter())
        .map(|(a_val, b_val)| concat_string!(a_val, b_val))
        .collect();

    flame::end("concat8");
    flame::dump_html(&mut File::create("concat8_flamegraph.html").unwrap()).unwrap();

    Python::with_gil(|py| {
        PyList::new(py, &result).to_object(py)
    })
}

#[pyfunction]
fn concat_pylists(list_a: &PyList, list_b: &PyList, list_c: &PyList, precision: Option<usize>,separator:Option<String>) -> PyObject {
    flame::start("concat8");
    let separator = separator.unwrap_or("".to_string());

    let a: Vec<String> = list_a.iter().map(|x| parse_value(x, precision)).collect();
    let b: Vec<String> = list_b.iter().map(|x| parse_value(x, precision)).collect();
    let c: Vec<String> = list_c.iter().map(|x| parse_value(x, precision)).collect();

    let result: Vec<String> = a
        .into_par_iter()
        .zip(b.into_par_iter())
        .zip(c.into_par_iter())
        .map(|((a_val, b_val), c_val)| concat_string!(a_val, separator, b_val, separator, c_val))
        .collect();

    flame::end("concat8");
    flame::dump_html(&mut File::create("concat8_flamegraph.html").unwrap()).unwrap();

    Python::with_gil(|py| {
        PyList::new(py, &result).to_object(py)
    })
}

fn parse_list(list: &&PyList, precision: Option<usize>) -> Vec<String> {
    
    list.iter().map(|x| parse_value(x, precision)).collect()
}

#[pyfunction]
fn concat_pylists_var(args: Vec<&PyList>, precision: Option<usize>,separator:Option<String>) -> PyObject {
    flame::start("concat9");
    let lists: Vec<Vec<String>> = args.iter().map(|list| parse_list(list, precision)).collect();

    let result: Vec<String> = lists
        .into_par_iter()
        .reduce(|| vec![], |acc, list| {
            acc.into_par_iter()
                .zip(list.into_par_iter())
                .map(|(a_val, b_val)| concat_string!(a_val, b_val))
                .collect()
        });

    flame::end("concat9");
    flame::dump_html(&mut File::create("concat9_flamegraph.html").unwrap()).unwrap();

    Python::with_gil(|py| {
        PyList::new(py, &result).to_object(py)
    })
}

#[pyfunction]
fn pass_through(list_a: &PyList, _list_b: &PyList) -> PyObject {
    Python::with_gil(|py| {
        list_a.to_object(py)
    })
}


#[pyfunction]
fn pass_through_vec(a: Vec<String>, _b: Vec<String>) -> PyResult<Vec<String>> {

    Ok(a)
}

/// A Python module implemented in Rust.
#[pymodule]
fn npconcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat_pylists, m)?)?;
    m.add_function(wrap_pyfunction!(concat_pylists_var, m)?)?;
    m.add_function(wrap_pyfunction!(concat6, m)?)?;
    m.add_function(wrap_pyfunction!(concat7, m)?)?;
    m.add_function(wrap_pyfunction!(concat8, m)?)?;
    m.add_function(wrap_pyfunction!(format, m)?)?;
    m.add_function(wrap_pyfunction!(pass_through, m)?)?;
    m.add_function(wrap_pyfunction!(pass_through_vec, m)?)?;
    Ok(())
}