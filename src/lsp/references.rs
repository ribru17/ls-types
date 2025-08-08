use serde::{Deserialize, Serialize};

use crate::lsp::{
    DynamicRegistrationClientCapabilities, PartialResultParams, TextDocumentPositionParams,
    WorkDoneProgressParams,
};

pub type ReferenceClientCapabilities = DynamicRegistrationClientCapabilities;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
    /// Include the declaration of the current symbol.
    pub include_declaration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceParams {
    // Text Document and Position fields
    #[serde(flatten)]
    pub text_document_position: TextDocumentPositionParams,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub partial_result_params: PartialResultParams,

    // ReferenceParams properties:
    pub context: ReferenceContext,
}
