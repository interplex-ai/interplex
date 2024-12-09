# Makefile
DOCKER_IMAGE = ghcr.io/interplex-ai/interplex:latest
BINARY_NAME = interplex
TARGET_DIR = /usr/local/bin
SYSTEMD_DIR = /etc/systemd/system
SERVICE_TEMPLATE = service/interplex.service
SERVICE_FILE = interplex.template
PACKAGE_NAME = interplex
VERSION = 0.0.11 # x-release-please-version

# Ensure verbose output
MAKEFLAGS += --debug=v

# Default target
all: release docker

# Build release binary
release:
	cargo build --release

# Create a Debian package
deb: release
	fpm -s dir -t deb \
		-n $(PACKAGE_NAME) \
		-v $(VERSION) \
		--prefix $(TARGET_DIR) \
		target/release/$(BINARY_NAME)=$(TARGET_DIR)/$(BINARY_NAME) \
		$(SERVICE_TEMPLATE)=$(SYSTEMD_DIR)/$(SERVICE_FILE)

# Clean build artifacts
clean:
	cargo clean

# Deploy using Helm
deploy:
	cd chart/interplex && helm install interplex .

# Remove Helm deployment
undeploy:
	helm delete interplex

# Build Docker image
docker:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target=x86_64-unknown-linux-musl
	docker build -t $(DOCKER_IMAGE) .

# Run Docker container
docker-run:
	docker run -e RUST_LOG=debug $(DOCKER_IMAGE)

# Push Docker image
push:
	docker push $(DOCKER_IMAGE)

# Install the binary and service
install:
	echo "Installing $(BINARY_NAME) to $(TARGET_DIR)..."
	cp target/release/$(BINARY_NAME) $(TARGET_DIR)/$(BINARY_NAME)
	cp $(SERVICE_TEMPLATE) $(SYSTEMD_DIR)/$(PACKAGE_NAME).service
	systemctl daemon-reload
	systemctl enable $(PACKAGE_NAME)
	systemctl start $(PACKAGE_NAME)

# Uninstall the binary and service
uninstall:
	echo "Uninstalling $(BINARY_NAME)..."
	rm -f $(TARGET_DIR)/$(BINARY_NAME)
	rm -f $(SYSTEMD_DIR)/$(PACKAGE_NAME).service
	systemctl daemon-reload
	systemctl disable $(PACKAGE_NAME)
	systemctl stop $(PACKAGE_NAME)
