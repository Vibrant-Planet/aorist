# Aorist

Aorist is a code-generation tool for MLOps. Its aim is to generate legible
code for common repetitive tasks in data science, such as data replication,
common transformations, as well as machine learning operations.

## Installation instructions

Aorist currently works on 64-bit Linux and OSX. Note that currently the OSX
implementation uses x64 emulation via Rosetta2. As such running aorist for
the first time will take a few extra seconds.

### Requirements:

- [Anaconda](https://www.anaconda.com/products/individual#linux). If using an
M1 processor you can use the x64 MacOS version. Ignore any warning about the
system not being 64-bit.  
- a working R install. Aorist was tested with R 4.1.0, but most modern R
installations should work.

## Minimal Example

1. Create Anaconda environment
```
conda create -n aorist -c scienz -c conda-forge aorist aorist_recipes scienz
```

2. Activate the environment
```
conda activate aorist
```

3. Try it on a test script:

```python:test.py
from aorist import *
from aorist_recipes import programs
from scienz import (probprog, subreddit_schema)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
result = dag(universe, ["ReplicateToLocal"],
             "python", programs)
universe = Universe(name="local_data", datasets=[subreddits],
                    endpoints=EndpointConfig(), compliance=None)
with open('generated_script.py', 'w') as f:
    f.write(result)
```

4. Run generated script:
```
python generated_script.py
```

The generated script should be something like:
```
Inserted 292 records into probprog
Example record:
id: 7tgerv
author: pinouchon
subreddit: probprog
created_utc: 1517095003
title: Any probabilistic programming people in Paris
selftext: I live in Paris, and I have a feeling that the Probabilistic Programming community here is quite small. I have yet to meet someone already familiar with it.

So if you are in Paris and you are interested in the subject (not necessarily an expert, I'm not an expert myself), please de-lurk :)

I would love to see if I can find *any*one in Paris and possibly start a meetup group.
```

## Machine Learning Example

What if we want to train a Machine Learning model? This is where aorist is quite expressive.
For instance, let's say we want to train an unsupervised Fasttext model and upload the generated
word embeddings to SQLite.

We run the following script to generate the code:
```python:test_ml.py
from aorist import *
from aorist_recipes import programs
from scienz import (
    probprog, subreddit_schema
)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 16",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=16,
        source_schema=subreddit_schema,
        text_attribute_name="selftext",
    )),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/fasttext_embedding',
    )),
    source_assets=list(subreddits.assets.values()),
)
subreddits.add_asset('embedding', Asset(embedding))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["UploadFasttextToSQLite"], 
             "python", programs)
with open('generated_script_ml.py', 'w') as f:
    f.write(result)
```

2. Then we run the generated code:
```
python generated_script_ml.py
```

3. The result should look something like:
```
Inserted 292 records into probprog
Example record:
id: 395o9e
author: pfumbi
subreddit: probprog
created_utc: 1433854717
title: Model-based machine learning (introductory article with a section on probabilistic programming)[2012]
selftext:
Read 0M words
Number of words:  15
Number of labels: 0
Progress: 100.0% words/sec/thread:    5441 lr:  0.000000 avg.loss:  4.123732 ETA:   0h 0m 0s
Inserted 15 records into embedding
Example record:
word_id: 9
word: practice
embedding: [-0.012267977930605412, -0.0007697424734942615, -0.00519704120233655, 0.007255943492054939, -0.004335511475801468, -0.013080609031021595, 0.007123162969946861, -0.0029513954650610685, 0.0031337994150817394, 0.007843499071896076, 0.000649303081445396, 0.0026010186411440372, -0.010062061250209808, 0.010018683038651943, -0.013150793500244617, -0.015687717124819756]
```

## Adding a new dataset

Let's say we don't just want to embed the `probprog` subreddit. Maybe we also want to add the `mlops` subreddit to our training data. To do so we can create a new dataset.

```python:test_ml2.py
from aorist import *
from aorist_recipes import programs
from scienz import build_subreddit_assets, subreddit_schema, subreddit_datum

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = DataSet(
    name="subreddits",
    description="""
    r/probprog and r/mlops
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=build_subreddit_assets(["probprog", "mlops"]),
    access_policies=[],
)
subreddits = subreddits.replicate_to_local(
    Storage(local), "/tmp/subreddits", Encoding(CSVEncoding())
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 16",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=16,
        source_schema=subreddit_schema,
        text_attribute_name="selftext",
    )),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/fasttext_embedding',
    )),
    source_assets=list(subreddits.assets.values()),
)
subreddits.add_asset('embedding', Asset(embedding))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["UploadFasttextToSQLite"], 
             "python", programs)
with open('generated_script_ml2.py', 'w') as f:
    f.write(result)
```

The output now should look like this:
```
Inserted 162 records into mlops
Example record:
id: oilgyt
author: LSTMeow
subreddit: mlops
created_utc: 1626070517
title: Don't be swayed by the clickbait, it's worth reading!
selftext:
Inserted 292 records into probprog
Example record:
id: 3cqt3q
author: pfumbi
subreddit: probprog
created_utc: 1436490165
title: Horizons in Probabilistic Programming and Bayesian Analysis (SciPy 2015 notes)
selftext:
Read 0M words
Number of words:  153
Number of labels: 0
Progress: 100.0% words/sec/thread:   39881 lr:  0.000000 avg.loss:  3.526413 ETA:   0h 0m 0s
Inserted 153 records into embedding
Example record:
word_id: 50
word: The
embedding: [-0.15435373783111572, -0.3686341941356659, -0.34006866812705994, -0.008660915307700634, 0.09429445117712021, 0.08011642098426819, 0.014333870261907578, -0.015342476777732372, 0.21049700677394867, -0.0027332764584571123, -0.10574445128440857, 0.09784656018018723, -0.4542456567287445, 0.14526918530464172, -0.29748550057411194, -0.10125137865543365]
```

# Developer Guide

## Package organization

Aorist has a Rust core and a Python interface. The project relies on the following sub-projects:
- `aorist_util` -- a Rust crate containing small utility functions used across the project.
- `aorist_derive` -- Rust crate exporting `derive` macros (and only those macros) used across the project.
- `aorist_primitives` -- Rust crate exporting "primitive" macros (such as `register_constraint`, `define_attribute`, etc.) used to abstract away boiler-plate code inside the Rust code base.
- `aorist_concept` -- a Rust crate dedicated to the `aorist` macro. This macro "decorates" structs and enums to make them "constrainable" in the aorist sense.
- `aorist_ast` -- a Rust crate implementing a cross-language Abstract Syntax Tree (AST), used for generating code in both Python and R. Aorist AST nodes get compiled into native Python or R AST nodes. More languages can be supported here.
- `aorist_attributes` -- this Rust crate exports a taxonomy of data attributes (e.g. `KeyStringIdentifier`, `POSIXTimestamp`), which can be used to impose data quality and compliance constraints across table schemas.
- `aorist_core` -- This is the core Rust crate for the Aorist project. The main object taxonomy is defined here. New structs and enums can be added here.
- `aorist_constraint` -- This Rust crate lists constraints that can be applied to Aorist universes made up of concepts as listed in `aorist_core`. Multiple `aorist_constraint` crates can be compiled against the `aorist_core` crate.
- `aorist` -- This Rust crate exports a Python library via a PyO3 binding. This directory also contains the conda recipe used for creating the `aorist` conda package (which includes the compiled Rust library, as well as a number of Python helpers).
- `aorist_recipes` -- This Python package contains recipes (using Python, TrinoSQL, R, or Bash) that can be used to satisfy constraints as defined in `aorist_constraint`. Multiple `aorist_recipes` packages can be provided at runtime. 
- `scienz` -- This Python package contains a set of pre-defined datasets which can be used out-of-the box with the `aorist` package.

## How to build

Because Aorist is a mixed Rust / Python project, building involves two stages:
- first a set of Rust libraries is built via `cargo`.
- then, a Python library is built bia `conda`.

### Rust library

#### Pre-requisites
You will need to [install Rust](https://www.rust-lang.org/tools/install) in order to compile Aorist.

#### Building
You can build individual Rust libraries directly by running `cargo build` from within the respective directory listed in the
[Package Organization](https://github.com/scie-nz/aorist#package-organization) section.

To build the entire project run `cargo build` from the root directory.

### Conda library

#### Pre-requisites

1. Install Anaconda.

2. Make sure you use conda-forge, rather than the default conda channel.

```
conda config --add channels conda-forge
conda config --set channel_priority strict
```

#### Building

Build the packages by running:

```
cd aorist && conda build . && cd .. && \
cd aorist_recipes && conda build . && cd .. && \
cd scienz && conda build . && cd ..
``` 

### Adding new datasets

You can add new canonical datasets to the `scienz` package. Once accepted for publication metadata associated with these datasets can be distributed painlessly. To do so, please follow the steps described below: 

1. specify your datasets in a new Python file in the `scienz/scienz` directory. (You can look at other files in that directory for examples)
2. make sure to import the datasets in `scienz/__init__.py`.
3. Run `conda build .` from within the `scienz` subdirectory. The build step will also trigger a test, which ensures that your dataset is correctly specified.
4. If `conda build .` succeeds, submit a Pull Request against scienz/aorist.
5. Once the PR is accepted, the `scienz` package will be rebuilt and your dataset will be accessible via Anaconda. 

## Overview of an Aorist universe

Let's say we are starting a new project which involves analyzing a number of
large graph datasets, such as the ones provided by the
[SNAP](snap.stanford.edu) project.

We will conduct our analysis in a mini data-lake, such as the
[Trino](trino.io) + [MinIO](min.io) solution specified by
[Walden](https://github.com/scie-nz/walden).

We would like to replicate all these graphs into our data lake before we
can start analyzing them. At a very high-level, this is achieved by defining
a "universe", the totality of things we care about in our project. One such
universe is specified below:

```python
from snap import snap_dataset
from aorist import (
    dag,
    Universe,
    ComplianceConfig,
    HiveTableStorage,
    MinioLocation,
    StaticHiveTableLayout,
    ORCEncoding,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS

universe = Universe(
    name="my_cluster",
    datasets=[
        snap_dataset,
    ],
    endpoints=DEFAULT_ENDPOINTS,
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    compliance=ComplianceConfig(
        description="""
        Testing workflow for data replication of SNAP data to
        local cluster. The SNAP dataset collection is provided
        as open data by Stanford University. The collection contains
        various social and technological network graphs, with
        reasonable and systematic efforts having been made to ensure
        the removal of all Personally Identifiable Information.
        """,
        data_about_human_subjects=True,
        contains_personally_identifiable_information=False,
    ),
)
```

The universe definition contains a number of things:
- the datasets we are talking about (more about it in a bit),
- the endpoints we have available (e.g. the fact that a MinIO server
  is available for storage, as opposed to HDFS or S3, etc., and where
  that server is available; what endpoint we should use for Presto /
  Trino, etc.)
- who the users and groups are that will access the dataset,
- some compliance annotations.

Note: Currently users, groups, and compliance annotations are supported as a
proof of concept -- these concepts are not essential to an introduction
so we will skip them for now.

## Generating a DAG

To generate a flow that replicates our data all we have to do is run:

```python
DIALECT = "python"
out = dag(
  universe, [
    "AllAssetsComputed",
  ], DIALECT
)
```
This will generate a set of Python tasks, which will do the following, for
each asset (i.e., each graph) in our dataset:

- download it from its remote location,
- decompress it, if necessary
- remove its header,
- convert the file to a CSV, if necessary
- upload the CSV data to MinIO
- create a Hive table backing the MinIO location
- convert the CSV-based Hive table to an ORC-based Hive table
- drop the temporary CSV-based Hive table

This set of tasks is also known as a Directed Acyclic Graph (DAG).
The same DAG can be generated as a Jupyter notebook, e.g. by setting:
```python
DIALECT = "jupyter"
```
Or we can set `DIALECT` to `"airflow"` for an Airflow DAG.


### Aside: what is actually going on?
What Aorist does is quite complex -- the following is an explanation of the
conceptual details, but you can skip this if you'd want something a bit more
concrete:
- first, you describe the universe. This universe is actually a
  highly-structured hierarchy of concepts, each of which can be
  "constrained".
- A constraint is something that "needs to happen". In this example all
  you declare that needs to happen is the constraint
  `AllAssetsComputed`. This constraint is attached to the Universe,
  which is a singleton object.
- Constraints attach to specific kinds of objects -- some attach to the entire
  Universe, others attach to tables, etc.
- Constraints are considered to be satisfied when their dependent constraints
  are satisfied. When we populate each constraint's own dependent constraints
  we follow a set of complex mapping rules that are nonetheless fairly
  intuitive (but difficult to express without a longer discussion, see the end
  of this document for that)
- Programs ("recipes") are attached to this constraint graph by a Driver. The
  Driver decides which languages are prefered (e.g. maybe the Driver likes
  Bash scrips more than Presto, etc.). The driver will complain if it can't
  provide a solution for a particular constraint.
- Once the recipes are attached, various minutiae are extracted from the
  concept hierarchy -- e.g., which endpoints to hit, actual schemas of input
  datasets, etc.
- Once the various minutiae are filled in, we have a graph of Python code
  snippets. If these snippets are repetitive (e.g. 100 instances of the same
  function call but with different arguments) we compress them into for loops
  over parameter dictionarie.
- We then take the compressed snippet-graph and further optimize it, for
  instance by pushing repeated parameters out of parameter dictionaries and
  into the main body of the for loop.
- We also compute unique, maximally-descriptive names for the tasks, a
  combination of the constraint name and the concept's position in the
  hierarchy. (e.g. `wine_table_has_replicated_schema`). These names are
  shortened as much as possible while still being unique (e.g., we may shorten
  things to `wine_schema`, a less mouthful of a task name).
- The driver then adds scaffolding for native Python, Airflow or Jupyter code
  generation. Other output formats (e.g. Prefect, Dagster, Makefiles, etc.)
  will be supported in the future.
- Finally, the driver converts the generated Python AST to a concrete string,
  which it then formats as a *pretty* (PEP8-compliant) Python program via
  Python [black](https://github.com/psf/black).

## Describing a dataset

Before we can turn our attention to what we would like to achieve with
our data, we need to determine what the data *is*, to begin with. We do
so via a dataset manifest, which is created using Python code.

Here's an example of how we'd create such a manifest for a canonical ML dataset
(the Wine dataset, as per `example/wine.py`).

First, we define our attribute list:
```python
attributes = attr_list([
    attr.Categorical("wine_class_identifier"),
    attr.PositiveFloat("alcohol"),
    attr.PositiveFloat("malic_acid"),
    attr.PositiveFloat("ash"),
    attr.PositiveFloat("alcalinity_of_ash"),
    attr.PositiveFloat("magnesium"),
    attr.PositiveFloat("total_phenols"),
    attr.PositiveFloat("non_flavanoid_phenols"),
    attr.PositiveFloat("proanthocyanins"),
    attr.PositiveFloat("color_intensity"),
    attr.PositiveFloat("hue"),
    attr.PositiveFloat("od_280__od_315_diluted_wines"),
    attr.PositiveFloat("proline"),
])
```

Then, we express the fact that a row corresponds to a struct
with the fields defined in the `attributes` list:
```python
wine_datum = RowStruct(
    name="wine_datum",
    attributes=attributes,
)
```

Then, we declare that our data can be found somewhere on the Web, in
the `remote` storage. Note that we also record the data being CSV-encoded,
and the location corresponding to a single file. This is where we could
note compression algorithms, headers, etc.:
```python
remote = RemoteStorage(
    location=WebLocation(
        address=("https://archive.ics.uci.edu/ml/"
                 "machine-learning-databases/wine/wine.data"),
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(),
)
```

We need this data to live locally, in a Hive table in ORC format, backed
by a MinIO location with the prefix `wine`:
```python
local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
```
Note a few things:
- we don't specify the table name, as this is automatically-generated from the
  asset name (we will define that momentarily)
- we declare, "this thing needs to be stored in MinIO", but do not concern
  ourselves with endpoints at this moment. Aorist will find the right endpoints
  for us and fill in secrets, etc. Or if MinIO is unavailable it will fail.
- this is also where we can indicate whether our table is static (i.e. there is
  no time dimension, or dynamic).

We are now ready to define our asset, called `wine_table`:
```python
wine_table = StaticDataTable(
    name="wine_table",
    schema=default_tabular_schema(wine_datum),
    setup=RemoteImportStorageSetup(
        tmp_dir="/tmp/wine",
        remote=remote,
        local=[local],
    ),
    tag="wine",
)
```
Here's what we do here:
  - we define an asset called `wine_table`. This is also going to be the name
  of any Hive table that will be created to back this asset (or file, or
  directory, etc., depending on the dataset storage).
  - we also define a schema. A schema tells us *exactly* how we can turn a row
  into a template. For instance, we need the exact order of columns in a row
  to know unambiguously how to convert it into a struct.
  - `default_tabular_schema` is a helper function that allows us to derive a
  schema where columnns in the table are in exactly the same order as fields in
  the struct.
  - the `setup` field introduces the notion of a "replicated" remote storage,
    via `RemoteImportStorageSetup`. The idea expressed here is that we should
    make sure the data available at the `remote` location is replicated exactly
    in the `local` locations (either by copying it over, or, if already
    availalbe, by checking that the remote and target data has the same
    checksum, etc.)
  - we also use a `tag` field to help generate legible task names and IDs
    (e.g., in Airflow)

Finally, let's define our dataset:

```python
wine_dataset = DataSet(
    name="wine",
    description="A DataSet about wine",
    sourcePath=__file__,
    datumTemplates=[wine_datum],
    assets={"wine_table": wine_table},
)
```
This dataset can then be imported into the universe discussed previously.

### Aside: The asset / template split

An Aorist dataset is meant to be a collection of two things:
- data *assets* -- concrete information, stored in one or multiple locations,
  remotely, on-premise, or in some sort of hybrid arrangement.
- datum *templates* -- information about what an instance of our data (i.e., a
  *datum*) represents.


For instance, a table is a data asset. It has rows and columns, and those rows
and columns are filled with some values that can be read from some location.

What those rows and columns *mean* depends on the template. Oftentimes rows in
tables translate to structs, for instance in a typical `dim_customers` table.
But if we're talking about graph data, then a row in our table represents a
tuple (more specifically a pair), and not a struct.

Other examples of data assets would be:
- directories with image files,
- concrete machine learning models,
- aggregations,
- scatterplots,

Other examples of data templates could be:
- a tensor data template corresponding to RGB images,
- an ML model template that takes a certain set of features (e.g. number of
  rooms and surface of a house, and produces a prediction, e.g. a valuation),
- a histogram data template, expressing the meaning of margin columns used for
  aggregations, as well as the aggregation function (a count for a histogram)
- a scatterplot template, encoding the meaning of the x and y axis, etc.

This conceptual differentiation allows us to use the same template to refer to
multiple assets. For instance, we may have multiple tables with exactly the
same schema, some being huge tables with real data, and others being
downsampled tables used for development. These tables should be refered to
using the same template.

This is also very useful in terms of tracking data lineage, on two levels:
semantically (how does template Y follow from template X?) and concretely (how
does row A in table T1 follow from row B in table T2?).

## Back to the SNAP dataset

The SNAP dataset we discussed initially is a bit different from the simple Wine
dataset. For one, it contains many assets -- this is a collection of different graphs
used for Machine Learning applications -- each graph is its own asset. But the
meaning of a row remains the same: it's a 2-tuple made up of identifiers. We
record this by defining the template:
```python
edge_tuple = IdentifierTuple(
    name="edge",
    attributes=attr_list([
        attr.NumericIdentifier("from_id"),
        attr.NumericIdentifier("to_id"),
    ]),
)
```

Then we define an asset for each of 12 datasets. Note that the names come from
the URL patterns corresponding to each dataset. We need to replace dashes with
underscores when creating asset names however (Hive tables don't like dashes
in their names):
```python
names = [
    "ca-AstroPh", "ca-CondMat", "ca-GrQc", "ca-HepPh",
    "ca-HepTh", "web-BerkStan", "web-Google", "web-NotreDame",
    "web-Stanford", "amazon0302", "amazon0312", "amazon0505",
]
tables = {}
for name in names:

    name_underscore = name.replace("-", "_").lower()
    remote = RemoteStorage(
        location=WebLocation(
            address="https://snap.stanford.edu/data/%s.txt.gz" % name,
        ),
        layout=SingleFileLayout(),
        encoding=TSVEncoding(
            compression=GzipCompression(),
            header=UpperSnakeCaseCSVHeader(num_lines=4),
        ),
    )
    local = HiveTableStorage(
        location=MinioLocation(name=name_underscore),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    )
    table = StaticDataTable(
        name=name_underscore,
        schema=default_tabular_schema(edge_tuple),
        setup=RemoteImportStorageSetup(
            tmp_dir="/tmp/%s" % name_underscore,
            remote=remote,
            local=[local],
        ),
        tag=name_underscore,
    )
    tables[name] = table

snap_dataset = DataSet(
    name="snap",
    description="The Snap DataSet",
    sourcePath=__file__,
    datumTemplates=[edge_tuple],
    assets=tables,
    tag="snap",
)
```

## What if we want to do Machine Learning?

As a proof-of-concept, ML models are not substantively different from
tabular-based data assets. Here's an example for how we can declare the
existence of an SVM regression model trained on the wine table:

```python
# We will train a classifier and store it in a local file.
classifier_storage = LocalFileStorage(
    location=MinioLocation(name="wine"),
    layout=SingleFileLayout(),
    encoding=ONNXEncoding(),
)
# We will use these as the features in our classifier.
features = attributes[2:10]
# This is the "recipe" for our classifier.
classifier_template = TrainedFloatMeasure(
    name="predicted_alcohol",
    comment="""
    Predicted alcohol content, based on the following inputs:
    %s
    """ % [x.name for x in features],
    features=features,
    objective=attributes[1],
    source_asset_name="wine_table",
)
# We now augment the dataset with this recipe.
wine_dataset.add_template(classifier_template)
# The classifier is computed from local data
# (note the source_asset_names dictionary)
classifier_setup = ComputedFromLocalData(
    source_asset_names={"training_dataset": "wine_table"},
    target=classifier_storage,
    tmp_dir="/tmp/wine_classifier",
)
# We finally define our regression_model as a concrete
# data asset, following a recipe defined by the template,
# while also connected to concrete storage, as defined
# by classifier_setup
regression_model = SupervisedModel(
    name="wine_alcohol_predictor",
    tag="predictor",
    setup=classifier_setup,
    schema=classifier_template.get_model_storage_tabular_schema(),
    algorithm=SVMRegressionAlgorithm(),
)
wine_dataset.add_asset(regression_model)
```

Note the use of imperative directives such as `wine_dataset.add_asset`. This is
a small compromise on our mostly-declarative syntax, but it maps well on the
following thought pattern common to ML models:
- we have some "primary sources", datasets external to the project,
- we then derive other data assets by building, iteratively on the primary
  sources.

The common development cycle, therefore, is one where, after the original data
sources are imported, we add new templates and assets to our dataset,
fine-tuning Python code by first running it in Jupyter, then in Native python,
then as an Airflow task, etc.

Also note that while currently Aorist only supports generating single files as
DAGs, in the future we expect it will support multiple file generation for
complex projects.

## SQL and derived assets

Especially when datasets are in tabular form, it makes sense to think of data
transformations in terms of standard SQL operations -- selections, projections,
groups, explodes, and joins. These transformations can be supported via a
`derive_asset` directive used in the process of Universe creation. For
instance, if we are interested in training a model for high-ABV wines only, we
can write:

```python
universe.derive_asset(
    """
    SELECT *
    FROM wine.wine_table
    WHERE wine.wine_table.alcohol > 14.0
    """,
    name="high_abv_wines",
    storage=HiveTableStorage(
        location=MinioLocation(name="high_abv_wines"),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    ),
    tmp_dir="/tmp/high_abv_wines",
)
```

Behind the scenes, this directive does two things:
- if necessary, creates a new template expressing the operation of filtering a
  table on the alcohol attribute.
- it creates a new `StaticDataTable` asset living in the indicated storage.
  This table will only be computed *after* its source tables (the ones in the
  `FROM` clause) are ready.


## How to build

To build cargo library (need Rust installed):

```
cargo build
```

To try out Python code against .so library:

```
./run.sh
```

To rebuild pip wheel (requires [maturin](https://github.com/PyO3/maturin)):
```
maturin build
```

# Internals

Aorist's "secret sauce" is a Rust core. Even though we usually interact with
Aorist via Python, deep down we rely on Rust's efficiency and type-safety to
help us deal with the impressive complexity of data tasks. The following notes
deal with some of the core concepts in Aorist. They are still Work-in-Progress.

## Concepts
Aorist uses two types of concepts:
- abstract concepts (e.g. a "location")
- concrete ones (e.g. "a Google Cloud Storage location", or `GCSLocation`).

The relationship between abstract concepts represents the core semantic model offered by Aorist. This is not expected to change on a regular basis. Ideally this would not change at all.

Concrete concepts "instantiate" abstract ones, much like classes can instantiate traits or interfaces in OOP languages (in fact, this is how concrete concepts are implemented in Aorist).

Abstract concepts have the following hierarchy:

- `Universe`: the core Aorist abstraction, one per project
  - `DataSet`:  a grouping of instantiated objects which have inter-related schemas
  - `User`:  someone accessing the objects
  - `Group`:  a group of users
  - `Roles`:  a way in which a user may access data.
  - `RoleBindings`:  a connection between users and roles

Here is the current hierarchy of Aorist concepts:

![Hierarchy of Aorist Concepts](./docs/aorist_constrainables.svg)

## Constraints

A constraint is a fact that can be verified about a concept.
A constraint may have dependent constraints. For instance, we may have
the constraint "is consistent" on `Universe`, that further breaks down into:
- "datasets are replicated",
- "users are instantiated",
- "role bindings are created",
- "data access policies are enforced".

Dependent constraints simply tell us what needs to hold, in order for a constraint
to be true. Dependent constraints may be defined on the same concept, on
dependent concepts, or on higher-order concepts, but they may not create a
cycle. So we cannot say that constraint A depends on B, B depends on C, and C
depends on A.

This is quite dry stuff. Here is a diagram of an example set of constraints to
help better visualize what's going on:

![](./docs/aorist_constraint_dag.svg)

![Hierarchy of Aorist Concepts and Constraints](./docs/aorist_constrainables_with_constraints.svg)


When dependent constraints are defined on lower-order concepts, we will consider
the dependency to be satisfied when *ALL* constraints of the dependent kind
associated with the lower-order concepts directly inheriting from the
constrained concept have been satisfied.

For instance we may say that a constraint placed on the Universe (our
abstraction for a Data Warehouse or Data Lake), of the kind: "no columns
contain PII" is to be satisfied when all columns in *ALL* the tables are
confirmed to not contain any PII.

When dependent constraints are defined on higher-order concepts, we will
consider the dependency to be satisfied when the dependent constraint placed on
the exact higher-order ancestor has been satisfied.

So for instance, a model trained on data from the data warehouse may be
publishable on the web if we can confirm that no data in the warehouse
whatsoever contains any PII. This is a very strict guarantee, but it is
logically correct -- if there is no PII in the warehouse, there can be no PII
in the model. This is why we could have a constraint at the model-level that
depends on the Universe-level "no PII" constraint.

## Constraint DAG generation

Both constraints and concept operate at a very abstract level. They are basic
semantic building blocks of how we understand the things we care about in our
data streams. But our YAML file will define ``instances'' of concepts, i.e.,
Aorist **objects**. `StaticDataTable` is a concept, but we may have 200 static
data tables, on which we would like to impose the same constraints. For
instance, we would like all these tables to be audited, etc.[1]

Looking back at the concept hierarchy mentioned above, we turn the constraint
DAG into the prototype of an ETL pipeline by "walking" both the concept
(black) and constraint (red) part of the DAG.

Here's what the Constraint DAG looks like

![Constraint DAG](./docs/aorist_dag.svg)

[1] (NOTE: in the future we will support filters on constraints, but for now
assume that all constraints must hold for all instances).

Some things to note about this DAG:
- it includes some superfluous dependencies, such as the one between
`DownloadDataFromRemote` and `ReplicatedData`
- some constraints are purely "cosmetic" -- `DataFromRemoteDownloaded` is
  really just a wrapper around `DownloadDataFromRemote` that "elevates" it to
  the root level, so that `UploadDataToLocal` can depend on it.

## Programs

Constraints are satisfiable in two stages:
1. First, any dependent constraints must be satisfied.
2. Then, a program associated with the constraint must run successfully.

The program is where the actual data manipulation happens. Examples of programs
are: "move this data from A to B", or "train this model", or "anonymize this
data," etc. The programs are written as templates, with full access to the
instantiated object hierarchy.

A program is written in a "dialect" that encompases what is considered to be
valid code. For instance, "Python3 with numpy and PyTorch" would be a dialect.
For Python dialects, we may attach a conda `requirements.txt` file, or a Docker
image to the dialect, etc. For R dialects we may attach a list of libraries and
an R version, or a Docker image.

## Drivers

Note that multiple programs may exist that could technically satisfy a
constraint. A **driver** decides which program to apply (given a preference
ordering) and is responsible for instantiating it into valid code that will run
in the specific deployment. A driver could, for instance, be responsible for
translating the constraint graph into valid Airflow code that will run in a
particular data deployment, etc.

# Testing
```
cargo test --no-default-features
```

Note: make sure that libpython is in your `LD_LIBRARY_PATH`. E.g.:


```
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/bogdan/anaconda3/lib/
```

# Developing a new recipe

## Testing the recipe:

1. Compile using cargo and create the required symlinks:

```
python build_for_testing.py
```

2. To check everything still compiles, run:

```
cd tests
PYTHONPATH=$PYTHONPATH:../aorist/:../aorist_recipes/:../scienz/ python test.py
```

Note: if you get a "different mach-o architecture" issue, it's because of a conflict
between python versions. Make sure you compile and run aorist against the same Python!
