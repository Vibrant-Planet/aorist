mod ast;
mod program;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyString, PyTuple};
use std::collections::{BTreeSet, HashMap};
pub type PythonStatementInput = (
    Vec<AoristStatement>,
    LinkedHashSet<String>,
    BTreeSet<Import>,
);

pub use ast::{
    AoristStatement, ArgType, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Formatted,
    Import, List, LiteralsMap, ParameterTuple, ParameterTupleDedupKey, Preamble, PythonNone,
    SimpleIdentifier, StringLiteral, Subscript, Tuple,
};
pub use program::PythonProgram;

pub fn format_code(code: String) -> PyResult<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let black: &PyModule = PyModule::import(py, "black").unwrap();
    /*let node = black
    .call1("lib2to3_parse", PyTuple::new(py, &[code]))
    .unwrap()*/
    let mode = black.call0("Mode")?;

    let py_code = PyString::new(py, &code);

    let mut kwargs = HashMap::<&str, &PyAny>::new();
    kwargs.insert("mode", mode);
    black
        .call(
            "format_str",
            PyTuple::new(py, &[py_code]),
            Some(kwargs.into_py_dict(py)),
        )
        .unwrap()
        .extract()
}