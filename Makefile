# PROJECT_PATH := .
EXECUTABLE := pricetracker 

.PHONY: all compile schedule clean

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
