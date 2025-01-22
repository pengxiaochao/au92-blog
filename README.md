
# au92-blog

A simple and extensible blogging platform built with Rust and Axum.

## Features
- Fast, secure, and minimalistic
- Refreshable post cache
- Structured handlers for posts, categories, tags, RSS, and sitemap
- Simple configuration via environment variables

## Architecture
- **Handlers**: Process requests and render content (e.g., `post_detail`, `render_index`)
- **Services**: Contain business logic for posts, categories, tags, etc.
- **Models**: Represent data structures like `Site` or domain entities
- **Routes**: Compose handlers into reusable routes
- **Middleware**: Central place for logging, security, and other cross-cutting concerns

## Advantages
- Clear separation of concerns
- Easy to extend with additional features
- Asynchronous support using `tokio` and `axum`
- Straightforward configuration for quick deployment

## Usage
1. Install Rust and Cargo.
2. Clone the repository.
3. Run `cargo run` to start the development server on the configured host and port.

## Contributing
1. Fork the project and clone your fork.
2. Create a feature branch.
3. Commit your changes and open a pull request.

## License
This project is available under the MIT License.