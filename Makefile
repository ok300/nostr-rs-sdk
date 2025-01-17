# Use 'verbose=1' to echo all commands, for example 'make help verbose=1'.
ifdef verbose
  Q :=
else
  Q := @
endif

precommit:
	$(Q)cargo fmt --all && cargo clippy --all

clean:
	$(Q)cargo clean

loc:
	$(Q)echo "--- Counting lines of .rs files (LOC):" && find crates/ -type f -name "*.rs" -exec cat {} \; | wc -l