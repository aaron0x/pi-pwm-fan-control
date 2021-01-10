all: build

build:
	cargo build --release

install:
	cp ./target/release/fan-control /usr/local/bin
	cp ./fan_control.service /etc/systemd/system

uninstall:
	rm /usr/local/bin/fan-control
	rm /etc/systemd/system/fan_control.service

start:
	systemctl enable fan_control.service
	systemctl daemon-reload
	systemctl start fan_control.service

stop:
	systemctl stop fan_control.service
	systemctl disable fan_control.service
	systemctl daemon-reload

clean:
	cargo clean