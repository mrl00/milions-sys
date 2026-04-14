# Hurl E2E: Location — Happy Path CRUD
# Ensure backend is running on localhost:8000 before running.
# Run with: hurl --test crud.hurl

###############################################################
# CREATE
###############################################################

POST http://localhost:8000/api/locations
Content-Type: application/json
{
    "name": "E2E Test Location",
    "description": "This is a test location"
}
HTTP/1.1 201
[Captures]
location_id: jsonpath "$.id"
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" isString
jsonpath "$.name" == "E2E Test Location"
jsonpath "$.description" == "This is a test location"
jsonpath "$.created_at" isString

###############################################################
# READ (post-create)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == {{location_id}}
jsonpath "$.name" == "E2E Test Location"
jsonpath "$.description" == "This is a test location"
jsonpath "$.created_at" isString

###############################################################
# LIST (recurso recém-criado deve aparecer)
###############################################################

GET http://localhost:8000/api/locations
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$" isCollection
jsonpath "$[*].id" includes {{location_id}}

###############################################################
# UPDATE
###############################################################

PUT http://localhost:8000/api/locations/{{location_id}}
Content-Type: application/json
{
    "name": "Updated Test Location",
    "description": "This location has been updated"
}
HTTP/1.1 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.id" == {{location_id}}
jsonpath "$.name" == "Updated Test Location"
jsonpath "$.description" == "This location has been updated"

###############################################################
# READ (post-update — confirma persistência)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 200
[Asserts]
jsonpath "$.id" == {{location_id}}
jsonpath "$.name" == "Updated Test Location"
jsonpath "$.description" == "This location has been updated"

###############################################################
# DELETE
###############################################################

DELETE http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 204

###############################################################
# READ (post-delete — deve retornar 404)
###############################################################

GET http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 404

###############################################################
# DELETE (idempotência — segundo DELETE no mesmo ID)
###############################################################

DELETE http://localhost:8000/api/locations/{{location_id}}
HTTP/1.1 404
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.error" isString
jsonpath "$.status" == 404
