pub mod ai_tagger;
pub mod analytics_rollup;
pub mod media_processor;
pub mod search_indexer;

use crate::config::AppState;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

pub async fn start_background_jobs(state: AppState) {
    let scheduler = JobScheduler::new().await.expect("Failed to create scheduler");

    // Nightly search index rebuild at 02:00
    let s1 = state.clone();
    scheduler.add(
        Job::new_async("0 0 2 * * *", move |_, _| {
            let s = s1.clone();
            Box::pin(async move {
                info!("Running search index rebuild...");
                if let Err(e) = search_indexer::rebuild_all(&s).await {
                    tracing::error!("Search index rebuild failed: {:?}", e);
                }
            })
        }).expect("Job create failed")
    ).await.expect("Job add failed");

    // Analytics rollup every hour at :05
    let s2 = state.clone();
    scheduler.add(
        Job::new_async("0 5 * * * *", move |_, _| {
            let s = s2.clone();
            Box::pin(async move {
                info!("Running analytics rollup...");
                if let Err(e) = analytics_rollup::run(&s).await {
                    tracing::error!("Analytics rollup failed: {:?}", e);
                }
            })
        }).expect("Job create failed")
    ).await.expect("Job add failed");

    // Process unprocessed media files every 5 minutes
    let s3 = state.clone();
    scheduler.add(
        Job::new_async("0 */5 * * * *", move |_, _| {
            let s = s3.clone();
            Box::pin(async move {
                if let Err(e) = media_processor::process_pending(&s).await {
                    tracing::error!("Media processor failed: {:?}", e);
                }
            })
        }).expect("Job create failed")
    ).await.expect("Job add failed");

    // AI tagger: tag untagged content every 10 minutes
    let s4 = state.clone();
    scheduler.add(
        Job::new_async("0 */10 * * * *", move |_, _| {
            let s = s4.clone();
            Box::pin(async move {
                if let Err(e) = ai_tagger::tag_pending(&s).await {
                    tracing::error!("AI tagger failed: {:?}", e);
                }
            })
        }).expect("Job create failed")
    ).await.expect("Job add failed");

    scheduler.start().await.expect("Scheduler start failed");
    info!("Background jobs started");
}
