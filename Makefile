# Define shared variables at the top level
folder := borrow_box

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
clean:
	cd $(folder) && cargo fmt &&cargo clean && cargo build 

push2:
	git push origin && git push github
	
