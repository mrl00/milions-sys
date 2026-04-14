# Hurl E2E: Location — Error Cases
# Cada bloco é autossuficiente: cria e destrói seu próprio fixture quando necessário.
# Ensure backend is running on localhost:8000 before running.
# Run with: hurl --test errors.hurl

###############################################################
# INPUT VALIDATION — CREATE
###############################################################

# Campo obrigatório ausente (name)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "description": "Missing name field"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 400

# Tipo inválido no campo name (int em vez de string)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": 1234,
    "description": "Name should be a string"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 400

# name vazio (string vazia)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "",
    "description": "Empty name should not be allowed"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 400

# name excede limite de caracteres (500 chars)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "description": "Name too long"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.status" == 400

###############################################################
# READ — IDs inválidos e inexistentes
###############################################################

# UUID sintaticamente inválido
GET http://localhost:8000/api/locations/not-a-uuid
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 400

# UUID válido mas inexistente (nil UUID)
GET http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 404

###############################################################
# UPDATE — IDs inválidos, inexistentes e payload inválido
###############################################################

# UUID sintaticamente inválido
PUT http://localhost:8000/api/locations/not-a-uuid
Content-Type: application/json
{
    "name": "Ghost",
    "description": "Invalid UUID format"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.status" == 400

# UUID válido mas inexistente
PUT http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{
    "name": "Ghost",
    "description": "Nonexistent location"
}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 404

# Payload inválido (name ausente) em recurso existente
# — cria fixture própria para não depender do fluxo crud.hurl
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "Fixture For Update Validation",
    "description": "Temp fixture"
}
HTTP/1.1 201
[Captures]
fixture_update_id: jsonpath "$.id"

PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "description": "Missing name field"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 400

# Cleanup do fixture
DELETE http://localhost:8000/api/locations/{{fixture_update_id}}
HTTP/1.1 204

###############################################################
# DELETE — IDs inválidos e inexistentes
###############################################################

# UUID sintaticamente inválido
DELETE http://localhost:8000/api/locations/not-a-uuid
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.status" == 400

# UUID válido mas inexistente
DELETE http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 404

###############################################################
# NOME DUPLICADO (unicidade)
###############################################################

# Cria fixture com nome único
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "Unique Location Name",
    "description": "First instance"
}
HTTP/1.1 201
[Captures]
fixture_dup_id: jsonpath "$.id"

# Tenta criar com o mesmo nome — espera conflito
# Se a API não impõe unicidade, ajuste para HTTP/1.1 201
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "Unique Location Name",
    "description": "Duplicate — should fail"
}
HTTP/1.1 409
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 409

# Cleanup do fixture
DELETE http://localhost:8000/api/locations/{{fixture_dup_id}}
HTTP/1.1 204

###############################################################
# AUTENTICAÇÃO (TODO — implementar quando Keycloak estiver ativo)
###############################################################

# Sem token — deve retornar 401
# POST http://localhost:8000/api/locations
# Content-Type: application/json
# { "name": "No Auth" }
# HTTP/1.1 401
# [Asserts]
# jsonpath "$.status" == 401

# Token inválido — deve retornar 401
# POST http://localhost:8000/api/locations
# Authorization: Bearer invalid.token.here
# Content-Type: application/json
# { "name": "Bad Token" }
# HTTP/1.1 401

# Token válido sem permissão de escrita — deve retornar 403
# POST http://localhost:8000/api/locations
# Authorization: Bearer {{readonly_token}}
# Content-Type: application/json
# { "name": "Forbidden" }
# HTTP/1.1 403
