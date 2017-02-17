#[macro_use] extern crate cpython;

use cpython::{Python, PyResult, ToPyObject, PyList};


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


py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    try!(m.add(py, "fun1", py_fn!(py, fun1())));
    try!(m.add(py, "fun2", py_fn!(py, fun2(s: String))));
    try!(m.add(py, "fun3", py_fn!(py, fun3(s: i64, b: i64))));
    try!(m.add(py, "fun4", py_fn!(py, fun4(l: PyList))));
    Ok(())
});
