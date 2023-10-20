# Rust Web Framework Benchmark

Contains Benchmarks for three major Rust Web frameworks:

- [Axum](/axum)
- [Actix](/actix)
- [Rocket](/rocket)

There is also a load benchmark [utility](/bench).

All three services in different frameworks are implementing the same three endpoints:

- `/test/simple` - Simple response of formatted parameter
- `/test/timed` - Delay 20ms before responed, simulation of SQL or REST query inside the handler
- `/test/bcrypt` - Calculate bcrypt hash of the param using cost=10, simulation of some heavy bussiness logic

More info and benchmark results: https://eternal-search.com/axum-vs-actix-vs-rocket
