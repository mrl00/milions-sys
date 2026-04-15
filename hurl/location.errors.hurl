# Hurl E2E: Location — Error Cases
# Cada bloco é autossuficiente: cria e destrói seu próprio fixture quando necessário.
# Ensure backend is running on localhost:8000 before running.
# Run with: hurl --test errors.hurl

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
jsonpath "$.error" isString
jsonpath "$.status" == 400

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
jsonpath "$.error" isString
jsonpath "$.status" == 400

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
jsonpath "$.status" == 400

# Payload completamente vazio
POST http://localhost:8000/api/locations
Content-Type: application/json
{}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.status" == 400

###############################################################
# INPUT VALIDATION — CREATE: tipos inválidos
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
jsonpath "$.status" == 400

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
jsonpath "$.status" == 400

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
jsonpath "$.error" isString
jsonpath "$.status" == 404

# Tipo inválido em campo do UPDATE (zipcode como inteiro)
# — cria fixture própria para garantir que o recurso existe
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

PUT http://localhost:8000/api/locations/{{fixture_update_id}}
Content-Type: application/json
{
    "zipcode": 99999999
}
HTTP/1.1 400
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.status" == 400

# Cleanup
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
# HASH / DUPLICATA (deduplicação por conteúdo)
# O campo `hash` é gerado via MD5 no banco (GENERATED ALWAYS AS).
# Dois payloads idênticos devem ser rejeitados se unicidade for imposta.
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
# Se a API não impõe unicidade via hash, ajuste para HTTP/1.1 201
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
jsonpath "$.error" isString
jsonpath "$.status" == 409

# Cleanup
DELETE http://localhost:8000/api/locations/{{fixture_dup_id}}
HTTP/1.1 204

###############################################################
# AUTENTICAÇÃO (TODO — descomentar quando Keycloak estiver ativo)
###############################################################

# Sem token — 401
# POST http://localhost:8000/api/locations
# Content-Type: application/json
# { "street": "No Auth", ... }
# HTTP/1.1 401
# [Asserts]
# jsonpath "$.status" == 401

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
