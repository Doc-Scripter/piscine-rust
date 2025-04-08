# Define shared variables at the top level
folder := panic

function:
	cargo new --lib $(folder)
	cargo new $(folder)_v2
	cd $(folder)_v2/src && touch lib.rs
	cd $(folder)_v2 && watch -n 0.01 cargo run

program:
	cargo new $(folder)_v2
	cd $(folder)_v2 && watch -n 0.01 cargo run

infinite:
	cd $(folder)_v2 && watch -n 0.01 cargo run

push:
	watch -n 0.01 git push

pull:
	watch -n 0.01 git pull
