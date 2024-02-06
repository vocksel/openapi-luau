local fs = require("@lune/fs")
local serde = require("@lune/serde")
local openapiTypes = require("./openapi31")
local generateApiTypes = require("./generateApiTypes")

type OpenAPIObject = openapiTypes.OpenAPIObject

local api: OpenAPIObject = serde.decode("yaml", fs.readFile("src/mocks/commented-schema.yml"))
local snapshot = fs.readFile("src/mocks/petstore-types.luau")

local generatedTypes = generateApiTypes(api)

assert(generatedTypes == snapshot, "generated types don't match snapshot")
