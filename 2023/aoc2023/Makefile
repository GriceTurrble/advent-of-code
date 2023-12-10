.PHONY: newday
newday:
	cookiecutter template

.PHONY: run_release_%
DATA=input
run_release_%:
	cargo run -rp day$* -- day$*/$(DATA).txt

.PHONY: run_%
run_%:
	cargo run -p day$* -- day$*/$(DATA).txt

.PHONY: test test_%
test:
	cargo test
# add `-- --show-output` to the end to show prints from inside methods outside tests
test_%:
	cargo test -p day$*
