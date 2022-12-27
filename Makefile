BAKE_OPTIONS=--no-input

help:
	@echo "bake 	generate project using defaults"
	@echo "watch 	generate project using defaults and watch for changes"
	@echo "replay 	replay last cookiecutter run and watch for changes"

bake:
	cookiecutter $(BAKE_OPTIONS) . --overwrite-if-exists

watch: bake
	watchmedo shell-command -p '*.*' -c 'make bake -e BAKE_OPTIONS=$(BAKE_OPTIONS)' -W -R -D \rust_geodistances/

replay: BAKE_OPTIONS=--replay
replay: watch
	;

pip_reinstall:
	python3 -m pip install -e ./

rust_release:
	cargo build --release --target-dir ./bin/rust_geodistances_backend

docs_rebuild_only:
	cd docs; make rebuild

docs_build:
	cd docs; make html -e; make text -e

docs_rebuild: docs_rebuild_only docs_build

full_rebuild: rust_release pip_reinstall docs_rebuild
