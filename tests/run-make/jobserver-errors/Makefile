include ../../run-make-fulldeps/tools.mk

# only-linux

all:
	$(RUSTC) test.rs
	touch $(TMPDIR)/empty
	echo a > $(TMPDIR)/empty
	$(TMPDIR)/test $(RUSTC_ORIGINAL) $(TMPDIR) test.rs

