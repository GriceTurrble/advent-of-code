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

TESTARGS=
.PHONY: test test_lib test_%
# add `-- --show-output` to the end to show prints from inside methods outside tests
test:
	cargo test $(TESTARGS)
test_lib:
	cargo test -p lube $(TESTARGS)
test_%:
	cargo test -p day$* $(TESTARGS)
