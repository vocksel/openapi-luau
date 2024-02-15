# OpenAPI for Lune-based Services

[Lune](https://lune-org.github.io/docs) is a standalone Luau runtime that comes equipped with an HTTP server, making it possible to host services written in Luau. This document defines the problem of creating an API with Lune, and options for supporting the [OpenAPI specification](https://swagger.io/specification/) as a way to create type-safe REST APIs in Luau.

# Problem

Lune exposes the [net](https://lune-org.github.io/docs/api-reference/net) API which allows the creation of an HTTP server. However, there is no built in mechanism for managing endpoints, and no packages exist for this purpose. As such, it is very hard to define a versioned REST API with documentation and reusable definitions for Lune services

# Requirements

A solution must be able to…

1. Parse an OpenAPI spec, and…
    1. Generate Luau types for all components, and anything else that would be relevant
    2. Generate HTTP server with stubs for each endpoint
    3. Generate HTTP client to interface with server
    4. Generate OpenAPI file from code?
2. Support versioning of the API, and allow v1 and all future versions to be used in tandem
3. Support the use of all existing OpenAPI tech, like Swagger for building docs sites

# Language of choice

| Option | Pros                                 | Cons |
| ------ | ------------------------------------ | ---- |
| Luau   | - Easier for contributors to dive in |
- Can be required directly in Lune services
- Could use js-to-lua to port existing OpenAPI library | - No AST parser, so harder to construct types and endpoint stubs
- No existing packages for OpenAPI |
| Rust | - full_moon can be used to easily generate Luau types
- Existing packages for OpenAPI parsing
- Consistent with other Luau tools like Wally, StyLua, and Selene, which are written in Rust | - No Luau API
- Docs for full_moon aren’t super easy to parse. Might need extensive examples to use it |

# Luau Types Generation

The idea is to take an OpenAPI spec and generate a `types.luau` file which can then be consumed by the server and clients for type-safe usage of endpoints.

| Option                                                                 | Pros                       | Cons                                                                     |
| ---------------------------------------------------------------------- | -------------------------- | ------------------------------------------------------------------------ |
| Create a CLI to generate Luau types from OpenAPI spec                  | - Can be written with Lune | - Type definitions need to be updated when new API versions are released |
| Generate Luau types on the backend and serve them at the service-level | - Can be written with Lune |
- Type definitions can be served off versioned endpoint
- The server does all the work, so there’s no need for a separate tool |  |

# Lune HTTP Server with OpenAPI Endpoints

| Option                                                                                           | Pros                                                                                                             | Cons                                                                    |
| ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Create a package that can consume an OpenAPI spec and leverage Lune to create HTTP server routes | - Bolster the Luau ecosystem by making it possible to define REST APIs and generate the routes for use with Lune |                                                                         |
| Use an ExpressJS server instead                                                                  | - There’s already a mountain of work for OpenAPI in the JS ecosystem. Would save a lot of time to leverage that  | - Work done for generating Luau types would not be usable by the server |
| Port ExpressJS and related OpenAPI tooling to Luau                                               |                                                                                                                  | - One person doing this might be insane                                 |

References:

- https://github.com/kogosoftwarellc/open-api
- https://github.com/kogosoftwarellc/open-api/tree/main/packages/express-openapi
- https://www.freecodecamp.org/news/how-to-build-explicit-apis-with-openapi/
- https://bump.sh/blog/express-api-openapi
- https://github.com/openapi-generators/openapi-python-client ← does codegen for http client
