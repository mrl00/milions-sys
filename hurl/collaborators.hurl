# =============================================================================
# POST /api/collaborators — Register Collaborator
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "John Doe",
  "cpf": "52998224725",
  "level": "painter",
  "contact": {
    "email": "john.doe@example.com",
    "phones": [
      { "value": "+5561999990001" }
    ]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 201
[Captures]
collaborator_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.name" == "John Doe"
jsonpath "$.cpf" == "52998224725"
jsonpath "$.level" isString
jsonpath "$.status" == "active"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/collaborators — CPF com formato inválido (garde)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "Bad CPF",
  "cpf": "1234",
  "level": "painter",
  "contact": {
    "email": "bad@example.com",
    "phones": [{ "value": "+5561999990002" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/collaborators — CPF com dígitos verificadores inválidos (value object)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "Bad CPF Digits",
  "cpf": "12345678900",
  "level": "painter",
  "contact": {
    "email": "digits@example.com",
    "phones": [{ "value": "+5561999990003" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 422
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/collaborators — Level inválido (garde)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "Bad Level",
  "cpf": "52998224725",
  "level": "ceo",
  "contact": {
    "email": "level@example.com",
    "phones": [{ "value": "+5561999990004" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/collaborators — Email inválido (garde)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "Bad Email",
  "cpf": "52998224725",
  "level": "painter",
  "contact": {
    "email": "nao-eh-email",
    "phones": [{ "value": "+5561999990005" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/collaborators — Conflict (CPF já existe)
# =============================================================================
POST http://localhost:8000/api/collaborators
Content-Type: application/json
{
  "name": "John Doe Duplicate",
  "cpf": "52998224725",
  "level": "helper",
  "contact": {
    "email": "john.duplicate@example.com",
    "phones": [{ "value": "+5561999990006" }]
  },
  "address": {
    "cep": "70040010",
    "street": "Esplanada dos Ministérios",
    "number": "S/N",
    "complement": null,
    "neighborhood": "Plano Piloto",
    "city": "Brasília",
    "state": "DF"
  }
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# GET /api/collaborators — List Collaborators
# =============================================================================
GET http://localhost:8000/api/collaborators

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].name" isString
jsonpath "$[0].cpf" isString
jsonpath "$[0].level" isString
jsonpath "$[0].status" isString
jsonpath "$[0].created_at" isString
jsonpath "$[0].updated_at" isString


# =============================================================================
# GET /api/collaborators/{uuid} — Get Collaborator
# =============================================================================
GET http://localhost:8000/api/collaborators/{{collaborator_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{collaborator_id}}
jsonpath "$.name" == "John Doe"
jsonpath "$.cpf" == "52998224725"
jsonpath "$.status" == "active"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# GET /api/collaborators/{uuid} — Not Found
# =============================================================================
GET http://localhost:8000/api/collaborators/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid} — Update Collaborator
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}
Content-Type: application/json
{
  "name": "John Updated",
  "cpf": null,
  "level": "supervisor"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{collaborator_id}}
jsonpath "$.name" == "John Updated"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# PUT /api/collaborators/{uuid} — CPF inválido no update (garde)
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}
Content-Type: application/json
{
  "name": null,
  "cpf": "1234",
  "level": null
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid} — Not Found
# =============================================================================
PUT http://localhost:8000/api/collaborators/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "name": "Ghost",
  "cpf": null,
  "level": null
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Deactivate
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}/status
Content-Type: application/json
{
  "status": "inactive"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{collaborator_id}}
jsonpath "$.status" == "inactive"


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Already Inactive
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}/status
Content-Type: application/json
{
  "status": "inactive"
}

HTTP 409
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Activate
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}/status
Content-Type: application/json
{
  "status": "active"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{collaborator_id}}
jsonpath "$.status" == "active"


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Already Active
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}/status
Content-Type: application/json
{
  "status": "active"
}

HTTP 409
[Asserts]
jsonpath "$.error" isString
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Status inválido (garde)
# =============================================================================
PUT http://localhost:8000/api/collaborators/{{collaborator_id}}/status
Content-Type: application/json
{
  "status": "suspended"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/collaborators/{uuid}/status — Not Found
# =============================================================================
PUT http://localhost:8000/api/collaborators/00000000-0000-7000-0000-000000000000/status
Content-Type: application/json
{
  "status": "active"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# DELETE /api/collaborators/{uuid} — Delete Collaborator
# =============================================================================
DELETE http://localhost:8000/api/collaborators/{{collaborator_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{collaborator_id}}


# =============================================================================
# DELETE /api/collaborators/{uuid} — Not Found
# =============================================================================
DELETE http://localhost:8000/api/collaborators/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString
