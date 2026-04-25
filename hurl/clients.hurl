# =============================================================================
# POST /api/clients — Register Client (success)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "John Doe",
  "document": "12345678909",
  "contact": {
    "email": "john.doe@example.com",
    "phones": [
      { "value": "+5561999990001" },
      { "value": "+5561999990002" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "complement": "Sala 101",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 201
[Captures]
client_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.name" == "John Doe"
jsonpath "$.status" == "active"
jsonpath "$.document" == "12345678909"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/clients — Validation error (invalid document)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "Jane Doe",
  "document": "123",
  "contact": {
    "email": "jane.doe@example.com",
    "phones": [
      { "value": "+5561999990003" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/clients — Validation error (invalid email)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "Jane Doe",
  "document": "12345678909",
  "contact": {
    "email": "invalid-email",
    "phones": [
      { "value": "+5561999990003" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/clients — Validation error (invalid phone)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "Jane Doe",
  "document": "12345678909",
  "contact": {
    "email": "jane.doe@example.com",
    "phones": [
      { "value": "invalid-phone" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/clients — Validation error (invalid CEP)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "Jane Doe",
  "document": "12345678909",
  "contact": {
    "email": "jane.doe@example.com",
    "phones": [
      { "value": "+5561999990003" }
    ]
  },
  "address": {
    "cep": "invalid",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/clients — Validation error (invalid state)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "Jane Doe",
  "document": "12345678909",
  "contact": {
    "email": "jane.doe@example.com",
    "phones": [
      { "value": "+5561999990003" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "invalid"
  }
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/clients — Conflict (document already exists)
# =============================================================================
POST http://localhost:8000/api/clients
Content-Type: application/json
{
  "name": "John Doe Duplicate",
  "document": "12345678909",
  "contact": {
    "email": "john.dup@example.com",
    "phones": [
      { "value": "+5561999990004" }
    ]
  },
  "address": {
    "cep": "01310100",
    "street": "Avenida Paulista",
    "number": "1578",
    "neighborhood": "Bela Vista",
    "city": "São Paulo",
    "state": "SP"
  }
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# GET /api/clients — List Clients
# =============================================================================
GET http://localhost:8000/api/clients

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].name" isString
jsonpath "$[0].status" isString
jsonpath "$[0].document" isString
jsonpath "$[0].created_at" isString
jsonpath "$[0].updated_at" isString


# =============================================================================
# GET /api/clients/{uuid} — Get Client
# =============================================================================
GET http://localhost:8000/api/clients/{{client_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{client_id}}
jsonpath "$.name" == "John Doe"
jsonpath "$.status" == "active"
jsonpath "$.document" == "12345678909"


# =============================================================================
# GET /api/clients/{uuid} — Not Found
# =============================================================================
GET http://localhost:8000/api/clients/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/clients/{uuid} — Update Client
# =============================================================================
PUT http://localhost:8000/api/clients/{{client_id}}
Content-Type: application/json
{
  "name": "John Doe Updated",
  "document": "98765432109"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{client_id}}
jsonpath "$.name" == "John Doe Updated"
jsonpath "$.document" == "98765432109"


# =============================================================================
# PUT /api/clients/{uuid} — Not Found
# =============================================================================
PUT http://localhost:8000/api/clients/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "name": "Ghost"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/clients/{uuid}/status — Activate Client
# =============================================================================
PUT http://localhost:8000/api/clients/{{client_id}}/status
Content-Type: application/json
{
  "status": "inactive"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{client_id}}
jsonpath "$.status" == "inactive"


# =============================================================================
# PUT /api/clients/{uuid}/status — Deactivate Client
# =============================================================================
PUT http://localhost:8000/api/clients/{{client_id}}/status
Content-Type: application/json
{
  "status": "active"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{client_id}}
jsonpath "$.status" == "active"


# =============================================================================
# PUT /api/clients/{uuid}/status — Validation error (invalid status)
# =============================================================================
PUT http://localhost:8000/api/clients/{{client_id}}/status
Content-Type: application/json
{
  "status": "invalid"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# DELETE /api/clients/{uuid} — Delete Client
# =============================================================================
DELETE http://localhost:8000/api/clients/{{client_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{client_id}}


# =============================================================================
# DELETE /api/clients/{uuid} — Not Found
# =============================================================================
DELETE http://localhost:8000/api/clients/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString
