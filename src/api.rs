use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, ResponseError};

use crate::config::AppConfig;
use crate::job_engine::spawn_job;
use crate::model::{
    clean_label, make_job_id, normalize_seed, validate_steps, ApiError, ErrorEnvelope,
    HealthResponse, JobCreateRequest, JobCreateResponse, JobRecord,
};
use crate::state::AppState;

#[derive(Clone)]
pub struct ApiContext {
    pub config: AppConfig,
    pub state: AppState,
}

impl ApiContext {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            state: AppState::new(),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(ErrorEnvelope {
            code: self.code.to_owned(),
            message: self.message.clone(),
        })
    }
}

pub fn json_payload_error(
    err: actix_web::error::JsonPayloadError,
    _: &HttpRequest,
) -> actix_web::Error {
    let response = HttpResponse::BadRequest().json(ErrorEnvelope {
        code: "invalid_payload".to_owned(),
        message: err.to_string(),
    });
    actix_web::error::InternalError::from_response(err, response).into()
}

#[post("/jobs")]
pub async fn create_job(
    context: web::Data<ApiContext>,
    payload: web::Json<JobCreateRequest>,
) -> Result<impl Responder, ApiError> {
    let label = clean_label(&payload.label);
    let steps = validate_steps(payload.steps, context.config.job_max_steps)?;
    let seed = normalize_seed(payload.seed.as_deref());
    let job_id = make_job_id(context.state.next_sequence());
    context
        .state
        .insert_job(JobRecord::queued(job_id.clone(), label, steps, seed));

    spawn_job(
        context.state.clone(),
        job_id.clone(),
        context.config.job_tick_ms,
    );

    Ok(HttpResponse::Created().json(JobCreateResponse { job_id }))
}

#[get("/jobs")]
pub async fn list_jobs(context: web::Data<ApiContext>) -> impl Responder {
    web::Json(context.state.list_snapshots())
}

#[get("/jobs/{id}")]
pub async fn get_job(
    context: web::Data<ApiContext>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let snapshot = context.state.job_snapshot(&id).ok_or_else(|| {
        ApiError::new(
            StatusCode::NOT_FOUND,
            "job_not_found",
            format!("job `{id}` was not found"),
        )
    })?;
    Ok(web::Json(snapshot))
}

#[get("/health")]
pub async fn health() -> impl Responder {
    web::Json(HealthResponse::ok())
}

pub async fn index(context: web::Data<ApiContext>) -> Result<NamedFile, ApiError> {
    let path = std::path::Path::new(&context.config.frontend_dir).join("index.html");
    NamedFile::open(path).map_err(|err| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "index_unavailable",
            format!("unable to open index.html: {err}"),
        )
    })
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health)
            .service(create_job)
            .service(list_jobs)
            .service(get_job),
    );
}
