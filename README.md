## Start Docker Container
```bash
docker build -t project-name .

docker run -it -v "$(pwd)":/app project-name
```

## Watch Tailwind
```bash
./tailwindcss -i input.css -o ./static/styles/output.css --watch 
```

## Watch Change
```bash
cargo watch -x run
```

## Browser path
http://localhost:8080/