use pyo3::prelude::*;
use aorist_util::init_logging;
use aorist_core::*;
use aorist_attributes::attributes_module;
use aorist_constraints::*;

#[pyfunction]
pub fn dag(universe: AoristRef<Universe>, constraints: Vec<String>, mode: &str) -> PyResult<String> {
    //extendr_engine::start_r();
    //let mut universe = Universe::from(inner);
    //universe.compute_uuids();
    let (output, _requirements) = match mode {
        "airflow" => PythonBasedDriver::<ConstraintBuilder, AirflowFlowBuilder>::new(
            &universe,
            constraints.into_iter().collect(),
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "prefect" => PythonBasedDriver::<ConstraintBuilder, PrefectFlowBuilder>::new(
            &universe,
            constraints.into_iter().collect(),
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "python" => PythonBasedDriver::<ConstraintBuilder, PythonFlowBuilder>::new(
            &universe,
            constraints.into_iter().collect(),
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "jupyter" => PythonBasedDriver::<ConstraintBuilder, JupyterFlowBuilder>::new(
            &universe,
            constraints.into_iter().collect(),
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        /*"r" => RBasedDriver::<ConstraintBuilder, RBasedFlowBuilder>::new(&universe, constraints.into_iter().collect())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
            .run(),*/
        _ => panic!("Unknown mode provided: {}", mode),
    }
    .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
    Ok(output.replace("\\\\", "\\"))
}

#[pymodule]
fn libaorist(py: pyo3::prelude::Python, m: &PyModule) -> PyResult<()> {
    init_logging();
    attributes_module(py, m)?;
    m.add_class::<PyAccessPolicy>()?;
    m.add_class::<PyApproveAccessSelector>()?;
    m.add_class::<PyRegressionAlgorithm>()?;
    m.add_class::<PyRandomForestRegressionAlgorithm>()?;
    m.add_class::<PySVMRegressionAlgorithm>()?;
    m.add_class::<PyAsset>()?;
    m.add_class::<PyStaticDataTable>()?;
    m.add_class::<PySupervisedModel>()?;
    m.add_class::<PyDerivedAsset>()?;
    m.add_class::<PyAttribute>()?;
    m.add_class::<PyPredicate>()?;
    m.add_class::<PyAPILayout>()?;
    m.add_class::<PyAPIOrFileLayout>()?;
    m.add_class::<PyFileBasedStorageLayout>()?;
    m.add_class::<PySingleFileLayout>()?;
    m.add_class::<PyPushshiftSubredditPostsAPILayout>()?;
    m.add_class::<PyTabularLayout>()?;
    m.add_class::<PyDynamicTabularLayout>()?;
    m.add_class::<PyStaticTabularLayout>()?;
    m.add_class::<PyGranularity>()?;
    m.add_class::<PyDailyGranularity>()?;
    m.add_class::<PyDataSet>()?;
    m.add_class::<PyRole>()?;
    m.add_class::<PyGlobalPermissionsAdmin>()?;
    m.add_class::<PyGzipCompression>()?;
    m.add_class::<PyDataCompression>()?;
    m.add_class::<PyZipCompression>()?;
    m.add_class::<PyComplianceConfig>()?;
    m.add_class::<PyCSVHeader>()?;
    m.add_class::<PyFileHeader>()?;
    m.add_class::<PyAlluxioLocation>()?;
    m.add_class::<PyBigQueryLocation>()?;
    m.add_class::<PyGCSLocation>()?;
    m.add_class::<PyGithubLocation>()?;
    m.add_class::<PyHiveLocation>()?;
    m.add_class::<PyLocalFileSystemLocation>()?;
    m.add_class::<PyOnPremiseLocation>()?;
    m.add_class::<PyMinioLocation>()?;
    m.add_class::<PyS3Location>()?;
    m.add_class::<PyPostgresLocation>()?;
    m.add_class::<PyPushshiftAPILocation>()?;
    m.add_class::<PyRemoteLocation>()?;
    m.add_class::<PySQLiteLocation>()?;
    m.add_class::<PyWebLocation>()?;
    m.add_class::<PyModel>()?;
    m.add_class::<PySingleObjectiveRegressor>()?;
    m.add_class::<PyGDBEncoding>()?;
    m.add_class::<PyCSVEncoding>()?;
    m.add_class::<PyTSVEncoding>()?;
    m.add_class::<PyEncoding>()?;
    m.add_class::<PyJSONEncoding>()?;
    m.add_class::<PyNewlineDelimitedJSONEncoding>()?;
    m.add_class::<PyORCEncoding>()?;
    m.add_class::<PyONNXEncoding>()?;
    m.add_class::<PyUndefinedTabularSchema>()?;
    m.add_class::<PyTabularSchema>()?;
    m.add_class::<PyTimeOrderedTabularSchema>()?;
    m.add_class::<PyDataSchema>()?;
    m.add_class::<PyUniverse>()?;
    m.add_class::<PyLocalStorageSetup>()?;
    m.add_class::<PyRemoteStorageSetup>()?;
    m.add_class::<PyReplicationStorageSetup>()?;
    m.add_class::<PyComputedFromLocalData>()?;
    m.add_class::<PyStorageSetup>()?;
    m.add_class::<PyStorage>()?;
    m.add_class::<PyBigQueryStorage>()?;
    m.add_class::<PySQLiteStorage>()?;
    m.add_class::<PyHiveTableStorage>()?;
    m.add_class::<PyRemoteStorage>()?;
    m.add_class::<PyLocalFileStorage>()?;
    m.add_class::<PyPostgresStorage>()?;
    m.add_class::<PyGitStorage>()?;
    m.add_class::<PyRoleBinding>()?;
    m.add_class::<PyDatumTemplate>()?;
    m.add_class::<PyIdentifierTuple>()?;
    m.add_class::<PyRowStruct>()?;
    m.add_class::<PyIntegerMeasure>()?;
    m.add_class::<PyFilter>()?;
    m.add_class::<PyUser>()?;
    m.add_class::<PyUserGroup>()?;
    m.add_class::<PyEndpointConfig>()?;
    m.add_class::<PyAWSConfig>()?;
    m.add_class::<PyGCPConfig>()?;
    m.add_class::<PyGiteaConfig>()?;
    m.add_class::<PyPostgresConfig>()?;
    m.add_class::<PyAlluxioConfig>()?;
    m.add_class::<PyRangerConfig>()?;
    m.add_class::<PyPrestoConfig>()?;
    m.add_class::<PyMinioConfig>()?;
    m.add_class::<PyTrainedFloatMeasure>()?;
    m.add_class::<PyPredictionsFromTrainedFloatMeasure>()?;
    m.add_class::<ConceptAncestry>()?;
    Ok(())
}