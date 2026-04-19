# =============================================================================
# POST /api/contacts — Register Contact
# =============================================================================
POST http://localhost:8000/api/contacts
Content-Type: application/json
{
  "email": "john.doe@example.com"
}

HTTP 201
[Captures]
contact_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.email" == "john.doe@example.com"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/contacts — Email inválido
# =============================================================================
POST http://localhost:8000/api/contacts
Content-Type: application/json
{
  "email": "nao-eh-um-email"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/contacts — Conflict (email já existe)
# =============================================================================
POST http://localhost:8000/api/contacts
Content-Type: application/json
{
  "email": "john.doe@example.com"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# GET /api/contacts — List Contacts
# =============================================================================
GET http://localhost:8000/api/contacts

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].email" isString
jsonpath "$[0].created_at" isString
jsonpath "$[0].updated_at" isString


# =============================================================================
# GET /api/contacts/{uuid} — Get Contact
# =============================================================================
GET http://localhost:8000/api/contacts/{{contact_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{contact_id}}
jsonpath "$.email" == "john.doe@example.com"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# GET /api/contacts/{uuid} — Not Found
# =============================================================================
GET http://localhost:8000/api/contacts/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/contacts/{uuid} — Update Contact Email
# =============================================================================
PUT http://localhost:8000/api/contacts/{{contact_id}}
Content-Type: application/json
{
  "email": "john.updated@example.com"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{contact_id}}
jsonpath "$.email" == "john.updated@example.com"
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# PUT /api/contacts/{uuid} — Email inválido
# =============================================================================
PUT http://localhost:8000/api/contacts/{{contact_id}}
Content-Type: application/json
{
  "email": "nao-eh-um-email"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/contacts/{uuid} — Not Found
# =============================================================================
PUT http://localhost:8000/api/contacts/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "email": "ghost@example.com"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/contacts/{uuid} — Conflict (email já pertence a outro contato)
# =============================================================================

# Cria um segundo contato para provocar conflito
POST http://localhost:8000/api/contacts
Content-Type: application/json
{
  "email": "other@example.com"
}

HTTP 201
[Captures]
other_contact_id: jsonpath "$.id"

# Tenta atualizar com o email do outro contato
PUT http://localhost:8000/api/contacts/{{contact_id}}
Content-Type: application/json
{
  "email": "other@example.com"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# POST /api/contacts/{uuid}/phones — Add Phone
# =============================================================================
POST http://localhost:8000/api/contacts/{{contact_id}}/phones
Content-Type: application/json
{
  "phone": "+5561999990001"
}

HTTP 201
[Captures]
phone_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" isString
jsonpath "$.phone" == "+5561999990001"
jsonpath "$.contact_id" == {{contact_id}}
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# POST /api/contacts/{uuid}/phones — Phone inválido
# =============================================================================
POST http://localhost:8000/api/contacts/{{contact_id}}/phones
Content-Type: application/json
{
  "phone": "numero-invalido"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# POST /api/contacts/{uuid}/phones — Phone duplicado
# =============================================================================
POST http://localhost:8000/api/contacts/{{contact_id}}/phones
Content-Type: application/json
{
  "phone": "+5561999990001"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# POST /api/contacts/{uuid}/phones — Contact Not Found
# =============================================================================
POST http://localhost:8000/api/contacts/00000000-0000-7000-0000-000000000000/phones
Content-Type: application/json
{
  "phone": "+5561999990099"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# GET /api/contacts/{uuid}/phones — List Phones
# =============================================================================
GET http://localhost:8000/api/contacts/{{contact_id}}/phones

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[0].id" isString
jsonpath "$[0].phone" isString
jsonpath "$[0].contact_id" == {{contact_id}}
jsonpath "$[0].created_at" isString
jsonpath "$[0].updated_at" isString


# =============================================================================
# GET /api/contacts/{uuid}/phones — Contact Not Found
# =============================================================================
GET http://localhost:8000/api/contacts/00000000-0000-7000-0000-000000000000/phones

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# Adiciona segundo phone para usar como referência de conflito no PUT
# =============================================================================
POST http://localhost:8000/api/contacts/{{contact_id}}/phones
Content-Type: application/json
{
  "phone": "+5561999990002"
}

HTTP 201
[Captures]
phone_id_2: jsonpath "$.id"


# =============================================================================
# PUT /api/phones/{uuid} — Phone inválido
# =============================================================================
PUT http://localhost:8000/api/phones/{{phone_id}}
Content-Type: application/json
{
  "phone": "numero-invalido"
}

HTTP 400
[Asserts]
jsonpath "$.error" == "validation_error"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/phones/{uuid} — Phone duplicado
# =============================================================================
PUT http://localhost:8000/api/phones/{{phone_id}}
Content-Type: application/json
{
  "phone": "+5561999990002"
}

HTTP 409
[Asserts]
jsonpath "$.error" == "conflict"
jsonpath "$.message" isString


# =============================================================================
# PUT /api/phones/{uuid} — Update Phone
# =============================================================================
PUT http://localhost:8000/api/phones/{{phone_id}}
Content-Type: application/json
{
  "phone": "+5561999990003"
}

HTTP 200
[Asserts]
jsonpath "$.id" == {{phone_id}}
jsonpath "$.phone" == "+5561999990003"
jsonpath "$.contact_id" == {{contact_id}}
jsonpath "$.created_at" isString
jsonpath "$.updated_at" isString


# =============================================================================
# PUT /api/phones/{uuid} — Not Found
# =============================================================================
PUT http://localhost:8000/api/phones/00000000-0000-7000-0000-000000000000
Content-Type: application/json
{
  "phone": "+5561999990099"
}

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString


# =============================================================================
# DELETE /api/phones/{uuid} — Remove Phone
# =============================================================================
DELETE http://localhost:8000/api/phones/{{phone_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{phone_id}}
jsonpath "$.contact_id" == {{contact_id}}


# =============================================================================
# DELETE /api/phones/{uuid} — Not Found
# =============================================================================
DELETE http://localhost:8000/api/phones/00000000-0000-7000-0000-000000000000

HTTP 404
[Asserts]
jsonpath "$.error" == "not_found"
jsonpath "$.message" isString
