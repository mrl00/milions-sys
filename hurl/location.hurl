# Hurl E2E: Location — Happy Path CRUD
# Ensure backend is running on localhost:8000 before running.
# Run with: hurl --test location.hurl

###############################################################
# CREATE
###############################################################

POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "street": "Rua das Flores",
    "number": "42",
    "city": "Brasilia",
    "state": "DF",
    "zipcode": "70040-010",
    "complement": "Bloco A",
    "public_space": "Rua",
    "unit": "Apt 101",
    "neighborhood": "Asa Norte",
    "locality": "Brasilia",
    "region": "Centro-Oeste",
    "ibge": "5300108",
    "gia": null,
    "ddd": "61",
    "siafi": "9701"
}
HTTP/1.1 201
[Captures]
location_id: jsonpath "$.id"
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" isString
jsonpath "$.street" == "Rua das Flores"
jsonpath "$.number" == "42"
jsonpath "$.city" == "Brasilia"
jsonpath "$.state" == "DF"
jsonpath "$.zipcode" == "70040-010"
jsonpath "$.address_complement" == "Bloco A"
jsonpath "$.public_space" == "Rua"
jsonpath "$.unit" == "Apt 101"
jsonpath "$.neighborhood" == "Asa Norte"
jsonpath "$.locality" == "Brasilia"
jsonpath "$.region" == "Centro-Oeste"
jsonpath "$.ibge" == "5300108"
jsonpath "$.ddd" == "61"
jsonpath "$.siafi" == "9701"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString

###############################################################
# READ (post-create)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == {{location_id}}
jsonpath "$.street" == "Rua das Flores"
jsonpath "$.number" == "42"
jsonpath "$.city" == "Brasilia"
jsonpath "$.state" == "DF"
jsonpath "$.zipcode" == "70040-010"
jsonpath "$.address_complement" == "Bloco A"
jsonpath "$.public_space" == "Rua"
jsonpath "$.unit" == "Apt 101"
jsonpath "$.neighborhood" == "Asa Norte"
jsonpath "$.locality" == "Brasilia"
jsonpath "$.region" == "Centro-Oeste"
jsonpath "$.ibge" == "5300108"
jsonpath "$.ddd" == "61"
jsonpath "$.siafi" == "9701"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString

###############################################################
# LIST (recurso recém-criado deve aparecer)
###############################################################

GET http://localhost:8000/api/locations
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$" isCollection
jsonpath "$[*].id" contains {{location_id}}

###############################################################
# UPDATE (partial — apenas campos alterados)
###############################################################

PUT http://localhost:8000/api/locations/{{location_id}}
Content-Type: application/json
{
    "street": "Quadra 204",
    "number": "Lote 5",
    "complement": "Sala 302",
    "neighborhood": "Asa Sul"
}
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == {{location_id}}
jsonpath "$.street" == "Quadra 204"
jsonpath "$.number" == "Lote 5"
jsonpath "$.address_complement" == "Sala 302"
jsonpath "$.neighborhood" == "Asa Sul"
# Campos não enviados devem permanecer inalterados
jsonpath "$.city" == "Brasilia"
jsonpath "$.state" == "DF"
jsonpath "$.ddd" == "61"
jsonpath "$.updated_at" isString

###############################################################
# READ (post-update — confirma persistência)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == {{location_id}}
jsonpath "$.street" == "Quadra 204"
jsonpath "$.number" == "Lote 5"
jsonpath "$.address_complement" == "Sala 302"
jsonpath "$.neighborhood" == "Asa Sul"
jsonpath "$.city" == "Brasilia"
jsonpath "$.state" == "DF"
jsonpath "$.ddd" == "61"

###############################################################
# DELETE
###############################################################

DELETE http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 200

###############################################################
# READ (post-delete — deve retornar 404)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString

###############################################################
# DELETE (idempotência — segundo DELETE no mesmo ID)
###############################################################

DELETE http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString
