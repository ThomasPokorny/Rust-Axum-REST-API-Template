# Rust Axum Hexagonal REST API Template

This repository serves as a template for building RESTful APIs using Rust, Axum, and the hexagonal architecture pattern.

## Features

- **Axum**: A web framework based on hyper for building asynchronous RESTful applications in Rust.
- **Hexagonal Architecture**: Promotes modularity, separation of concerns, and testability.
- **Diesel ORM**: Provides a high-level query builder and migrations for interacting with the database.
- **Docker**: Dockerfile-compose provided for easy db containerization.

## Requirements

- Rust
- Docker
- Diesel CLI (`cargo install diesel_cli`)

## Getting Started

1. Clone this repository:

    ```bash
    git clone https://github.com/yourusername/axum-hexagonal-template.git
    ```

2. Navigate into the project directory:

    ```bash
    cd axum-hexagonal-template
    ```

3. Update the Docker Compose file (`docker-compose.yml`) as needed to configure the PostgreSQL image.

4. Update the environment variables in the `.env` file to configure connection details and server settings.

5. Start the PostgreSQL container:

    ```bash
    docker-compose up
    ```

6. Manually execute migrations to set up the database schema and populate default values for the "tweet" table:

    ```bash
    diesel migration run
    ```

7. Build the application:

    ```bash
    cargo build
    ```

8. Run the application:

    ```bash
    cargo run
    ```

9. Default access the API at `http://localhost:4000`.
   To 'fetch all tweets' GE:  http://localhost:4000/api/v1/tweets

## Project Structure

tbd.

## Contributing

Contributions are welcome! Feel free to open issues or pull requests for any improvements or features you'd like to see.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
