
use rmcp::{model::*, service::RequestContext, Error as McpError, RoleServer, ServerHandler};
use serde::{Serialize, Deserialize};
use rmcp::handler::server::tool::IntoCallToolResult;
use serde_json::json;
use std::string::ToString;

use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
};



#[derive(Clone)]
pub struct CrayonMcpServer;

impl CrayonMcpServer {
    pub fn new() -> Self {
        Self
    }
}


#[tool(tool_box)]
impl ServerHandler for CrayonMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("This server provides crayon tools".to_string()),
            capabilities: ServerCapabilities::builder()
            .enable_resources()
            .enable_tools()
            .build(),
            ..Default::default()
        }
    }
    async fn list_resources(
            &self,
            request: PaginatedRequestParam,
            context: rmcp::service::RequestContext<RoleServer>,
        ) -> Result<ListResourcesResult, McpError> {
            Ok(ListResourcesResult {
                resources: vec![
                    AppState::resource(),
                ],
                next_cursor: None,
            })
    }
    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }
    
    
}


pub struct AppState {
    pub tool_state: ToolState,
}

impl AppState {
    fn resource() -> Resource {
       RawResource::new(
        "app_state",
        "The state of the Crayon application",
       ).no_annotation()
    }
}




/// Represents the set of tools available in the Crayon application.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, schemars::JsonSchema, strum::Display)]
pub enum ToolState {
    /// The transform tool, used for moving and resizing objects on the canvas.
    #[default]
    #[schemars(description = "the transform tool")]
    Transform,
    /// The comment tool, used for adding annotations to the canvas.
    #[schemars(description = "the comment tool")]
    Comment,
    /// The markup tool, used for drawing or highlighting areas on the canvas.
    #[schemars(description = "the markup tool")]
    Markup,
    /// The block tool, used for creating or manipulating block elements.
    #[schemars(description = "the block tool")]
    Block,
}





/// Represents incoming events/commands for the Crayon application.
/// These are used to control the application state or trigger actions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, strum::Display)]
#[serde(tag = "type", content = "data")]
pub enum CrayonInEvent {
    /// Change the active tool in the application.
    #[schemars(description = "change the active tool")]
    ChangeTool(ToolState),
    /// Exit the application.
    #[schemars(description = "exit the application")]
    ExitApp,
}
impl IntoCallToolResult for CrayonInEvent {
    fn into_call_tool_result(self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            self.to_string(),
        )]))
    }
}


#[tool(tool_box)]
impl CrayonMcpServer {
    /// Triggers the application to exit.
    #[tool(description = "Exit the Crayon application.")]
    fn exit_app() -> CrayonInEvent {
        CrayonInEvent::ExitApp
    }

    /// Switches the active tool to Transform mode for manipulating objects.
    #[tool(description = "Switch to the transform tool for moving and resizing objects on the canvas.")]
    fn transform_tool() -> CrayonInEvent {
        CrayonInEvent::ChangeTool(ToolState::Transform)
    }

    /// Switches the active tool to Comment mode for adding annotations.
    #[tool(description = "Switch to the comment tool for adding annotations to the canvas.")]
    fn comment_tool() -> CrayonInEvent {
        CrayonInEvent::ChangeTool(ToolState::Comment)
    }

    /// Switches the active tool to Markup mode for drawing or highlighting.
    #[tool(description = "Switch to the markup tool for drawing or highlighting areas on the canvas.")]
    fn markup_tool() -> CrayonInEvent {
        CrayonInEvent::ChangeTool(ToolState::Markup)
    }

    /// Switches the active tool to Block mode for creating or manipulating block elements.
    #[tool(description = "Switch to the block tool for creating or manipulating block elements.")]
    fn block_tool() -> CrayonInEvent {
        CrayonInEvent::ChangeTool(ToolState::Block)
    }
  
}