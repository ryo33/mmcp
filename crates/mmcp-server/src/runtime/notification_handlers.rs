use mmcp_protocol::mcp::{
    CancelledNotification, ClientNotification, InitializedNotification, ProgressNotification,
    RootsListChangedNotification,
};

use crate::MCPServer;

impl MCPServer {
    pub async fn handle_notification(
        &self,
        notification: mmcp_protocol::mcp::JSONRPCNotification,
    ) -> anyhow::Result<()> {
        let client_notification =
            serde_json::from_value::<ClientNotification>(serde_json::json!({
                "method": notification.method,
                "params": notification.params,
            }))?;

        match client_notification {
            ClientNotification::CancelledNotification(n) => {
                self.handle_cancelled_notification(n).await
            }
            ClientNotification::InitializedNotification(n) => {
                self.handle_initialized_notification(n).await
            }
            ClientNotification::ProgressNotification(n) => {
                self.handle_progress_notification(n).await
            }
            ClientNotification::RootsListChangedNotification(n) => {
                self.handle_roots_list_changed_notification(n).await
            }
        }
    }

    async fn handle_cancelled_notification(
        &self,
        _notification: CancelledNotification,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_initialized_notification(
        &self,
        _notification: InitializedNotification,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_progress_notification(
        &self,
        _notification: ProgressNotification,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_roots_list_changed_notification(
        &self,
        _notification: RootsListChangedNotification,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
