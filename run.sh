# pdm run script/run.py --project_name c-algorithms --cache_dir ./project_cache/c-algorithms --project_src_folders "include, src" --generation_only
# pdm run script/run.py --project_name avl --cache_dir ./project_cache/avl --project_src_folders "include, src" --generation_only

# pdm run script/run.py --project_name bzp --cache_dir ./project_cache/bzp --project_src_folders "include, src" --generation_only
# pdm run script/run.py --project_name cmptlz --cache_dir ./project_cache/cmptlz --project_src_folders "include, src" --generation_only
# pdm run script/run.py --project_name rapidlz --cache_dir ./project_cache/rapidlz --project_src_folders "include, src" --generation_only
# pdm run script/run.py --project_name md5 --cache_dir ./project_cache/md5 --project_src_folders "include, src" --generation_only
# pdm run script/run.py --project_name sha256 --cache_dir ./project_cache/sha256 --project_src_folders "include, src" --generation_only

pdm run script/run.py --project_name c-algorithms --cache_dir ./project_cache/c-algorithms --project_src_folders "include, src" --output_path ./c-algorithms_report.json
pdm run script/run.py --project_name avl --cache_dir ./project_cache/avl --project_src_folders "include, src" --output_path ./avl_report.json
pdm run script/run.py --project_name bzp --cache_dir ./project_cache/bzp --project_src_folders "include, src" --output_path ./bzp_report.json
pdm run script/run.py --project_name cmptlz --cache_dir ./project_cache/cmptlz --project_src_folders "include, src" --output_path ./cmptlz_report.json
pdm run script/run.py --project_name rapidlz --cache_dir ./project_cache/rapidlz --project_src_folders "include, src" --output_path ./rapidlz_report.json
pdm run script/run.py --project_name md5 --cache_dir ./project_cache/md5 --project_src_folders "include, src" --output_path ./md5_report.json
pdm run script/run.py --project_name sha256 --cache_dir ./project_cache/sha256 --project_src_folders "include, src" --output_path ./sha256_report.json