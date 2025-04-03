function:
	$(eval folder := borrow_me_the_reference)
	cargo new --lib $(folder)
	cargo new $(folder)_v2
	cd $(folder)_v2/src && touch lib.rs
	cd $(folder)_v2 && watch -n 0.01 cargo run

program:
	$(eval folder := string_literals)
	cargo new $(folder)_v2
	cd $(folder)_v2 && watch -n 0.01 cargo run
