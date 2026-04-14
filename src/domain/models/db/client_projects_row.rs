use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

use sqlx::types::BigDecimal;
use uuid::Uuid;

// =============================================================================
// PROJETO
// Obra contratada por um cliente com localização física própria.
// Centraliza informações financeiras e operacionais da obra.
// =============================================================================
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ProjectRow {
    /// Identificador único do projeto
    pub pk_project: Uuid,
    pub idx_project: i32,
    /// Nome do projeto
    pub tx_name: String,
    /// Descrição detalhada do projeto
    pub tx_description: Option<String>,
    /// Status do projeto: planning | in_progress | paused | completed | cancelled
    pub tx_status: String,
    /// Data de início prevista
    pub dt_start_date: Option<NaiveDate>,
    /// Data de término estimada
    pub dt_estimated_end_date: Option<NaiveDate>,
    /// Data de término real
    pub dt_actual_end_date: Option<NaiveDate>,
    /// Área total prevista em m²
    pub nr_total_area_m2: Option<BigDecimal>,
    /// Custo estimado da obra
    pub nr_estimated_cost: Option<BigDecimal>,
    /// Custo real acumulado (atualizado pela aplicação)
    pub nr_actual_cost: Option<BigDecimal>,
    /// Observações gerais
    pub tx_notes: Option<String>,
    /// Indica se o projeto está ativo
    pub bl_active: bool,
    pub ts_project_created_at: NaiveDateTime,
    pub ts_project_updated_at: NaiveDateTime,
    /// FK para clients.tb_client
    pub fk_client: Uuid,
    /// FK para locations.tb_location
    pub fk_address: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProjectRow {
    pub tx_name: String,
    pub tx_description: Option<String>,
    pub tx_status: ProjectStatus,
    pub dt_start_date: Option<NaiveDate>,
    pub dt_estimated_end_date: Option<NaiveDate>,
    pub nr_total_area_m2: Option<BigDecimal>,
    pub nr_estimated_cost: Option<BigDecimal>,
    pub tx_notes: Option<String>,
    pub fk_client: Uuid,
    pub fk_address: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProjectRow {
    pub tx_name: Option<String>,
    pub tx_description: Option<String>,
    pub tx_status: Option<ProjectStatus>,
    pub dt_start_date: Option<NaiveDate>,
    pub dt_estimated_end_date: Option<NaiveDate>,
    pub dt_actual_end_date: Option<NaiveDate>,
    pub nr_total_area_m2: Option<BigDecimal>,
    pub nr_estimated_cost: Option<BigDecimal>,
    pub nr_actual_cost: Option<BigDecimal>,
    pub tx_notes: Option<String>,
    pub bl_active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "planning"),
            ProjectStatus::InProgress => write!(f, "in_progress"),
            ProjectStatus::Paused => write!(f, "paused"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

// =============================================================================
// ETAPA DO PROJETO
// Fases de execução: Preparação, Selagem, Massa Corrida, Pintura Base, Acabamento.
// =============================================================================
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ProjectStageRow {
    /// Identificador único da etapa
    pub pk_project_stage: Uuid,
    pub idx_project_stage: i32,
    /// FK para clients.tb_project
    pub fk_project: Uuid,
    /// Nome da etapa (ex: Preparação, Pintura Base)
    pub tx_name: String,
    pub tx_description: Option<String>,
    /// Ordem de execução da etapa dentro do projeto
    pub nr_order: i32,
    /// Status da etapa: pending | in_progress | completed | skipped
    pub tx_status: String,
    pub dt_start_date: Option<NaiveDate>,
    pub dt_end_date: Option<NaiveDate>,
    pub ts_created_at_project_stage: NaiveDateTime,
    pub ts_updated_at_project_stage: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateProjectStageRow {
    pub fk_project: Uuid,
    pub tx_name: String,
    pub tx_description: Option<String>,
    pub nr_order: i32,
    pub tx_status: ProjectStageStatus,
    pub dt_start_date: Option<NaiveDate>,
    pub dt_end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateProjectStageRow {
    pub tx_name: Option<String>,
    pub tx_description: Option<String>,
    pub nr_order: Option<i32>,
    pub tx_status: Option<ProjectStageStatus>,
    pub dt_start_date: Option<NaiveDate>,
    pub dt_end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStageStatus {
    Pending,
    InProgress,
    Completed,
    Skipped,
}

impl std::fmt::Display for ProjectStageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStageStatus::Pending => write!(f, "pending"),
            ProjectStageStatus::InProgress => write!(f, "in_progress"),
            ProjectStageStatus::Completed => write!(f, "completed"),
            ProjectStageStatus::Skipped => write!(f, "skipped"),
        }
    }
}

// =============================================================================
// TIPO DE SERVIÇO
// Catálogo de serviços oferecidos (Pintura Interna, Textura, Massa Corrida etc.)
// =============================================================================
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ProjectServiceTypeRow {
    /// Identificador único do tipo de serviço
    pub pk_service_type: Uuid,
    pub idx_service_type: i32,
    /// Nome do serviço (ex: Pintura Interna Lisa)
    pub tx_name: String,
    pub tx_description: Option<String>,
    /// Unidade de medida: m2 | m_linear | unit | hour
    pub tx_unit: String,
    /// Preço unitário padrão (pode ser sobrescrito em tb_project_service)
    pub nr_default_unit_price: Option<BigDecimal>,
    pub bl_active: bool,
    pub ts_created_at_service_type: NaiveDateTime,
    pub ts_updated_at_service_type: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProjectServiceTypeRow {
    pub tx_name: String,
    pub tx_description: Option<String>,
    pub tx_unit: ProjectServiceUnit,
    pub nr_default_unit_price: Option<BigDecimal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProjectServiceTypeRow {
    pub tx_name: Option<String>,
    pub tx_description: Option<String>,
    pub tx_unit: Option<ProjectServiceUnit>,
    pub nr_default_unit_price: Option<BigDecimal>,
    pub bl_active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectServiceUnit {
    M2,
    MLinear,
    Unit,
    Hour,
}

impl std::fmt::Display for ProjectServiceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectServiceUnit::M2 => write!(f, "m2"),
            ProjectServiceUnit::MLinear => write!(f, "m_linear"),
            ProjectServiceUnit::Unit => write!(f, "unit"),
            ProjectServiceUnit::Hour => write!(f, "hour"),
        }
    }
}

// =============================================================================
// SERVIÇO DO PROJETO
// Serviços contratados dentro de um projeto com quantitativo e preço.
// nr_total_price é coluna gerada pelo banco (quantity * unit_price).
// =============================================================================
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ProjectServiceRow {
    /// Identificador único do serviço no projeto
    pub pk_project_service: Uuid,
    pub idx_project_service: i32,
    /// FK para clients.tb_project
    pub fk_project: Uuid,
    /// FK para clients.tb_project_stage (opcional)
    pub fk_project_stage: Option<Uuid>,
    /// FK para clients.tb_service_type
    pub fk_service_type: Uuid,
    pub tx_description: Option<String>,
    pub nr_quantity: BigDecimal,
    pub nr_unit_price: BigDecimal,
    /// Calculado pelo banco: nr_quantity * nr_unit_price
    pub nr_total_price: Option<BigDecimal>,
    /// Status: pending | in_progress | completed
    pub tx_status: String,
    pub ts_created_at_project_service: NaiveDateTime,
    pub ts_updated_at_project_service: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProjectServiceRow {
    pub fk_project: Uuid,
    pub fk_project_stage: Option<Uuid>,
    pub fk_service_type: Uuid,
    pub tx_description: Option<String>,
    pub nr_quantity: BigDecimal,
    pub nr_unit_price: BigDecimal,
    pub tx_status: ProjectServiceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProjectServiceRow {
    pub fk_project_stage: Option<Uuid>,
    pub tx_description: Option<String>,
    pub nr_quantity: Option<BigDecimal>,
    pub nr_unit_price: Option<BigDecimal>,
    pub tx_status: Option<ProjectServiceStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectServiceStatus {
    Pending,
    InProgress,
    Completed,
}

impl std::fmt::Display for ProjectServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectServiceStatus::Pending => write!(f, "pending"),
            ProjectServiceStatus::InProgress => write!(f, "in_progress"),
            ProjectServiceStatus::Completed => write!(f, "completed"),
        }
    }
}

// =============================================================================
// ALOCAÇÃO DIÁRIA DE COLABORADORES
// Registra quais colaboradores trabalharam em qual projeto a cada dia.
// nr_hourly_rate_snapshot congela o valor/hora vigente no momento do registro.
// =============================================================================
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ProjectDailyAllocationRow {
    /// Identificador único da alocação diária
    pub pk_project_daily_allocation: Uuid,
    pub idx_project_daily_allocation: i32,
    /// FK para clients.tb_project
    pub fk_project: Uuid,
    /// FK para collaborators.tb_collaborator
    pub fk_collaborator: Uuid,
    /// Data do trabalho realizado
    pub dt_work_date: NaiveDate,
    /// Horas efetivamente trabalhadas
    pub nr_hours_worked: Option<BigDecimal>,
    /// Valor/hora congelado no momento do registro (preserva histórico financeiro)
    pub nr_hourly_rate_snapshot: Option<BigDecimal>,
    pub tx_notes: Option<String>,
    /// Indica se o colaborador compareceu
    pub bl_present: bool,
    pub ts_allocated_collaborator_created_at: NaiveDateTime,
    pub ts_allocated_collaborator_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProjectDailyAllocationRow {
    pub fk_project: Uuid,
    pub fk_collaborator: Uuid,
    pub dt_work_date: NaiveDate,
    pub nr_hours_worked: Option<BigDecimal>,
    pub nr_hourly_rate_snapshot: Option<BigDecimal>,
    pub tx_notes: Option<String>,
    pub bl_present: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProjectDailyAllocationRow {
    pub nr_hours_worked: Option<BigDecimal>,
    pub nr_hourly_rate_snapshot: Option<BigDecimal>,
    pub tx_notes: Option<String>,
    pub bl_present: Option<bool>,
}
