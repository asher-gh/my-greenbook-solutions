IN = $(filter-out $@, $(MAKECMDGOALS))

$(IN):
	cargo run --bin $@ 2>/dev/null

.PHONY: $(IN)
