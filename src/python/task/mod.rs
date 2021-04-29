mod compressed;
mod key;
mod standalone;
mod uncompressible;

pub use compressed::*;
pub use standalone::*;

use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, ETLTask};
use crate::python::{PythonImport, AST};

pub enum PythonBasedTask<T>
where
    T: ETLFlow,
{
    StandalonePythonBasedTask(StandalonePythonBasedTask<T>),
    ForLoopPythonBasedTask(ForLoopPythonBasedTask<T>),
}
impl<T> ETLTask<T> for PythonBasedTask<T>
where
    T: ETLFlow,
{
    type S = StandalonePythonBasedTask<T>;
    type F = ForLoopPythonBasedTask<T>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandalonePythonBasedTask(task)
    }
}

impl<T> PythonBasedTask<T>
where
    T: ETLFlow,
{
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<String>, Vec<PythonImport>) {
        match &self {
            PythonBasedTask::StandalonePythonBasedTask(x) => x.get_statements(endpoints),
            PythonBasedTask::ForLoopPythonBasedTask(x) => x.get_statements(endpoints),
        }
    }
    #[allow(dead_code)]
    fn for_loop_task(task: ForLoopPythonBasedTask<T>) -> Self {
        Self::ForLoopPythonBasedTask(task)
    }
}
