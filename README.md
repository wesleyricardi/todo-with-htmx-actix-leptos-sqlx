### Start Docker Container
```bash
docker compose up -d
```

### Run Migrate
```bash
sqlx migrate run
```

### Run App
```bash
cargo run
```

### Watch Tailwind
```bash
./tailwindcss -i input.css -o ./static/styles/output.css --watch 
```

### Watch Change
```bash
cargo watch -x run
```

### Config Environment Variables 
file on: docker-compose.yml

### Browser path
http://localhost:8080/