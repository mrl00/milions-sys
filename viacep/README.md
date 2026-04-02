# viacep

A Rust project using hexagonal architecture (ports and adapters).

## Project Structure

```
src/
├── adapters/
│   ├── driven/      # Outgoing adapters (repositories, external services)
│   └── driving/     # Incoming adapters (controllers, handlers)
├── application/     # Application services / use cases
└── domain/
    ├── model.rs     # Domain models
    └── ports.rs     # Port interfaces (traits)
```

## Architecture

### Domain Layer

Contains core business logic with no external dependencies. Defines domain models and port interfaces (traits) that abstract external concerns.

### Application Layer

Orchestrates use cases by coordinating domain objects and ports. Contains application services that implement business workflows.

### Adapters Layer

Implements ports to connect the application to external systems.

- **Driving adapters** (inbound): Receive input and invoke application services (e.g., HTTP handlers, CLI commands)
- **Driven adapters** (outbound): Implement output interfaces (e.g., database repositories, external API clients)

## License

This project is licensed under the MIT License.
