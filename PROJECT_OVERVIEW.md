## Visão geral do projeto

- **Tipo de projeto**: workspace Rust com múltiplos crates (`config`, `domain`, `repository`, `services`) e um binário principal (`milions-sys`).
- **Objetivo aparente**: sistema backend que acessa um banco Postgres, organizado por contexto de domínio (clientes, contatos, colaboradores, localizações), ainda sem camada HTTP/API implementada.
- **Tecnologias principais**:
  - **Runtime assíncrono**: `tokio`.
  - **Banco de dados**: `sqlx` com Postgres, migrações SQL manuais em `[migrations](migrations)`.
  - **Configuração**: crate `[config](config/src/lib.rs)` lendo YAML + variáveis de ambiente.
  - **Domínio**: crate `[domain](domain/src/lib.rs)` com tipos, validações e erros.
  - **Acesso a dados**: crate `[repository](repository/src/lib.rs)` organizado em módulos (`clients`, `contacts`, `collaborators`, `locations`).
  - **Serviços**: crate `[services](services/src/lib.rs)` como camada de orquestração acima de `repository`.

## Estrutura de diretórios e crates

- **Raiz do workspace**
  - `[Cargo.toml](Cargo.toml)`: define o workspace e o binário `milions-sys`, com dependências em `services` e `config`.
  - `[src/main.rs](src/main.rs)`: ponto de entrada com `#[tokio::main] async fn main() {}` ainda sem lógica.
  - `[migrations](migrations)`: scripts SQL que criam schemas e tabelas para `clients`, `contacts`, `collaborators`, `locations`.
- **Crate `config`** (`[config/src/lib.rs](config/src/lib.rs)`)
  - Define `Settings`, `DatabaseSettings`, `ApplicationSettings`.
  - Monta `PgConnectOptions` (host, porta, usuário, senha, SSL) para Postgres.
  - Lê arquivos YAML em `files/app_config/base.yaml` e `files/app_config/{environment}.yaml` + variáveis de ambiente `APP_*`.
  - Entrega um ponto único para obter configs da aplicação e do banco.
- **Crate `domain`** (`[domain/src](domain/src)`)
  - Contém tipos de domínio e validações (ex.: telefone, erros específicos de domínio).
  - Depende de `thiserror` e `regex`, sugerindo foco em regras de negócio e validação de dados.
  - Fica desacoplado de infra (sem `sqlx` aqui), que é boa separação de camadas.
- **Crate `repository`** (`[repository/src/lib.rs](repository/src/lib.rs)`)
  - Exports módulos:
    - `[clients](repository/src/clients)`: `client_query.rs`, `client_mutations.rs`, `models/*`.
    - `[contacts](repository/src/contacts)`: `contact_query.rs`, `contact_mutation.rs`, `phone_*`, `models/*`.
    - `[collaborators](repository/src/collaborators)`: `collaborator_query.rs`, `collaborator_mutation.rs`, `models/*`.
    - `[locations](repository/src/locations)`: `location_query.rs`, `location_mutation.rs`, `models/*`.
  - Usa `sqlx` + `tokio` + `uuid` + `chrono` + `serde`.
  - Implementa um **repository pattern** claro:
    - `*_query.rs` para leitura.
    - `*_mutation.rs` para inserção/atualização/remoção.
    - `models/*` para mapear linhas/tabelas em structs Rust.
- **Crate `services`** (`[services/src/lib.rs](services/src/lib.rs)`)
  - Depende de `repository`.
  - Atua como camada de serviço/aplicação, onde se pode orquestrar múltiplos repositórios e encapsular regras de caso de uso.
  - É o lugar natural para concentrar lógica de negócio que é "mais que só CRUD".

## Banco de dados e migrações

- **Tecnologia**: Postgres acessado via `sqlx` com migrations (`migrate`).
- **Config de conexão**: `DatabaseSettings` constrói `PgConnectOptions` a partir de YAML + env (`APP_ENVIRONMENT`, `APP__DATABASE__*`).
- **Migrações** (`[migrations](migrations)`):
  - Arquivos como `20260222180342_create_clients_schema.sql` criam schemas (`clients`, `contacts`, `collaborators`, `locations`) e suas tabelas.
  - Organização por contexto de domínio, com FKs bem definidas e índices para chaves estrangeiras.
- **Alinhamento com código**:
  - Estrutura das tabelas é refletida nos `models` em `[repository/src/*/models](repository/src)`.
  - Essa aderência facilita usar `sqlx::query_as`/`FromRow` com segurança de tipos.

## Pontos fortes da arquitetura atual

- **Separação de camadas**:
  - `config` (infra de configuração), `domain` (regras de negócio), `repository` (persistência), `services` (orquestração) e binário (`main.rs`) estão bem separados.
- **Organização por contexto de domínio**:
  - `clients`, `contacts`, `collaborators`, `locations` possuem diretórios e migrações próprias, o que ajuda na evolução modular.
- **Infra de configuração robusta**:
  - Suporte a múltiplos ambientes via YAML + env, com tipos fortes em Rust.
- **Uso de `sqlx`**:
  - `sqlx` com features de `migrate` + `macros` permite segurança de consultas em tempo de compilação, se for usado com `query!`/`query_as!`.

## Lacunas e pontos de evolução

- **Camada HTTP/API ainda inexistente**:
  - `src/main.rs` não inicializa servidor HTTP, não expõe endpoints nem integra `services` com o mundo externo.
- **Inicialização da aplicação**:
  - Não há, ainda, código que:
    - Carregue `Settings` do crate `config`.
    - Crie pool de conexão com Postgres usando `DatabaseSettings`.
    - Propague esse pool para `repository`/`services`.
- **Integração `domain` ↔ `repository`/`services`**:
  - Falta ver (ou ainda não existe) um mapeamento explícito entre tipos de domínio e modelos de banco.

## Próximos passos sugeridos

- **1. Definir a camada HTTP/API**
  - Escolher um framework (`axum`, `actix-web` ou outro) e planejar estrutura de rotas.
  - Criar um módulo de `startup` que:
    - Leia configurações via `config`.
    - Crie pool `PgPool`/`PgConnection` com `sqlx`.
    - Construa `services` injetando dependências (repository, config).
- **2. Amarrar `main.rs` à infraestrutura**
  - Implementar uma função `run()` ou similar que receba `Settings` e inicialize o servidor HTTP.
  - Usar `tokio` para subir o servidor na porta configurada.
- **3. Consolidar contratos entre camadas**
  - Definir claramente quais tipos pertencem ao `domain` e quais são apenas `models` de banco (`repository`).
  - Evitar vazar structs de `sqlx`/`repository` para fora de `services`.
- **4. Padronizar erros e logging**
  - Definir uma estratégia de erro (por exemplo, um enum de erro de aplicação mapeado para HTTP) na camada de `services`/API.
  - Integrar logging estruturado (caso ainda não exista) para requisições, queries lentas, etc.
