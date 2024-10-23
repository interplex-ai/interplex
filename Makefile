# Makefile
DOCKER_IMAGE = ghcr.io/interplex-ai/interplex:latest
BINARY_NAME = interplex
TARGET_DIR = /usr/local/bin
SYSTEMD_DIR = /etc/systemd/system
SERVICE_TEMPLATE = service/interplex.service.template
SERVICE_FILE = interplex.service
PACKAGE_NAME = interplex
VERSION = 1.0

all: release docker

release:
	cargo build --release

deb: release
	rm $(PACKAGE_NAME)*.deb > /dev/null 2>&1
	fpm -s dir -t deb -n $(PACKAGE_NAME) -v $(VERSION) --prefix $(TARGET_DIR) target/release/$(BINARY_NAME)=$(TARGET_DIR)/$(BINARY_NAME)
	fpm -s dir -t deb -n $(PACKAGE_NAME)-service -v $(VERSION) --prefix $(SYSTEMD_DIR) --after-install service/post-install.sh $(SERVICE_TEMPLATE)=$(SYSTEMD_DIR)/$(SERVICE_FILE)

clean:
	cargo clean

docker:
	docker build -t $(DOCKER_IMAGE) .

push:
	docker push $(DOCKER_IMAGE)

install: release
	echo "Install into $(TARGET_DIR)/$(BINARY_NAME)..."
	cp target/release/$(BINARY_NAME) $(TARGET_DIR)/$(BINARY_NAME)
	sed "s/__USERNAME__/$(USER)/" $(SERVICE_TEMPLATE) > $(SYSTEMD_DIR)/$(SERVICE_FILE)
	systemctl daemon-reload
	systemctl enable $(SERVICE_FILE)
	systemctl start $(SERVICE_FILE)

uninstall:
	rm $(TARGET_DIR)/$(BINARY_NAME)
	rm $(SYSTEMD_DIR)/$(SERVICE_FILE)
	systemctl daemon-reload
