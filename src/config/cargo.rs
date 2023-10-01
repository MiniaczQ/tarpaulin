use serde::{Deserialize, Serialize};
use tracing::info;

use super::{Mode, Config};

/// Configuration exclusive to `cargo` usage
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CargoConfig {
    /// Don't update `Cargo.lock`.
    pub locked: bool,
    /// Don't update `Cargo.lock` or any caches.
    pub frozen: bool,
    /// Run with the given profile
    pub profile: Option<String>,
    /// Number of jobs used for building the tests
    pub jobs: Option<usize>,
    /// Include all available features in target build
    #[serde(rename = "all-features")]
    pub all_features: bool,
    /// Do not include default features in target build
    #[serde(rename = "no-default-features")]
    pub no_default_features: bool,
    /// Features to include in the target project build, e.g. "feature1 feature2"
    pub features: Option<String>,
    /// Build all packages in the workspace
    #[serde(alias = "workspace")]
    pub all: bool,
    /// Build in release mode
    pub release: bool,
    /// Packages to include when building the target project
    pub packages: Vec<String>,
    /// Packages to exclude from testing
    pub exclude: Vec<String>,
    /// Build for the target triple.
    pub target: Option<String>,
    /// Run tarpaulin on project without accessing the network
    pub offline: bool,
    /// Unstable cargo features to use
    #[serde(rename = "Z")]
    pub unstable_features: Vec<String>,
    /// Cargo subcommand to run. So far only test and build are supported
    pub command: Mode,
    /// Varargs to be forwarded to the test executables.
    #[serde(rename = "args")]
    pub varargs: Vec<String>,
}

impl Default for CargoConfig {
    fn default() -> Self {
        Self {
            locked: false,
            command: Mode::Test,
            no_default_features: false,
            features: None,
            unstable_features: vec![],
            all: false,
            packages: vec![],
            exclude: vec![],
            varargs: vec![],
            release: false,
            all_features: false,
            frozen: false,
            target: None,
            offline: false,
            profile: None,
            jobs: None,
        }
    }
}

impl CargoConfig {
    pub fn merge(&mut self, other: &CargoConfig) {
        self.no_default_features |= other.no_default_features;
        self.release |= other.release;
        self.all_features |= other.all_features;
        self.offline |= other.offline;
        self.target = Config::pick_optional_config(&self.target, &other.target);
        self.all |= other.all;
        self.frozen |= other.frozen;
        self.locked |= other.locked;
        if self.jobs.is_none() {
            self.jobs = other.jobs;
        }

        if self.profile.is_none() && other.profile.is_some() {
            self.profile = other.profile.clone();
        }
        if other.features.is_some() {
            if self.features.is_none() {
                self.features = other.features.clone();
            } else if let Some(features) = self.features.as_mut() {
                features.push(' ');
                features.push_str(other.features.as_ref().unwrap());
            }
        }

        let additional_packages = other
            .packages
            .iter()
            .filter(|package| !self.packages.contains(package))
            .cloned()
            .collect::<Vec<String>>();
        self.packages.extend(additional_packages);

        let additional_excludes = other
            .exclude
            .iter()
            .filter(|package| !self.exclude.contains(package))
            .cloned()
            .collect::<Vec<String>>();
        self.exclude.extend(additional_excludes);

        let additional_varargs = other
            .varargs
            .iter()
            .filter(|package| !self.varargs.contains(package))
            .cloned()
            .collect::<Vec<String>>();
        self.varargs.extend(additional_varargs);

        let additional_z_opts = other
            .unstable_features
            .iter()
            .filter(|package| !self.unstable_features.contains(package))
            .cloned()
            .collect::<Vec<String>>();
        self.unstable_features.extend(additional_z_opts);

        let exclude = &self.exclude;
        self.packages.retain(|package| {
            let keep = !exclude.contains(package);
            if !keep {
                info!("{} is in exclude list removing from packages", package);
            }
            keep
        });
    }
}
