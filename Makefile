NUM=$(filter-out $@, $(MAKECMDGOALS))
HEADER="Project Euler Problem $(NUM)"
RUNTIME="Compiler/Interpreter: %s\n\n"
ANSWER="Answer:"

help:
	@echo "Usage: make [lang] [number]"

rust:
	@echo $(HEADER)
	@printf $(RUNTIME) "`rustc --version`"
	@cd rust && cargo clean
	@cd rust && cargo build --release --bin p$(NUM) > /dev/null
	@time -p ./rust/target/release/p$(NUM)

python:
	@echo $(HEADER)
	@printf $(RUNTIME) "`python3 --version`"
	@time -p python3 python/p$(NUM).py


%:
	@: # phony rule to quiet warning about no rule for 'number' argument

.PHONY: python rust
