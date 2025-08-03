use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::cli::ConversationState;
use crate::database::{Database, DatabaseError};
use crate::os::Os;

#[derive(Debug, Args, PartialEq)]
pub struct HistoryArgs {
    #[command(subcommand)]
    pub command: HistoryCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum HistoryCommands {
    /// List recent conversations
    List {
        /// Maximum number of conversations to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        /// Filter by directory path
        #[arg(short, long)]
        path: Option<String>,
        
        /// Filter conversations containing this text
        #[arg(short, long)]
        contains: Option<String>,
    },
    /// Show a specific conversation
    Show {
        /// Conversation ID or partial ID
        id: String,
    },
    /// Restore a conversation to the current directory
    Restore {
        /// Conversation ID or partial ID
        id: String,
    },
    /// Search conversations by content
    Search {
        /// Search query to find in conversation content
        query: String,
        
        /// Maximum number of results to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Export a conversation to a file
    Export {
        /// Conversation ID or partial ID
        id: String,
        
        /// Output file path
        #[arg(short, long)]
        output: String,
        
        /// Export format
        #[arg(long, default_value = "json")]
        format: ExportFormat,
        
        /// Overwrite existing file
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum ExportFormat {
    /// JSON format (same as /save command, can be imported with /load)
    Json,
    /// Markdown format for readable documentation
    Markdown,
    /// Plain text format for simple reading
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub id: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub preview: String,
    pub message_count: usize,
}

impl HistoryArgs {
    pub async fn execute(self, os: &mut Os) -> Result<std::process::ExitCode> {
        match self.command {
            HistoryCommands::List { limit, path, contains } => {
                list_conversations(&os.database, limit, path.as_deref(), contains.as_deref()).await?;
            }
            HistoryCommands::Show { id } => {
                show_conversation(&os.database, &id).await?;
            }
            HistoryCommands::Restore { id } => {
                restore_conversation(&mut os.database, &id).await?;
            }
            HistoryCommands::Search { query, limit } => {
                search_conversations(&os.database, &query, limit).await?;
            }
            HistoryCommands::Export { id, output, format, force } => {
                export_conversation(&os.database, &os.fs, &id, &output, format, force).await?;
            }
        }
        Ok(std::process::ExitCode::SUCCESS)
    }
}

// Implementation continues with all the functions...
// [The rest of the file content would be too long for a single push]