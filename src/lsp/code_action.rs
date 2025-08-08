use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    lsp::{
        Command, Diagnostic, PartialResultParams, Range, TextDocumentIdentifier,
        WorkDoneProgressOptions, WorkDoneProgressParams, WorkspaceEdit,
    },
    macros::lsp_enum,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CodeActionProviderCapability {
    Simple(bool),
    Options(CodeActionOptions),
}

impl From<CodeActionOptions> for CodeActionProviderCapability {
    fn from(from: CodeActionOptions) -> Self {
        Self::Options(from)
    }
}

impl From<bool> for CodeActionProviderCapability {
    fn from(from: bool) -> Self {
        Self::Simple(from)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionClientCapabilities {
    ///
    /// This capability supports dynamic registration.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// The client support code action literals as a valid
    /// response of the `textDocument/codeAction` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action_literal_support: Option<CodeActionLiteralSupport>,

    /// Whether code action supports the `isPreferred` property.
    ///
    /// @since 3.15.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preferred_support: Option<bool>,

    /// Whether code action supports the `disabled` property.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_support: Option<bool>,

    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/codeAction` and a
    /// `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_support: Option<bool>,

    /// Whether the client supports resolving additional code action
    /// properties via a separate `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_support: Option<CodeActionCapabilityResolveSupport>,

    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// `CodeAction#edit` property by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_change_annotations: Option<bool>,
}

/// Whether the client supports resolving additional code action
/// properties via a separate `codeAction/resolve` request.
///
/// @since 3.16.0
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionCapabilityResolveSupport {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionLiteralSupport {
    /// The code action kind is support with the following value set.
    pub code_action_kind: CodeActionKindLiteralSupport,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionKindLiteralSupport {
    /// The code action kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Vec<String>,
}

/// Params for the `CodeActionRequest`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionParams {
    /// The document in which the command was invoked.
    pub text_document: TextDocumentIdentifier,

    /// The range for which the command was invoked.
    pub range: Range,

    /// Context carrying additional information.
    pub context: CodeActionContext,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub partial_result_params: PartialResultParams,
}

/// Response for `CodeActionRequest`
pub type CodeActionResponse = Vec<CodeActionOrCommand>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CodeActionOrCommand {
    Command(Command),
    CodeAction(Box<CodeAction>),
}

impl From<Command> for CodeActionOrCommand {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl From<CodeAction> for CodeActionOrCommand {
    fn from(action: CodeAction) -> Self {
        Self::CodeAction(Box::new(action))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize, Hash)]
pub struct CodeActionKind(Cow<'static, str>);

impl CodeActionKind {
    /// Empty kind.
    pub const EMPTY: Self = Self::new("");

    /// Base kind for quickfix actions: 'quickfix'
    pub const QUICKFIX: Self = Self::new("quickfix");

    /// Base kind for refactoring actions: 'refactor'
    pub const REFACTOR: Self = Self::new("refactor");

    /// Base kind for refactoring extraction actions: 'refactor.extract'
    ///
    /// Example extract actions:
    ///
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    /// - ...
    pub const REFACTOR_EXTRACT: Self = Self::new("refactor.extract");

    /// Base kind for refactoring inline actions: 'refactor.inline'
    ///
    /// Example inline actions:
    ///
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    /// - ...
    pub const REFACTOR_INLINE: Self = Self::new("refactor.inline");

    /// Base kind for refactoring rewrite actions: 'refactor.rewrite'
    ///
    /// Example rewrite actions:
    ///
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    /// - ...
    pub const REFACTOR_REWRITE: Self = Self::new("refactor.rewrite");

    /// Base kind for source actions: `source`
    ///
    /// Source code actions apply to the entire file.
    pub const SOURCE: Self = Self::new("source");

    /// Base kind for an organize imports source action: `source.organizeImports`
    pub const SOURCE_ORGANIZE_IMPORTS: Self = Self::new("source.organizeImports");

    /// Base kind for a 'fix all' source action: `source.fixAll`.
    ///
    /// 'Fix all' actions automatically fix errors that have a clear fix that
    /// do not require user input. They should not suppress errors or perform
    /// unsafe fixes such as generating new types or classes.
    ///
    /// @since 3.17.0
    pub const SOURCE_FIX_ALL: Self = Self::new("source.fixAll");

    #[must_use]
    pub const fn new(tag: &'static str) -> Self {
        Self(Cow::Borrowed(tag))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for CodeActionKind {
    fn from(from: String) -> Self {
        Self(Cow::from(from))
    }
}

impl From<&'static str> for CodeActionKind {
    fn from(from: &'static str) -> Self {
        Self::new(from)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    /// A short, human-readable, title for this code action.
    pub title: String,

    /// The kind of the code action.
    /// Used to filter code actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CodeActionKind>,

    /// The diagnostics that this code action resolves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<Diagnostic>>,

    /// The workspace edit this code action performs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<WorkspaceEdit>,

    /// A command this code action executes. If a code action
    /// provides an edit and a command, first the edit is
    /// executed and then the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,

    /// Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can be targeted
    /// by keybindings.
    /// A quick fix should be marked preferred if it properly addresses the underlying error.
    /// A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
    ///
    /// @since 3.15.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<bool>,

    /// Marks that the code action cannot currently be applied.
    ///
    /// Clients should follow the following guidelines regarding disabled code actions:
    ///
    /// - Disabled code actions are not shown in automatic
    ///   [lightbulb](https://code.visualstudio.com/docs/editor/editingevolved#_code-action)
    ///   code action menu.
    ///
    /// - Disabled actions are shown as faded out in the code action menu when the user request
    ///   a more specific type of code action, such as refactorings.
    ///
    /// - If the user has a keybinding that auto applies a code action and only a disabled code
    ///   actions are returned, the client should show the user an error message with `reason`
    ///   in the editor.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<CodeActionDisabled>,

    /// A data entry field that is preserved on a code action between
    /// a `textDocument/codeAction` and a `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionDisabled {
    /// Human readable description of why the code action is currently disabled.
    ///
    /// This is displayed in the code actions UI.
    pub reason: String,
}

/// The reason why code actions were requested.
///
/// @since 3.17.0
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CodeActionTriggerKind(i32);

lsp_enum! {
    impl CodeActionTriggerKind {
        /// Code actions were explicitly requested by the user or by an extension.
        const INVOKED = 1;

        /// Code actions were requested automatically.
        ///
        /// This typically happens when current selection in a file changes, but can
        /// also be triggered when file content changes.
        const AUTOMATIC = 2;
    }
}

/// Contains additional diagnostic information about the context in which
/// a code action is run.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionContext {
    /// An array of diagnostics.
    pub diagnostics: Vec<Diagnostic>,

    /// Requested kind of actions to return.
    ///
    /// Actions not of this kind are filtered out by the client before being shown. So servers
    /// can omit computing them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<CodeActionKind>>,

    /// The reason why code actions were requested.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_kind: Option<CodeActionTriggerKind>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionOptions {
    /// `CodeActionKinds` that this server may return.
    ///
    /// The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
    /// may list out every specific kind they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action_kinds: Option<Vec<CodeActionKind>>,

    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,

    /// The server provides support to resolve additional
    /// information for a code action.
    ///
    /// @since 3.16.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_serialization;

    #[test]
    fn test_code_action_response() {
        test_serialization(
            &vec![
                CodeActionOrCommand::Command(Command {
                    title: "title".to_string(),
                    command: "command".to_string(),
                    arguments: None,
                }),
                CodeActionOrCommand::CodeAction(Box::new(CodeAction {
                    title: "title".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    command: None,
                    diagnostics: None,
                    edit: None,
                    is_preferred: None,
                    ..CodeAction::default()
                })),
            ],
            r#"[{"title":"title","command":"command"},{"title":"title","kind":"quickfix"}]"#,
        );
    }
}
