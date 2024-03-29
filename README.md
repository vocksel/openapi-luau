# openapi-luau

Implementation of the [OpenAPI specification](https://github.com/OAI/OpenAPI-Specification) for [Lune](https://github.com/lune-org/lune) services

## Development

Requirements:
- [Just](https://github.com/casey/just)

Install tools and initialize settings:
```sh
just init
```

## Testing

Test cases will be revamped once [Jest](https://github.com/jsdotlua/jest-lua) becomes accessible. In the meantime, we're writing adhoc test cases that rely on asserts.

Run tests:
```sh
just test
```

Run analysis:
```sh
just analyze
```
