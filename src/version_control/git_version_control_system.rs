use std::any::Any;
use crate::command::Command;
use crate::terminal_output::TerminalOutput;
use crate::version_control::download_status::DownloadStatus;
use crate::version_control::version_control_system::VersionControlSystem;

pub struct GitVersionControlSystem {
    /// The git command
    git_command: Box<dyn Command>,

    /// The folder where all repositories are going to be cloned into
    destination_folder: String
}

impl VersionControlSystem for GitVersionControlSystem {
    fn download(&self, repositories: &Vec<String>) -> Result<DownloadStatus, &str> {
        for repository in repositories {
            self.clone_repository(&repository);
        }

        return Ok(DownloadStatus::Downloaded);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GitVersionControlSystem {
    pub fn new(git_command: Box<dyn Command>, destination_folder: &str) -> Box<GitVersionControlSystem> {
        return Box::new(Self {
            git_command,
            destination_folder: destination_folder.to_string()
        });
    }

    fn clone_repository(&self, repository: &String) {
        let terminal_output = TerminalOutput::new();
        terminal_output.loading(&format!("Cloning {}", &repository));

        let result = self.git_command
            .execute(&[
                "clone".to_string(),
                repository.to_string()
            ]);
        let is_success = result.is_ok_and(|output| output);

        if is_success {
            terminal_output.finish_with_success(&format!("Successfully cloned {}", &repository));
        } else {
            terminal_output.finish_with_failure(&format!("Unable to clone {}", &repository));
        }
    }
}