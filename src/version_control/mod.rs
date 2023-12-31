mod version_control_system;
mod git_version_control_system;
mod version_control_system_factory;

pub use version_control_system::MockVersionControlSystem;
pub use version_control_system::VersionControlSystem;
pub use version_control_system_factory::VersionControlSystemFactory;
pub use version_control_system_factory::VersionControlSystemFactoryImpl;