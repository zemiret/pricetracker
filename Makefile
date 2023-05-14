EXECUTABLE := pricetracker 
DBSETUPEXECUTABLE := dbsetup

.PHONY: all compile schedule clean run dbsetup test

all: compile schedule_cron schedule_anacron

compile:
	@cargo build --release --bin $(EXECUTABLE)

schedule: schedule_cron schedule_anacron

schedule_cron:
	@echo "0 * * * * $$(pwd)/target/release/$(EXECUTABLE)" | crontab -

schedule_anacron:
	@echo "@daily	0	$(EXECUTABLE)	$$(pwd)/target/release/$(EXECUTABLE)" >> /etc/anacrontab

clean:
	@cargo clean

run:
	@cargo run --bin $(EXECUTABLE) 

dbsetup:
	@cargo run --bin $(DBSETUPEXECUTABLE)

test:
	@cargo test
