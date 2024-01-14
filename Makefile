SHELL := /bin/bash
PY_PROJECTS=2020 2021 2022 scratchpad

# -----------------------------------------------------------------------------------------------------------------------------------------
# LOCK
#
.PHONY: $(addprefix lock_,$(PY_PROJECTS)) lock_all lock

$(addprefix lock_,$(PY_PROJECTS)):
	pushd $(@:lock_%=%) && poetry lock && popd

lock_all: $(addprefix lock_,$(PY_PROJECTS))

lock:
	$(MAKE) -j$(words $(PY_PROJECTS)) lock_all


# -----------------------------------------------------------------------------------------------------------------------------------------
# DEPS
#
.PHONY: $(addprefix deps_,$(PY_PROJECTS)) deps_all deps

$(addprefix deps_,$(PY_PROJECTS)):
	pushd $(@:deps_%=%) && poetry install && popd

deps_all: $(addprefix deps_,$(PY_PROJECTS))

deps:
	$(MAKE) -j$(words $(PY_PROJECTS)) deps_all
