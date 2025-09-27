# Rust Practice API

A modern REST API built with Rust using the Axum web framework. This project serves as a learning exercise and foundation for building scalable web APIs in Rust.

## 🚀 Features

- **Modern Web Framework**: Built with [Axum](https://github.com/tokio-rs/axum) - a fast, ergonomic web framework
- **Async/Await**: Full async support with Tokio runtime
- **JSON API**: RESTful endpoints with JSON serialization/deserialization
- **CORS Support**: Cross-origin resource sharing enabled
- **Structured Logging**: Comprehensive logging with tracing
- **Error Handling**: Proper error handling with custom error types
- **Type Safety**: Leverages Rust's type system for compile-time safety
- **UUID Support**: Unique identifier generation for resources

## 📋 API Endpoints

### Health Check
- `GET /health` - Check API health status

### Items Management
- `POST /api/items` - Create a new item
- `GET /api/items` - Get all items
- `GET /api/items/:id` - Get item by ID

## 🛠️ Tech Stack

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Serialization**: [Serde](https://serde.rs/)
- **HTTP**: [Hyper](https://hyper.rs/)
- **Logging**: [Tracing](https://tracing.rs/)
- **UUID**: [uuid](https://docs.rs/uuid/)
- **CORS**: [tower-http](https://docs.rs/tower-http/)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd rust-practice-app
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run the development server**
   ```bash
   cargo run
   ```

   The server will start on `http://localhost:3000`

### Development Commands

```bash
# Run the application
cargo run

# Run in development mode with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Build for production
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📖 API Usage Examples

### Health Check
```bash
curl http://localhost:3000/health
```

Response:
```json
{
  "status": "healthy",
  "message": "Rust Practice API is running!",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Create an Item
```bash
curl -X POST http://localhost:3000/api/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Rust Book",
    "description": "Learn Rust programming",
    "price": 29.99
  }'
```

Response:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Rust Book",
  "description": "Learn Rust programming",
  "price": 29.99,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

### Get All Items
```bash
curl http://localhost:3000/api/items
```

### Get Item by ID
```bash
curl http://localhost:3000/api/items/123e4567-e89b-12d3-a456-426614174000
```

## 🏗️ Project Structure

```
rust-practice-app/
├── src/
│   ├── main.rs          # Application entry point and server setup
│   ├── lib.rs           # Library exports
│   ├── handlers.rs      # HTTP request handlers
│   └── models.rs        # Data models and types
├── Cargo.toml           # Project dependencies and metadata
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## 🔧 Configuration

The application can be configured using environment variables:

- `RUST_LOG`: Set logging level (default: `rust_practice_app=debug,tower_http=debug`)
- `PORT`: Server port (default: `3000`)

Example:
```bash
RUST_LOG=info PORT=8080 cargo run
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

## 📦 Building for Production

1. **Build optimized binary**
   ```bash
   cargo build --release
   ```

2. **Run the production binary**
   ```bash
   ./target/release/rust-practice-app
   ```

## 🚀 Deployment

### Docker (Optional)

Create a `Dockerfile`:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-practice-app /usr/local/bin/
EXPOSE 3000
CMD ["rust-practice-app"]
```

Build and run:
```bash
docker build -t rust-practice-app .
docker run -p 3000:3000 rust-practice-app
```

## 🔮 Future Enhancements

- [ ] Database integration (PostgreSQL/SQLite)
- [ ] Authentication and authorization
- [ ] Input validation middleware
- [ ] Rate limiting
- [ ] API documentation with OpenAPI/Swagger
- [ ] Unit and integration tests
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Metrics and monitoring
- [ ] Caching layer

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serde Documentation](https://serde.rs/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) team for the excellent web framework
- [Tokio](https://tokio.rs/) team for the async runtime
- Rust community for the amazing ecosystem

---

**Happy coding with Rust! 🦀**