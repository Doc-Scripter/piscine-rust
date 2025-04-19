# Define shared variables at the top level
folder := commits_stats
function:
	cargo new $(folder)
	cd $(folder)/src && touch lib.rs
	cd $(folder) && watch -n 0.01 cargo run

program:
	cargo new $(folder)
	cd $(folder) && watch -n 0.01 cargo run

infinite:
	cd $(folder) && watch -n 0.01 cargo run

push:
	watch -n 0.01 git push

pull:
	watch -n 0.01 git pull
clean:
	cd $(folder) && cargo fmt &&cargo clean && cargo build 

push2:
	git push origin && git push github
	
