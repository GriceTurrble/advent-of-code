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
