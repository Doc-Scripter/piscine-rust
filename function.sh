#!bin/bash
# folder name
folder = "my_folder"
cargo --lib new${folder}
cargo new${folder_v2}
cd folder_v2
watch -n 0.01 cargo run .