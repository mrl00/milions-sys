# Hurl E2E: Location — Error Cases
# Cada bloco é autossuficiente: cria e destrói seu próprio fixture quando necessário.
# Ensure backend is running on localhost:8000 before running.
# Run with: hurl --test location_errors.hurl

# Payload base reutilizável como referência (não executável — apenas documentação):
# {
#     "street": "Rua das Flores", "number": "42", "city": "Brasília",
#     "state": "DF", "zipcode": "70040-010", "complement": "Bloco A",
#     "public_space": "Rua", "unit": "Apt 101", "neighborhood": "Asa Norte",
#     "locality": "Brasília", "region": "Centro-Oeste", "ddd": "61"
# }

###############################################################
# INPUT VALIDATION — CREATE: campos obrigatórios ausentes
###############################################################

# street ausente
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# ddd ausente
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# city ausente
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# Payload completamente vazio
POST http://localhost:8000/api/locations
Content-Type: application/json
{}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

###############################################################
# INPUT VALIDATION — CREATE: formatos inválidos
###############################################################

# zipcode como inteiro (deve ser string)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": 70040010,
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# ddd como inteiro (deve ser string)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": 61
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# zipcode com formato inválido (não bate com o padrão)
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "invalid-zip",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# state com mais de 2 caracteres
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasília",
    "state": "DFX",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# street vazia
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "",
    "number": "42",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

###############################################################
# INPUT VALIDATION — UPDATE: formatos inválidos
###############################################################

# — cria fixture para os testes de update
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Fixture Update Validation",
    "number": "1",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "",
    "public_space": "Rua",
    "unit": "S/N",
    "neighborhood": "Asa Norte",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 201
[Captures]
fixture_update_id: jsonpath "$.id"

# zipcode como inteiro no update
PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "zipcode": 99999999
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_body"
jsonpath "$.message" isString

# zipcode com formato inválido no update
PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "zipcode": "invalid-zip"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# state com mais de 2 caracteres no update
PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "state": "DFX"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# street vazia no update
PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "street": ""
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# ddd com mais de 3 caracteres no update
PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "ddd": "0611"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString

# Cleanup
DELETE http://localhost:8000/api/locations/{{fixture_update_id}}
HTTP/1.1 200

###############################################################
# READ — IDs inválidos e inexistentes
###############################################################

# UUID sintaticamente inválido
GET http://localhost:8000/api/locations/not-a-uuid
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_path"
jsonpath "$.message" isString

# UUID válido mas inexistente (nil UUID)
GET http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString

###############################################################
# UPDATE — IDs inválidos e inexistentes
###############################################################

# UUID sintaticamente inválido
PUT http://localhost:8000/api/locations/not-a-uuid
Content-Type: application/json
{
    "street": "Qualquer"
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_path"
jsonpath "$.message" isString

# UUID válido mas inexistente
PUT http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
Content-Type: application/json
{
    "street": "Fantasma",
    "city": "Nenhuma"
}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString

###############################################################
# DELETE — IDs inválidos e inexistentes
###############################################################

# UUID sintaticamente inválido
DELETE http://localhost:8000/api/locations/not-a-uuid
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "invalid_path"
jsonpath "$.message" isString

# UUID válido mas inexistente
DELETE http://localhost:8000/api/locations/00000000-0000-0000-0000-000000000000
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString

###############################################################
# HASH / DUPLICATA (deduplicação por conteúdo)
###############################################################

POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua Única",
    "number": "1",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70000-000",
    "complement": "",
    "public_space": "Rua",
    "unit": "S/N",
    "neighborhood": "Plano Piloto",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 201
[Captures]
fixture_dup_id: jsonpath "$.id"

# Segundo POST com payload idêntico — espera conflito por hash duplicado
POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua Única",
    "number": "1",
    "city": "Brasília",
    "state": "DF",
    "zipcode": "70000-000",
    "complement": "",
    "public_space": "Rua",
    "unit": "S/N",
    "neighborhood": "Plano Piloto",
    "locality": "Brasília",
    "region": "Centro-Oeste",
    "ddd": "61"
}
HTTP/1.1 409
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString

# Cleanup
DELETE http://localhost:8000/api/locations/{{fixture_dup_id}}
HTTP/1.1 200

###############################################################
# AUTENTICAÇÃO (TODO — descomentar quando Keycloak estiver ativo)
###############################################################

# Sem token — 401
# POST http://localhost:8000/api/locations
# Content-Type: application/json
# { "street": "No Auth", ... }
# HTTP/1.1 401
# [Asserts]
# jsonpath "$.error" == "unauthorized"

# Token inválido — 401
# POST http://localhost:8000/api/locations
# Authorization: Bearer invalid.token.here
# Content-Type: application/json
# { "street": "Bad Token", ... }
# HTTP/1.1 401

# Token sem permissão de escrita — 403
# POST http://localhost:8000/api/locations
# Authorization: Bearer {{readonly_token}}
# Content-Type: application/json
# { "street": "Forbidden", ... }
# HTTP/1.1 403
