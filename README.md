# openapi-luau


## Development

Requirements:
- [Visual Studio Code](https://code.visualstudio.com/) and the following extensions:
  - [Selene](https://marketplace.visualstudio.com/items?itemName=Kampfkarren.selene-vscode)
  - [StyLua](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.stylua)
  - [Luau LSP](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.luau-lsp)
- [Foreman](https://github.com/Roblox/foreman/)
- [Just](https://github.com/casey/just)

Install tools and initialize settings:
```sh
just init
```

Start the server:
```sh
just serve
```


Edit [`plugin/src/constants.lua`](https://github.com/vocksel/vscode-theme-importer-lua/blob/main/plugin/src/constants.lua) and set `SERVER_URL` to `http://localhost:8080`.

Build the plugin:
```sh
just build-watch
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
