# QR and Short Link generator frontend (QRL)

## Requirements
- MongoDB (Version 3.1.0 or higher, lower versions may work but are untested)

## How to run?
### Shell
To run QRL on your terminal, ensure Rust is installed and then use the command below:
```Bash
  trunk serve
```

If you don't have Trunk installed, first install it using the command below:
```Bash
  cargo install trunk
```

### Docker
To run QRL in docker, use the Dockerfile included in the repo.
```Bash
  docker build -t .
```
MongoDB is not included in the image above.
To use this with a separate MongoDB image, you can use the following template file.
```YAML
services:
  backend:
    image: qrl-fe-alpine:latest
    ports:
      - "5151:5151"
```
```Bash
  docker compose up -d
```

## How to contribute?
You are free to open issues and create PRs.
You can also contact me directly via my e-mail: [me@mazoochian.com](mailto:me@mazoochian.com).