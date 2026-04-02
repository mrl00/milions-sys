# Glossary

Domain terms used across milions-backend.

## Entities

- **Client** — Company or individual who contracts construction services
- **Collaborator** — Worker/employee assigned to construction projects
- **Contact** — Communication record (email + phones) linked to a client or collaborator
- **Location** — Physical address with Brazilian postal data (CEP fields)
- **Project** — Construction project owned by a client, with stages, services, and workforce

## Project Lifecycle

- **Planning** — Initial status, project not yet started
- **InProgress** — Active construction work
- **Paused** — Temporarily halted
- **Completed** — Finished successfully
- **Cancelled** — Abandoned before completion

## Collaborator

- **Level** (P0–P3) — Skill/experience tier
- **Status** — Active or Inactive
- **Daily Allocation** — Workforce assignment to a project for a specific date

## Project Components

- **Stage** — Ordered phase within a project (e.g., foundation, structure, finishing)
- **Service Type** — Catalog of service categories (e.g., m2, linear meter, hour, unit)
- **Project Service** — Concrete service instance with quantity, unit price, and status
- **Daily Allocation** — Collaborator assigned to a project on a specific work date

## Brazilian Types

- **CPF** — Cadastro de Pessoa Física (individual tax ID, 11 digits)
- **CNPJ** — Cadastro Nacional de Pessoa Jurídica (company tax ID, 14 digits)
- **CEP** — Código de Endereçamento Postal (postal code, 8 digits)
- **DDD** — Discagem Direta Distância (area code)
- **IBGE** — Instituto Brasileiro de Geografia e Estatística (municipality code)
- **GIA** — Guia de Informação e Apuração (ICMS registration, São Paulo)
- **SIAFI** — Sistema Integrado de Administração Financeira (federal treasury code)

## Architecture Terms

- **Bounded Context** — Self-contained module with its own domain, ports, and adapters
- **Port** — Trait defining a contract (what the domain needs)
- **Adapter** — Implementation of a port (how infrastructure delivers it)
- **Use Case** — Application-level operation defined as a trait with `execute()`
- **Service** — Struct implementing use case traits, wired to repository ports
- **Repository** — Port trait for data access, implemented by postgres adapters
- **Super-trait** — Composite trait grouping common port combinations (e.g., `FindAndCreate`)
- **ACL** — Anti-Corruption Layer adapter for translating external context types
