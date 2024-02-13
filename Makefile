EXECUTABLE := pricetracker 
DBSETUPEXECUTABLE := dbsetup
DISPLAY_SERVER_EXECUTABLE := displayserver

.PHONY: all compile schedule clean run dbsetup test displayserver

# all: compile schedule_cron schedule_anacron

compile:
	@cargo build --release --bin $(EXECUTABLE)

# schedule: schedule_cron schedule_anacron
# 
# schedule_cron:
# 	@echo "0 * * * * $$(pwd)/target/release/$(EXECUTABLE)" | crontab -
# 
# schedule_anacron:
# 	@echo "@daily	0	$(EXECUTABLE)	$$(pwd)/target/release/$(EXECUTABLE)" >> /etc/anacrontab

clean:
	@cargo clean

run:
	@cargo run --bin $(EXECUTABLE) 

dbsetup:
	@cargo run --bin $(DBSETUPEXECUTABLE)

run_display_server:
	@cargo run --bin $(DISPLAY_SERVER_EXECUTABLE)

test:
	@cargo test
