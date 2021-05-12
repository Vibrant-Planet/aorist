use crate::python::{AST, Assignment, StringLiteral};
use crate::r::r_import::RImport;
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    NativeRTask,
    |task: &NativeRTask| task.statements.clone(),
    |task: &NativeRTask| {

        let mut statements: Vec<AST> = Vec::new();

        let mut it = task.statements.iter();

        let mut maybe_statement = it.next();
        let mut task_val_assigned = false;
        while let Some(statement) = maybe_statement {
            maybe_statement = it.next();
            statements.push(match statement {
                AST::Assignment(_) => statement.clone(),
                AST::Expression(expr) => match maybe_statement {
                    Some(_) => statement.clone(),
                    None => {
                        task_val_assigned = true;
                        AST::Assignment(Assignment::new_wrapped(
                            task.task_val.clone(),
                            expr.read().unwrap().inner().clone(),
                        ))
                    }
                },
                _ => panic!(
                    "AST node of type {} found in NativePythonTask body",
                    statement.name()
                ),
            });
        }
        if !task_val_assigned {
            statements.push(AST::Assignment(Assignment::new_wrapped(
                task.task_val.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string(), false)),
            )));
        }
        statements

    },
    |task: &NativeRTask| task.imports.clone(),
    RImport,
    statements: Vec<AST>,
    imports: Vec<RImport>,
    task_val: AST,
);