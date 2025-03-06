cd c_metadata
pdm run src/preprocess.py
cd ../
cp -r c_metadata/output rust_metadata/metadata
cd rust_metadata
pdm run src/rust_metadata.py
