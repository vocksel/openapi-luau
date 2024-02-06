#!/usr/bin/env just --justfile

set dotenv-load

project_name := "openapi-luau"

source := "src"
rojo_project := "default.project.json"
model_file_name := project_name + ".rbxm"

global_defs_path := tmpdir / "globalTypes.d.lua"
sourcemap_path := tmpdir / "sourcemap.json"

tmpdir := `mktemp -d`

default:
  @just --list

lint:
	selene {{ source }}
	stylua --check {{ source }}

init:
	foreman install
	lune setup

test:
	find src -name "*.spec.lua*" -exec lune run {} \;

analyze:
	rojo sourcemap {{ rojo_project }} -o {{ sourcemap_path }}
	luau-lsp analyze --sourcemap={{ sourcemap_path }} \
		--settings="./.vscode/settings.json" \
		{{ source }}

