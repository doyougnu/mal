
EXEC_DIR := target/release

UPPER_STEPS := $(EXEC_DIR)/src/step4_if_fn_do \
               $(EXEC_DIR)/src/step5_tco \
               $(EXEC_DIR)/src/step6_file \
               $(EXEC_DIR)/src/step7_quote \
               $(EXEC_DIR)/src/step8_macros \
               $(EXEC_DIR)/src/step9_try \
               $(EXEC_DIR)/src/stepA_mal
STEP0       := $(EXEC_DIR)/src/step0_repl
STEP1-2     := $(EXEC_DIR)/src/step1_read_print \
               $(EXEC_DIR)/src/step2_eval
STEP3       := $(EXEC_DIR)/src/step3_env
STEPS := $(STEP0) $(STEP1-2) $(STEP3) $(UPPER_STEPS)

all: $(STEPS)

$(STEPS): $(EXEC_DIR)/%: %.rs
	cargo build --release --bin $*

$(STEPS): readline.rs
$(STEP1-2) $(STEP3) $(UPPER_STEPS): types.rs reader.rs printer.rs
$(STEP3) $(UPPER_STEPS): env.rs
$(UPPER_STEPS): core.rs

lint:
	rustfmt *.rs
	cargo clippy

.PHONY: clean lint all

clean:
	rm -fr target/
	rm -f .mal-history *~ Cargo.lock
