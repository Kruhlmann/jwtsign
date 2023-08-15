$(OUTDIR)/doc/.dirstamp: $(RUST_SRC)
	$(CARGO) doc
	touch $@
