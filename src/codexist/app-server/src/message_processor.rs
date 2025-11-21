use std::path::PathBuf;

use crate::codexist_message_processor::CodexistMessageProcessor;
use crate::error_code::INVALID_REQUEST_ERROR_CODE;
use crate::outgoing_message::OutgoingMessageSender;
use codexist_app_server_protocol::ClientInfo;
use codexist_app_server_protocol::ClientRequest;
use codexist_app_server_protocol::InitializeResponse;

use codexist_app_server_protocol::JSONRPCError;
use codexist_app_server_protocol::JSONRPCErrorError;
use codexist_app_server_protocol::JSONRPCNotification;
use codexist_app_server_protocol::JSONRPCRequest;
use codexist_app_server_protocol::JSONRPCResponse;
use codexist_core::AuthManager;
use codexist_core::ConversationManager;
use codexist_core::config::Config;
use codexist_core::default_client::USER_AGENT_SUFFIX;
use codexist_core::default_client::get_codexist_user_agent;
use codexist_feedback::CodexistFeedback;
use codexist_protocol::protocol::SessionSource;
use std::sync::Arc;

pub(crate) struct MessageProcessor {
    outgoing: Arc<OutgoingMessageSender>,
    codexist_message_processor: CodexistMessageProcessor,
    initialized: bool,
}

impl MessageProcessor {
    /// Create a new `MessageProcessor`, retaining a handle to the outgoing
    /// `Sender` so handlers can enqueue messages to be written to stdout.
    pub(crate) fn new(
        outgoing: OutgoingMessageSender,
        codexist_linux_sandbox_exe: Option<PathBuf>,
        config: Arc<Config>,
        feedback: CodexistFeedback,
    ) -> Self {
        let outgoing = Arc::new(outgoing);
        let auth_manager = AuthManager::shared(
            config.codexist_home.clone(),
            false,
            config.cli_auth_credentials_store_mode,
        );
        let conversation_manager = Arc::new(ConversationManager::new(
            auth_manager.clone(),
            SessionSource::VSCode,
        ));
        let codexist_message_processor = CodexistMessageProcessor::new(
            auth_manager,
            conversation_manager,
            outgoing.clone(),
            codexist_linux_sandbox_exe,
            config,
            feedback,
        );

        Self {
            outgoing,
            codexist_message_processor,
            initialized: false,
        }
    }

    pub(crate) async fn process_request(&mut self, request: JSONRPCRequest) {
        let request_id = request.id.clone();
        let request_json = match serde_json::to_value(&request) {
            Ok(request_json) => request_json,
            Err(err) => {
                let error = JSONRPCErrorError {
                    code: INVALID_REQUEST_ERROR_CODE,
                    message: format!("Invalid request: {err}"),
                    data: None,
                };
                self.outgoing.send_error(request_id, error).await;
                return;
            }
        };

        let codexist_request = match serde_json::from_value::<ClientRequest>(request_json) {
            Ok(codexist_request) => codexist_request,
            Err(err) => {
                let error = JSONRPCErrorError {
                    code: INVALID_REQUEST_ERROR_CODE,
                    message: format!("Invalid request: {err}"),
                    data: None,
                };
                self.outgoing.send_error(request_id, error).await;
                return;
            }
        };

        match codexist_request {
            // Handle Initialize internally so CodexistMessageProcessor does not have to concern
            // itself with the `initialized` bool.
            ClientRequest::Initialize { request_id, params } => {
                if self.initialized {
                    let error = JSONRPCErrorError {
                        code: INVALID_REQUEST_ERROR_CODE,
                        message: "Already initialized".to_string(),
                        data: None,
                    };
                    self.outgoing.send_error(request_id, error).await;
                    return;
                } else {
                    let ClientInfo {
                        name,
                        title: _title,
                        version,
                    } = params.client_info;
                    let user_agent_suffix = format!("{name}; {version}");
                    if let Ok(mut suffix) = USER_AGENT_SUFFIX.lock() {
                        *suffix = Some(user_agent_suffix);
                    }

                    let user_agent = get_codexist_user_agent();
                    let response = InitializeResponse { user_agent };
                    self.outgoing.send_response(request_id, response).await;

                    self.initialized = true;
                    return;
                }
            }
            _ => {
                if !self.initialized {
                    let error = JSONRPCErrorError {
                        code: INVALID_REQUEST_ERROR_CODE,
                        message: "Not initialized".to_string(),
                        data: None,
                    };
                    self.outgoing.send_error(request_id, error).await;
                    return;
                }
            }
        }

        self.codexist_message_processor
            .process_request(codexist_request)
            .await;
    }

    pub(crate) async fn process_notification(&self, notification: JSONRPCNotification) {
        // Currently, we do not expect to receive any notifications from the
        // client, so we just log them.
        tracing::info!("<- notification: {:?}", notification);
    }

    /// Handle a standalone JSON-RPC response originating from the peer.
    pub(crate) async fn process_response(&mut self, response: JSONRPCResponse) {
        tracing::info!("<- response: {:?}", response);
        let JSONRPCResponse { id, result, .. } = response;
        self.outgoing.notify_client_response(id, result).await
    }

    /// Handle an error object received from the peer.
    pub(crate) fn process_error(&mut self, err: JSONRPCError) {
        tracing::error!("<- error: {:?}", err);
    }
}
