download_location = download_blob_to_file(
    'gcp-public-data-sentinel2',
    'index.csv.gz-backup',
    '/tmp/sentinel2',
    'sentinel-2-metadata-table')
flow.add_node(download_location)
download_remote = ConstantTask('DownloadDataFromRemote')
flow.add_node(download_remote)
flow.add_edge(download_location, download_remote)
decompress = ShellTask(command='gunzip {tmp_dir}/{file_name}'.format(
    tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table'))
flow.add_node(decompress)
flow.add_edge(download_location, decompress)

remove_header = ShellTask(command="""
tail -n +{n} {tmp_dir}/{file_name} > {tmp_dir}/{file_name}.no_header &&
rm {tmp_dir}/{file_name}
""".format(n='2', tmp_dir='/tmp/sentinel2', file_name='sentinel-2-metadata-table'))
flow.add_node(remove_header)
flow.add_edge(decompress, remove_header)

convert_orc = ShellTask(
    command="""
csv-import {tmp_dir}/{table_name}.no_header {tmp_dir}/{table_name}.no_header
""".format(
        schema=(
            'granule_id:STRING,product_id:STRING'
            ',datatake_identifier:STRING,mgrs_tile:STRING'
            ',sensing_time:BIGINT,total_size:BIGINT,cloud_cover:STRING'
            ',geometric_quality_flag:STRING,generation_time:BIGINT'
            ',north_lat:FLOAT,south_lat:FLOAT,west_lon:FLOAT'),
        tmp_dir='/tmp/sentinel2',
        table_name='sentinel-2-metadata-table'))
flow.add_node(convert_orc)
for dep in [download_remote, remove_header]:
    flow.add_edge(dep, convert_orc)

replicated_data = ConstantTask('ReplicatedData')
flow.add_node(replicated_data)
for dep in [download_location, download_remote, remove_header, convert_orc]:
    flow.add_edge(dep, replicated_data)
hive_created = ShellTask(command="""
presto -e 'CREATE TABLE IF NOT EXISTS sentinel-2-metadata-table (
    west_lon REAL COMMENT \'Western longitude of the tile`s bounding box.\',
    sensing_time BIGINT,
    base_url VARCHAR,
    datatake_identifier VARCHAR,
    north_lat REAL COMMENT \'Northern latitude of the tile`s bounding box.\',
    east_lon REAL COMMENT \'Eastern longitude of the tile`s bounding box.\',
    product_id VARCHAR,
    cloud_cover VARCHAR,
    total_size BIGINT,
    mgrs_tile VARCHAR,
    granule_id VARCHAR,
    geometric_quality_flag VARCHAR,
    generation_time BIGINT,
    south_lat REAL COMMENT \'Southern latitude of the tile`s bounding box.\'
) WITH (format=\'ORC\');
'""")
flow.add_node(hive_created)
replicated_schema = ConstantTask('ReplicatedSchema')
flow.add_node(replicated_schema)
flow.add_edge(hive_created, replicated_schema)


is_consistent = ConstantTask('IsConsistent')
flow.add_node(is_consistent)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(dep, is_consistent)

is_audited = ConstantTask('IsAudited')
flow.add_node(is_audited)
for dep in [replicated_data, replicated_schema]:
    flow.add_edge(dep, is_audited)
