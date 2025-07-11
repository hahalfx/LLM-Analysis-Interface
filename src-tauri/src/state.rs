use crate::db::database::DatabaseService;
use crate::services::audio_controller::AudioController;
use crate::services::workflow::ControlHandle;
use std::sync::Arc;
use serde::de::Expected;
use tauri::ipc::Channel;
use tokio::sync::Mutex;
use reqwest::Client;
use parking_lot::Mutex as ParkingLotMutex;
use tesseract::Tesseract;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseService>,
    pub current_task_id: Arc<tokio::sync::RwLock<Option<i64>>>,
    pub is_testing: Arc<tokio::sync::RwLock<bool>>,
    pub audio_controller: AudioController,// 这个音频控制器如果后面不用可以删掉
    pub workflow_handle: Arc<Mutex<Option<ControlHandle>>>,
    pub http_client: Client,
    pub ocr_engine: Arc<ParkingLotMutex<Option<Tesseract>>>,
    pub ocr_channel: Arc<Mutex<Option<Channel>>>, 
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let db = Arc::new(DatabaseService::new(database_url).await?);

        // 初始化默认数据
        db.initialize_default_data().await?;

        let (audio_controller, _audio_task_handle) = AudioController::new();

        Ok(Self {
            db,
            current_task_id: Arc::new(tokio::sync::RwLock::new(None)),
            is_testing: Arc::new(tokio::sync::RwLock::new(false)),
            audio_controller,
            workflow_handle: Arc::new(Mutex::new(None)),
            http_client: Client::new(),
            ocr_engine: Arc::new(ParkingLotMutex::new(None)),
            ocr_channel: Arc::new(Mutex::new(None)),
        })
    }
}
