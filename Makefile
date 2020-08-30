all: build

build:
	docker build -f Dockerfile.intermediate -t herald-latest .
	docker build -f Dockerfile.package -t dreg.vsix.me:9443/herald:latest .
	docker push dreg.vsix.me:9443/herald:latest

precreate-build:
	docker build -f Dockerfile.precreate-build -t dreg.vsix.me:9443/herald-build:latest .
	docker push dreg.vsix.me:9443/herald-build:latest
