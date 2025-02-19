use {
    crate::{
        analytics::RPCAnalytics,
        env::Config,
        error::RpcError,
        handlers::identity::IdentityResponse,
        metrics::Metrics,
        project::Registry,
        providers::ProviderRepository,
        storage::KeyValueStorage,
        utils::build::CompileInfo,
    },
    cerberus::project::ProjectData,
    sqlx::PgPool,
    std::sync::Arc,
    tap::TapFallible,
    tracing::info,
};

pub struct AppState {
    pub config: Config,
    pub postgres: PgPool,
    pub providers: ProviderRepository,
    pub metrics: Arc<Metrics>,
    pub registry: Registry,
    pub identity_cache: Option<Arc<dyn KeyValueStorage<IdentityResponse>>>,
    pub analytics: RPCAnalytics,
    pub compile_info: CompileInfo,
    /// Service instance uptime measurement
    pub uptime: std::time::Instant,
}

#[allow(clippy::too_many_arguments)]
pub fn new_state(
    config: Config,
    postgres: PgPool,
    providers: ProviderRepository,
    metrics: Arc<Metrics>,
    registry: Registry,
    identity_cache: Option<Arc<dyn KeyValueStorage<IdentityResponse>>>,
    analytics: RPCAnalytics,
) -> AppState {
    AppState {
        config,
        postgres,
        providers,
        metrics,
        registry,
        identity_cache,
        analytics,
        compile_info: CompileInfo {},
        uptime: std::time::Instant::now(),
    }
}

impl AppState {
    pub async fn update_provider_weights(&self) {
        self.providers.update_weights(&self.metrics).await;
    }

    #[tracing::instrument(skip(self), level = "debug")]
    async fn get_project_data_validated(&self, id: &str) -> Result<ProjectData, RpcError> {
        let project = self
            .registry
            .project_data(id)
            .await
            .tap_err(|_| self.metrics.add_rejected_project())?;

        project.validate_access(id, None).tap_err(|e| {
            self.metrics.add_rejected_project();
            info!("Denied access for project: {id}, with reason: {e}");
        })?;

        Ok(project)
    }

    pub async fn validate_project_access(&self, id: &str) -> Result<(), RpcError> {
        self.get_project_data_validated(id).await.map(drop)
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn validate_project_access_and_quota(&self, id: &str) -> Result<(), RpcError> {
        let project = self.get_project_data_validated(id).await?;

        validate_project_quota(&project).tap_err(|_| {
            self.metrics.add_quota_limited_project();
            info!(
                project_id = id,
                max = project.quota.max,
                current = project.quota.current,
                "Quota limit reached"
            );
        })
    }
}

fn validate_project_quota(project_data: &ProjectData) -> Result<(), RpcError> {
    if project_data.quota.is_valid {
        Ok(())
    } else {
        Err(RpcError::QuotaLimitReached)
    }
}

#[cfg(test)]
mod test {
    use {
        super::{ProjectData, RpcError},
        cerberus::project::Quota,
    };

    #[test]
    fn validate_project_quota() {
        // TODO: Handle this in some stub implementation of "Registry" abstraction.
        let mut project = ProjectData {
            uuid: "".to_owned(),
            creator: "".to_owned(),
            name: "".to_owned(),
            push_url: None,
            keys: vec![],
            is_enabled: true,
            is_verify_enabled: false,
            is_rate_limited: false,
            allowed_origins: vec![],
            verified_domains: vec![],
            quota: Quota {
                current: 0,
                max: 0,
                is_valid: true,
            },
        };

        match super::validate_project_quota(&project) {
            Ok(()) => {}
            res => panic!("Invalid result: {res:?}"),
        }

        project.quota.is_valid = false;
        match super::validate_project_quota(&project) {
            Err(RpcError::QuotaLimitReached) => {}
            res => panic!("Invalid result: {res:?}"),
        }
    }
}
