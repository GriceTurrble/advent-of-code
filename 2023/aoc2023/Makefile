.PHONY: newday
newday:
	cookiecutter template

.PHONY: run_sample_%
run_sample_%:
	cargo run -p day$* -- day$*/sample.txt

.PHONY: run_release_%
run_release_%:
	cargo run -rp day$* -- day$*/input.txt

.PHONY: run_%
run_%:
	cargo run -p day$* -- day$*/input.txt
