#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult, ToPyObject, PyList, PyDict};


fn fun1(_: Python) -> PyResult<String> {
    Ok("Hello!".to_string())
}

fn fun2(_: Python, s: String) -> PyResult<String> {
    Ok(s + "z")
}

fn fun3(_: Python, a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

fn fun4(py: Python, l: PyList) -> PyResult<PyList> {
    Ok(l
        .iter(py).map(|x| x.extract::<i64>(py).unwrap())
        .map(|x| x * 2)
        .map(|x| x.into_py_object(py)).collect::<Vec<_>>().into_py_object(py)
    )
}

fn fun5(_: Python, l: Vec<i64>) -> PyResult<Vec<i64>> {
    Ok(l.iter().map(|x| x * 2).collect::<Vec<_>>())
}

fn fun6(py: Python, d: PyDict) -> PyResult<i64> {
    Ok(
        d.get_item(py, "a").unwrap().extract::<i64>(py).unwrap() *
        d.get_item(py, "b").unwrap().extract::<i64>(py).unwrap()
    )
}

py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    try!(m.add(py, "fun1", py_fn!(py, fun1())));
    try!(m.add(py, "fun2", py_fn!(py, fun2(s: String))));
    try!(m.add(py, "fun3", py_fn!(py, fun3(s: i64, b: i64))));
    try!(m.add(py, "fun4", py_fn!(py, fun4(l: PyList))));
    try!(m.add(py, "fun5", py_fn!(py, fun5(l: Vec<i64>))));
    try!(m.add(py, "fun6", py_fn!(py, fun6(d: PyDict))));
    Ok(())
});
