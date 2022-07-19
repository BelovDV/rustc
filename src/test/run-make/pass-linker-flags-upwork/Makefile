-include ../../run-make-fulldeps/tools.mk

flag_1 = some_flag

all:
	# Build deps
	$(RUSTC) dep_1.rs --crate-type=staticlib
	$(RUSTC) dep_2.rs --crate-type=staticlib
	$(RUSTC) lib.rs -l static:-bundle=dep_1 -llink-arg=${flag_1} -l static:-bundle=dep_2 --crate-type=lib

	# Check sequence of linker args
	$(RUSTC) main.rs --extern lib=$(TMPDIR)/liblib.rlib --crate-type=bin --print link-args | $(CGREP) -e '"-ldep_1" "some_flag" "-ldep_2"'
