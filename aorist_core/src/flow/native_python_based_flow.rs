use crate::endpoints::EndpointConfig;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    BashPythonTask, ConstantPythonTask, NativePythonTask, PrestoPythonTask, PythonImport,
    PythonPreamble, RPythonTask,
};
use aorist_ast::{Call, Expression, Formatted, SimpleIdentifier, StringLiteral, AST};
use aorist_core::Dialect;
use aorist_primitives::register_task_nodes;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

register_task_nodes! {
    PythonTask,
    PythonImport,
    BashPythonTask,
    RPythonTask,
    NativePythonTask,
    ConstantPythonTask,
    PrestoPythonTask,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct NativePythonBasedFlow {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,

    endpoints: EndpointConfig,
    node: PythonTask,
}
impl ETLFlow for NativePythonBasedFlow {
    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;

    fn get_preamble(&self) -> Vec<PythonPreamble> {
        let preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![PythonPreamble::new(p.clone())],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }
    fn get_imports(&self) -> Vec<PythonImport> {
        self.node.get_imports()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AST> {
        self.node.get_statements()
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        endpoints: EndpointConfig,
    ) -> Self {
        let command = match &dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::R(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    call.as_ref().unwrap().to_string(),
                    false,
                )),
                kwargs.clone(),
            )),
            Some(Dialect::Presto(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    call.as_ref().unwrap().to_string(),
                    true,
                )),
                kwargs.clone(),
            )),
            Some(Dialect::Python(_)) => AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    call.as_ref().unwrap().clone(),
                )),
                args.clone(),
                kwargs.clone(),
            )),
            None => AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string(), false)),
        };
        let node = match &dialect {
            Some(Dialect::Presto(_)) => {
                let presto_endpoints = endpoints.presto.as_ref().unwrap().clone();
                PythonTask::PrestoPythonTask(PrestoPythonTask::new_wrapped(
                    command,
                    task_val.clone(),
                    presto_endpoints,
                ))
            }
            Some(Dialect::Bash(_)) => {
                PythonTask::BashPythonTask(BashPythonTask::new_wrapped(command, task_val.clone()))
            }
            Some(Dialect::R(_)) => {
                PythonTask::RPythonTask(RPythonTask::new_wrapped(command, task_val.clone()))
            }
            Some(Dialect::Python(_)) => {
                PythonTask::NativePythonTask(NativePythonTask::new_wrapped(
                    vec![AST::Expression(Expression::new_wrapped(command))],
                    // TODO: add imports from preamble
                    Vec::new(),
                    task_val.clone(),
                ))
            }
            None => PythonTask::ConstantPythonTask(ConstantPythonTask::new_wrapped(
                command,
                task_val.clone(),
            )),
        };

        Self {
            task_id,
            task_val,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect: dialect.clone(),
            endpoints,
            node,
        }
    }
    fn get_type() -> String {
        "python".to_string()
    }
}
pub struct PythonFlowBuilder {}
impl FlowBuilderBase for PythonFlowBuilder {
    type T = NativePythonBasedFlow;
    fn new() -> Self {
        Self {}
    }
}
impl PythonBasedFlowBuilder for PythonFlowBuilder {
    fn get_flow_imports(&self) -> Vec<PythonImport> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        _py: Python<'a>,
        statements: Vec<(String, Option<String>, Option<String>, Vec<&'a PyAny>)>,
        _ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)> {
        statements
            .into_iter()
            .map(|(name, title, body, code)| {
                (
                    match title {
                        Some(t) => match body {
                            Some(b) => format!(
                                "## {}\n{}",
                                t,
                                b.split("\n")
                                    .map(|x| format!("# {}", x).to_string())
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            )
                            .to_string(),
                            None => format!("## {}", t).to_string(),
                        },
                        None => format!("## {}", name).to_string(),
                    },
                    code,
                )
            })
            .collect()
    }
}