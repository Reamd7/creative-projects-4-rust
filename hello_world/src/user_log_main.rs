use log::{debug, error, info, warn};

pub fn user_log_main() {
    env_logger::init();
    error!("Error Message");
    warn!("Warning Message");
    info!("Info Message");
    debug!("Debugger Message");
}