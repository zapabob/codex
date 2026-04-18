use super::PluginManifestInterface;
use super::load_plugin_manifest;
use codex_app_server_protocol::PluginAuthPolicy;
use codex_app_server_protocol::PluginInstallPolicy;
use codex_git_utils::get_git_repo_root;
use codex_plugin::PluginId;
use codex_plugin::PluginIdError;
use codex_protocol::protocol::Product;
use codex_utils_absolute_path::AbsolutePathBuf;
use dirs::home_dir;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use tracing::warn;

const MARKETPLACE_RELATIVE_PATH: &str = ".agents/plugins/marketplace.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedMarketplacePlugin {
    pub plugin_id: PluginId,
    pub source_path: AbsolutePathBuf,
    pub auth_policy: MarketplacePluginAuthPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marketplace {
    pub name: String,
    pub path: AbsolutePathBuf,
    pub interface: Option<MarketplaceInterface>,
    pub plugins: Vec<MarketplacePlugin>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceListError {
    pub path: AbsolutePathBuf,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MarketplaceListOutcome {
    pub marketplaces: Vec<Marketplace>,
    pub errors: Vec<MarketplaceListError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceInterface {
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplacePlugin {
    pub name: String,
    pub source: MarketplacePluginSource,
    pub policy: MarketplacePluginPolicy,
    pub interface: Option<PluginManifestInterface>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplacePluginSource {
    Local { path: AbsolutePathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplacePluginPolicy {
    pub installation: MarketplacePluginInstallPolicy,
    pub authentication: MarketplacePluginAuthPolicy,
    // TODO: Surface or enforce product gating at the Codex/plugin consumer boundary instead of
    // only carrying it through core marketplace metadata.
    pub products: Option<Vec<Product>>,
    pub products: Vec<Product>,

}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum MarketplacePluginInstallPolicy {
    #[serde(rename = "NOT_AVAILABLE")]
    NotAvailable,
    #[default]
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "INSTALLED_BY_DEFAULT")]
    InstalledByDefault,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum MarketplacePluginAuthPolicy {
    #[default]
    #[serde(rename = "ON_INSTALL")]
    OnInstall,
    #[serde(rename = "ON_USE")]
    OnUse,
}

impl From<MarketplacePluginInstallPolicy> for PluginInstallPolicy {
    fn from(value: MarketplacePluginInstallPolicy) -> Self {
        match value {
            MarketplacePluginInstallPolicy::NotAvailable => Self::NotAvailable,
            MarketplacePluginInstallPolicy::Available => Self::Available,
            MarketplacePluginInstallPolicy::InstalledByDefault => Self::InstalledByDefault,
        }
    }
}

impl From<MarketplacePluginAuthPolicy> for PluginAuthPolicy {
    fn from(value: MarketplacePluginAuthPolicy) -> Self {
        match value {
            MarketplacePluginAuthPolicy::OnInstall => Self::OnInstall,
            MarketplacePluginAuthPolicy::OnUse => Self::OnUse,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("{context}: {source}")]
    Io {
        context: &'static str,
        #[source]
        source: io::Error,
    },

    #[error("marketplace file `{path}` does not exist")]
    MarketplaceNotFound { path: PathBuf },

    #[error("invalid marketplace file `{path}`: {message}")]
    InvalidMarketplaceFile { path: PathBuf, message: String },

    #[error("plugin `{plugin_name}` was not found in marketplace `{marketplace_name}`")]
    PluginNotFound {
        plugin_name: String,
        marketplace_name: String,
    },

    #[error(
        "plugin `{plugin_name}` is not available for install in marketplace `{marketplace_name}`"
    )]
    PluginNotAvailable {
        plugin_name: String,
        marketplace_name: String,
    },

    #[error("plugins feature is disabled")]
    PluginsDisabled,

    #[error("{0}")]
    InvalidPlugin(String),
}

impl MarketplaceError {
    fn io(context: &'static str, source: io::Error) -> Self {
        Self::Io { context, source }
    }
}

// Always read the specified marketplace file from disk so installs see the
// latest marketplace.json contents without any in-memory cache invalidation.
pub fn resolve_marketplace_plugin(
    marketplace_path: &AbsolutePathBuf,
    plugin_name: &str,
) -> Result<ResolvedMarketplacePlugin, MarketplaceError> {
    let marketplace = load_marketplace(marketplace_path.as_path())?;
    let marketplace_name = marketplace.name.clone();
    let mut matches = marketplace
        .plugins
        .into_iter()
        .filter(|plugin| plugin.name == plugin_name)
        .collect::<Vec<_>>();

    if matches.len() > 1 {
        return Err(MarketplaceError::DuplicatePlugin {
            plugin_name: plugin_name.to_string(),
            marketplace_name,
        });
    }

    if let Some(plugin) = matches.pop() {
        let plugin_id = PluginId::new(plugin.name, marketplace.name).map_err(|err| match err {
            PluginIdError::Invalid(message) => MarketplaceError::InvalidPlugin(message),
        })?;
        return Ok(ResolvedMarketplacePlugin {
            plugin_id,
            source_path: resolve_plugin_source_path(marketplace_path.as_path(), plugin.source)?,
            auth_policy: plugin.policy.authentication,
        });
    Err(MarketplaceError::PluginNotFound {
        plugin_name: plugin_name.to_string(),
        marketplace_name,
    })
}

pub fn resolve_marketplace_plugin_from_paths(
    marketplace_paths: &[PathBuf],
    plugin_name: &str,
    marketplace_name: &str,
    restriction_product: Option<Product>,
) -> Result<ResolvedMarketplacePlugin, MarketplaceError> {
    for marketplace_path in marketplace_paths {
        let marketplace = load_marketplace(marketplace_path)?;
        let discovered_marketplace_name = marketplace.name;
        let mut matches = marketplace
            .plugins
            .into_iter()
            .filter(|plugin| plugin.name == plugin_name)
            .collect::<Vec<_>>();
        if matches.len() > 1 {
            return Err(MarketplaceError::DuplicatePlugin {
                plugin_name: plugin_name.to_string(),
                marketplace_name: discovered_marketplace_name,
            });
        }
        if let Some(plugin) = matches.pop() {
            let install_policy = plugin.policy.installation;
            let product_allowed = plugin.policy.products.is_empty()
                || restriction_product
                    .is_some_and(|product| product.matches_product_restriction(&plugin.policy.products));

            if install_policy == MarketplacePluginInstallPolicy::NotAvailable || !product_allowed {
                return Err(MarketplaceError::PluginNotAvailable {
                    plugin_name: plugin_name.to_string(),
                    marketplace_name: discovered_marketplace_name,
                });
            }

            let plugin_id =
                PluginId::new(plugin.name, discovered_marketplace_name).map_err(|err| match err {
                    PluginIdError::Invalid(message) => MarketplaceError::InvalidPlugin(message),
                })?;
            return Ok(ResolvedMarketplacePlugin {
                plugin_id,
                source_path: resolve_plugin_source_path(marketplace_path, plugin.source)?,
                auth_policy: plugin.policy.authentication,
            });
        }
    }

    Err(MarketplaceError::PluginNotFound {
        plugin_name: plugin_name.to_string(),
        marketplace_name: marketplace_name.to_string(),
    })
}
    for marketplace_path in marketplace_paths {
        let marketplace = load_marketplace(marketplace_path)?;
        let discovered_marketplace_name = marketplace.name;
        let mut matches = marketplace
            .plugins
            .into_iter()
            .filter(|plugin| plugin.name == plugin_name)
            .collect::<Vec<_>>();
// STASHED:     restriction_product: Option<Product>,
// STASHED: ) -> Result<ResolvedMarketplacePlugin, MarketplaceError> {
// STASHED:     let marketplace = load_raw_marketplace_manifest(marketplace_path)?;
// STASHED:     let marketplace_name = marketplace.name;
// STASHED:     let plugin = marketplace
// STASHED:         .plugins
// STASHED:         .into_iter()
// STASHED:         .find(|plugin| plugin.name == plugin_name);
// STASHED: 
// STASHED:     let Some(plugin) = plugin else {
// STASHED:         return Err(MarketplaceError::PluginNotFound {
// STASHED:             plugin_name: plugin_name.to_string(),
// STASHED:             marketplace_name,
// STASHED:         });
// STASHED:     };


    let RawMarketplaceManifestPlugin {
        name,
        source,
        policy,
        ..
    } = plugin;
    let install_policy = policy.installation;
    let product_allowed = match policy.products.as_deref() {
        None => true,
        Some([]) => false,
        Some(products) => {
            restriction_product.is_some_and(|product| product.matches_product_restriction(products))
        }
    };
    let product_allowed = policy.products.is_empty()
        || restriction_product
            .is_some_and(|product| product.matches_product_restriction(&policy.products));

    if install_policy == MarketplacePluginInstallPolicy::NotAvailable || !product_allowed {
        return Err(MarketplaceError::PluginNotAvailable {
            plugin_name: name,
            marketplace_name,
        });
    }

    let plugin_id = PluginId::new(name, marketplace_name).map_err(|err| match err {
        PluginIdError::Invalid(message) => MarketplaceError::InvalidPlugin(message),
    })?;
    Ok(ResolvedMarketplacePlugin {
        plugin_id,
        source_path: resolve_plugin_source_path(marketplace_path, source)?,
        auth_policy: policy.authentication,
    })
}

pub fn list_marketplaces(
    additional_roots: &[AbsolutePathBuf],
) -> Result<MarketplaceListOutcome, MarketplaceError> {
    list_marketplaces_with_home(additional_roots, home_dir().as_deref())
}

pub(crate) fn load_marketplace(path: &AbsolutePathBuf) -> Result<Marketplace, MarketplaceError> {
    let marketplace = load_raw_marketplace_manifest(path)?;
    let mut plugins = Vec::new();

    for plugin in marketplace.plugins {
        let RawMarketplaceManifestPlugin {
            name,
            source,
            policy,
            category,
        } = plugin;
        let source_path = resolve_plugin_source_path(path, source)?;
        let source = MarketplacePluginSource::Local {
            path: source_path.clone(),
        };
        let mut interface =
            load_plugin_manifest(source_path.as_path()).and_then(|manifest| manifest.interface);
        if let Some(category) = category {
            // Marketplace taxonomy wins when both sources provide a category.
            interface
                .get_or_insert_with(PluginManifestInterface::default)
                .category = Some(category);
        }

        plugins.push(MarketplacePlugin {
            name,
            source,
            policy: MarketplacePluginPolicy {
                installation: policy.installation,
                authentication: policy.authentication,
                products: policy.products,
            },
            interface,
        });
    }

    Ok(Marketplace {
        name: marketplace.name,
        path: path.clone(),
        interface: resolve_marketplace_interface(marketplace.interface),
        plugins,
    })
}

fn list_marketplaces_with_home(
    additional_roots: &[AbsolutePathBuf],
    home_dir: Option<&Path>,
) -> Result<MarketplaceListOutcome, MarketplaceError> {
    let mut outcome = MarketplaceListOutcome::default();

    for marketplace_path in discover_marketplace_paths_from_roots(additional_roots, home_dir) {
        match load_marketplace(&marketplace_path) {
            Ok(marketplace) => outcome.marketplaces.push(marketplace),
            Err(err) => {
                warn!(
                    path = %marketplace_path.display(),
                    error = %err,
                    "skipping marketplace that failed to load"
                );
                outcome.errors.push(MarketplaceListError {
                    path: marketplace_path,
                    message: err.to_string(),
                });
            }
        }
    }

    Ok(outcome)
}

fn discover_marketplace_paths_from_roots(
    additional_roots: &[AbsolutePathBuf],
    home_dir: Option<&Path>,
) -> Vec<AbsolutePathBuf> {
pub fn discover_marketplace_paths(cwd: &Path) -> Vec<PathBuf> {
// STASHED: pub fn list_marketplaces(
// STASHED:     additional_roots: &[AbsolutePathBuf],
// STASHED: ) -> Result<Vec<Marketplace>, MarketplaceError> {
// STASHED:     list_marketplaces_with_home(additional_roots, home_dir().as_deref())
// STASHED: }
// STASHED: 
// STASHED: pub(crate) fn load_marketplace(path: &AbsolutePathBuf) -> Result<Marketplace, MarketplaceError> {
// STASHED:     let marketplace = load_raw_marketplace_manifest(path)?;
// STASHED:     let mut plugins = Vec::new();
// STASHED:     for plugin in marketplace.plugins {
// STASHED:         let RawMarketplaceManifestPlugin {
// STASHED:             name,
// STASHED:             source,
// STASHED:             policy,
// STASHED:             category,
// STASHED:         } = plugin;
// STASHED:         let source_path = resolve_plugin_source_path(path, source)?;
// STASHED:         let source = MarketplacePluginSource::Local {
// STASHED:             path: source_path.clone(),
// STASHED:         };
// STASHED:         let mut interface =
// STASHED:             load_plugin_manifest(source_path.as_path()).and_then(|manifest| manifest.interface);
// STASHED:         if let Some(category) = category {
// STASHED:             // Marketplace taxonomy wins when both sources provide a category.
// STASHED:             interface
// STASHED:                 .get_or_insert_with(PluginManifestInterface::default)
// STASHED:                 .category = Some(category);
// STASHED:         }
// STASHED:         plugins.push(MarketplacePlugin {
// STASHED:             policy: MarketplacePluginPolicy {
// STASHED:                 installation: policy.installation,
// STASHED:                 authentication: policy.authentication,
// STASHED:                 products: policy.products,
// STASHED:             },
// STASHED:             interface,
// STASHED:         });
// STASHED:     }
// STASHED:     Ok(Marketplace {
// STASHED:         name: marketplace.name,
// STASHED:         path: path.clone(),
// STASHED:         interface: resolve_marketplace_interface(marketplace.interface),
// STASHED:         plugins,
// STASHED:     })
// STASHED: fn list_marketplaces_with_home(
// STASHED:     home_dir: Option<&Path>,
// STASHED:     let mut marketplaces = Vec::new();
// STASHED:     for marketplace_path in discover_marketplace_paths_from_roots(additional_roots, home_dir) {
// STASHED:         match load_marketplace(&marketplace_path) {
// STASHED:             Ok(marketplace) => marketplaces.push(marketplace),
// STASHED:             Err(err) => {
// STASHED:                 warn!(
// STASHED:                     path = %marketplace_path.display(),
// STASHED:                     error = %err,
// STASHED:                     "skipping marketplace that failed to load"
// STASHED:                 );
// STASHED:             }
// STASHED:     Ok(marketplaces)
// STASHED: fn discover_marketplace_paths_from_roots(
// STASHED: ) -> Vec<AbsolutePathBuf> {

    let mut paths = Vec::new();

    if let Some(home) = home_dir {
        let path = home.join(MARKETPLACE_RELATIVE_PATH);
        if path.is_file()
            && let Ok(path) = AbsolutePathBuf::try_from(path)
        {
            paths.push(path);
        }
    }

    for root in additional_roots {
        // Curated marketplaces can now come from an HTTP-downloaded directory that is not a git
        // checkout, so check the root directly before falling back to repo-root discovery.
        if let Ok(path) = root.join(MARKETPLACE_RELATIVE_PATH)
            && path.as_path().is_file()
            && !paths.contains(&path)
        {
            paths.push(path);
            continue;
        }
        if let Some(repo_root) = get_git_repo_root(root.as_path())
            && let Ok(repo_root) = AbsolutePathBuf::try_from(repo_root)
            && let Ok(path) = repo_root.join(MARKETPLACE_RELATIVE_PATH)
            && path.as_path().is_file()
            && !paths.contains(&path)
        {
            paths.push(path);
        }
    }

    paths
}

fn load_raw_marketplace_manifest(
    path: &AbsolutePathBuf,
) -> Result<RawMarketplaceManifest, MarketplaceError> {
    let contents = fs::read_to_string(path.as_path()).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            MarketplaceError::MarketplaceNotFound {
                path: path.to_path_buf(),
            }
        } else {
            MarketplaceError::io("failed to read marketplace file", err)
        }
    })?;
    serde_json::from_str(&contents).map_err(|err| MarketplaceError::InvalidMarketplaceFile {
        path: path.to_path_buf(),
        message: err.to_string(),
    })
}

fn resolve_plugin_source_path(
    marketplace_path: &AbsolutePathBuf,
    source: RawMarketplaceManifestPluginSource,
) -> Result<AbsolutePathBuf, MarketplaceError> {
    match source {
        RawMarketplaceManifestPluginSource::Local { path } => {
            let Some(path) = path.strip_prefix("./") else {
                return Err(MarketplaceError::InvalidMarketplaceFile {
                    path: marketplace_path.to_path_buf(),
                    message: "local plugin source path must start with `./`".to_string(),
                });
            };
            if path.is_empty() {
                return Err(MarketplaceError::InvalidMarketplaceFile {
                    path: marketplace_path.to_path_buf(),
                    message: "local plugin source path must not be empty".to_string(),
                });
            }

            let relative_source_path = Path::new(path);
            if relative_source_path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
            {
                return Err(MarketplaceError::InvalidMarketplaceFile {
                    path: marketplace_path.to_path_buf(),
                    message: "local plugin source path must stay within the marketplace root"
                        .to_string(),
                });
            }

            // `marketplace.json` lives under `<root>/.agents/plugins/`, but local plugin paths
            // are resolved relative to `<root>`, not relative to the `plugins/` directory.
            marketplace_root_dir(marketplace_path)?
                .join(relative_source_path)
                .map_err(|err| MarketplaceError::InvalidMarketplaceFile {
                    path: marketplace_path.to_path_buf(),
                    message: format!("plugin source path must resolve to an absolute path: {err}"),
                })
        }
    }
}

fn marketplace_root_dir(
    marketplace_path: &AbsolutePathBuf,
) -> Result<AbsolutePathBuf, MarketplaceError> {
    let Some(plugins_dir) = marketplace_path.parent() else {
        return Err(MarketplaceError::InvalidMarketplaceFile {
            path: marketplace_path.to_path_buf(),
            message: "marketplace file must live under `<root>/.agents/plugins/`".to_string(),
        });
    };
    let Some(dot_agents_dir) = plugins_dir.parent() else {
        return Err(MarketplaceError::InvalidMarketplaceFile {
            path: marketplace_path.to_path_buf(),
            message: "marketplace file must live under `<root>/.agents/plugins/`".to_string(),
        });
    };
    let Some(marketplace_root) = dot_agents_dir.parent() else {
        return Err(MarketplaceError::InvalidMarketplaceFile {
            path: marketplace_path.to_path_buf(),
            message: "marketplace file must live under `<root>/.agents/plugins/`".to_string(),
        });
    };

    if plugins_dir.as_path().file_name().and_then(|s| s.to_str()) != Some("plugins")
        || dot_agents_dir
            .as_path()
            .file_name()
            .and_then(|s| s.to_str())
            != Some(".agents")
    {
        return Err(MarketplaceError::InvalidMarketplaceFile {
            path: marketplace_path.to_path_buf(),
            message: "marketplace file must live under `<root>/.agents/plugins/`".to_string(),
        });
    }

    Ok(marketplace_root)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMarketplaceManifest {
    name: String,
    #[serde(default)]
    interface: Option<RawMarketplaceManifestInterface>,
    plugins: Vec<RawMarketplaceManifestPlugin>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMarketplaceManifestInterface {
    #[serde(default)]
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMarketplaceManifestPlugin {
    name: String,
    source: RawMarketplaceManifestPluginSource,
    #[serde(default)]
    policy: RawMarketplaceManifestPluginPolicy,
    #[serde(default)]
    category: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMarketplaceManifestPluginPolicy {
    #[serde(default)]
    installation: MarketplacePluginInstallPolicy,
    #[serde(default)]
    authentication: MarketplacePluginAuthPolicy,
    products: Option<Vec<Product>>,
    #[serde(default)]
    products: Vec<Product>,

}

#[derive(Debug, Deserialize)]
#[serde(tag = "source", rename_all = "lowercase")]
enum RawMarketplaceManifestPluginSource {
    Local { path: String },
}

fn resolve_marketplace_interface(
    interface: Option<RawMarketplaceManifestInterface>,
) -> Option<MarketplaceInterface> {
    let interface = interface?;
    if interface.display_name.is_some() {
        Some(MarketplaceInterface {
            display_name: interface.display_name,
        })
    } else {
        None
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::tempdir;
    #[test]
    fn resolve_marketplace_plugin_finds_repo_marketplace_plugin() {
        let tmp = tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        fs::create_dir_all(repo_root.join(".git")).unwrap();
        fs::create_dir_all(repo_root.join(".agents/plugins")).unwrap();
        fs::create_dir_all(repo_root.join("nested")).unwrap();
        fs::write(
            repo_root.join(".agents/plugins/marketplace.json"),
            r#"{
  "name": "codex-curated",
  "plugins": [
    {
      "name": "local-plugin",
      "source": {
        "source": "local",
        "path": "./plugin-1"
      }
    }
  ]
}"#,
        )
        .unwrap();
        let marketplace_path =
            AbsolutePathBuf::try_from(repo_root.join(".agents/plugins/marketplace.json")).unwrap();
        let resolved = resolve_marketplace_plugin(&marketplace_path, "local-plugin").unwrap();
        assert_eq!(
            resolved,
            ResolvedMarketplacePlugin {
                plugin_id: PluginId::new("local-plugin".to_string(), "codex-curated".to_string())
                    .unwrap(),
                source_path: AbsolutePathBuf::try_from(repo_root.join(".agents/plugins/plugin-1"))
            }
        );
    fn resolve_marketplace_plugin_reports_missing_plugin() {
            r#"{"name":"codex-curated","plugins":[]}"#,
        let err = resolve_marketplace_plugin(&marketplace_path, "missing").unwrap_err();
            err.to_string(),
            "plugin `missing` was not found in marketplace `codex-curated`"
    fn resolve_marketplace_plugin_prefers_repo_over_home_for_same_plugin() {
        let home_root = tmp.path().join("home");
        let home_marketplace = home_root.join(".agents/plugins/marketplace.json");
        let repo_marketplace = repo_root.join(".agents/plugins/marketplace.json");
        fs::create_dir_all(home_root.join(".agents/plugins")).unwrap();
            home_marketplace.clone(),
        "path": "./home-plugin"
            repo_marketplace.clone(),
        "path": "./repo-plugin"
        let resolved = resolve_marketplace_plugin_from_paths(
            &[repo_marketplace, home_marketplace],
            "local-plugin",
            "codex-curated",
                source_path: AbsolutePathBuf::try_from(
                    repo_root.join(".agents/plugins/repo-plugin"),
                )
                .unwrap(),
    fn resolve_marketplace_plugin_rejects_non_relative_local_paths() {
        "path": "../plugin-1"
        let err = resolve_marketplace_plugin(&marketplace_path, "local-plugin").unwrap_err();
            format!(
                "invalid marketplace file `{}`: local plugin source path must start with `./`",
                repo_root.join(".agents/plugins/marketplace.json").display()
            )
// STASHED: fn resolve_marketplace_interface(
// STASHED:     interface: Option<RawMarketplaceManifestInterface>,
// STASHED: ) -> Option<MarketplaceInterface> {
// STASHED:     let interface = interface?;
// STASHED:     if interface.display_name.is_some() {
// STASHED:         Some(MarketplaceInterface {
// STASHED:             display_name: interface.display_name,
// STASHED:         })
// STASHED:     } else {
// STASHED:         None

    }
}

#[cfg(test)]
#[path = "marketplace_tests.rs"]
mod tests;
