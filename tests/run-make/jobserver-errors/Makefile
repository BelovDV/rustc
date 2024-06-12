include ../../run-make-fulldeps/tools.mk

# only-linux

all:
	$(RUSTC) test.rs
	$(TMPDIR)/test $(RUSTC_ORIGINAL) $(TMPDIR) test.rs

