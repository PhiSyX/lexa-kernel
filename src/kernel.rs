// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

pub(super) mod error;
pub(super) mod extension;
pub(super) mod interface;
pub(super) mod settings;

use std::path;

use self::settings::KernelSettings;
use crate::logger::signal::LoggerSignal;
use crate::process::ProcessMode;
use crate::{ApplicationAdapterInterface, ApplicationAdapterSettingsInterface};

// --------- //
// Structure //
// --------- //

pub struct Kernel<ApplicationAdapter, UserEnv = (), UserCLI = ()>
{
	/// Nom de l'application.
	application_name: ApplicationName,
	/// Version de l'application.
	application_version: ApplicationVersion,
	/// Application Adapter.
	pub application_adapter: ApplicationAdapter,
	/// Paramètres du kernel.
	pub settings: KernelSettings,
	/// Les variables d'environnent.
	env_vars: Option<UserEnv>,
	/// Arguments de la CLI.
	cli_args: Option<UserCLI>,
	/// Logger Signal.
	logger_signal: LoggerSignal,
}

pub type ApplicationName = String;
pub type ApplicationVersion = String;

// -------------- //
// Implémentation //
// -------------- //

impl<ApplicationAdapter, E, C> Kernel<ApplicationAdapter, E, C>
where
	ApplicationAdapter: ApplicationAdapterInterface,
{
	pub fn new(
		application_name: impl Into<ApplicationName>,
		application_version: impl Into<ApplicationVersion>,
		application_root_directory: impl Into<path::PathBuf>,
	) -> Self
	{
		let settings = KernelSettings::new(application_root_directory);

		let application_name = application_name.into();
		let application_version = application_version.into();

		let logger_signal = LoggerSignal::create(&application_name, &application_version);

		Self {
			application_name,
			application_version,
			application_adapter: Self::make_adapter(&settings),
			settings,
			env_vars: Default::default(),
			cli_args: Default::default(),
			logger_signal,
		}
	}

	fn make_adapter(settings: &KernelSettings) -> ApplicationAdapter
	{
		let application_adapter_settings_filename =
			<ApplicationAdapter::Settings as ApplicationAdapterSettingsInterface>::FILENAME;

		let application_adapter_settings =
			fetch_config(application_adapter_settings_filename, &settings).unwrap_or_default();

		ApplicationAdapter::new(application_adapter_settings)
	}
}

impl<A, E, C> Kernel<A, E, C>
{
	/// Définit le répertoire racine du projet.
	pub fn define_project_directory(mut self, dir: impl Into<path::PathBuf>) -> Self
	{
		self.settings.directory.set_project_directory(dir);
		self
	}

	/// Définit un répertoire de configuration.
	///
	/// NOTE: lorsque le répertoire de configuration est redéfinit,
	/// l'application adapter est recrée.
	pub fn define_config_directory(mut self, dir: impl Into<path::PathBuf>) -> Self
	where
		A: ApplicationAdapterInterface,
	{
		self.settings.directory.set_config_directory(dir);
		self.application_adapter = Self::make_adapter(&self.settings);
		self
	}

	/// Définit le type d'extension à récupérer pour les fichiers de
	/// configurations de l'application.
	pub fn define_loader_extension(mut self, loader_extension: impl Into<lexa_fs::Extension>) -> Self
	{
		self.settings.loader_extension = loader_extension.into();
		self
	}

	/// Voir [fetch_config()]
	pub fn fetch_config<O>(&self, config_name: impl AsRef<str> + 'static) -> std::io::Result<O>
	where
		O: serde::de::DeserializeOwned,
	{
		fetch_config(config_name, &self.settings)
	}

	/// Voir [fetch_config_or_prompt()]
	pub fn fetch_config_or_prompt<O>(&self, config_name: impl AsRef<str> + 'static) -> std::io::Result<O>
	where
		O: std::fmt::Debug,
		O: lexa_prompt::Prompt,
		O: serde::ser::Serialize + serde::de::DeserializeOwned,
	{
		fetch_config_or_prompt(config_name, &self.settings)
	}

	pub fn signal(&self) -> &LoggerSignal
	{
		&self.logger_signal
	}
}

// -------- //
// Fonction //
// -------- //

/// Désérialise un fichier de configuration situé dans son répertoire de
/// configuration en une structure de données en fonction du mode d'exécution.
///
/// À savoir que **par défaut** :
///
///     1) Le répertoire de configuration se trouve à la racine du
///     projet `config/`
///
///     2) Le fichier de configuration du logger se trouve dans :
///
///         2.1) En local : `config/<config_name>.<EXT>`.
///         2.2) En dev   : `config/dev/<config_name>.<EXT>`.
///         2.3) En prod  : `config/prod/<config_name>.<EXT>`.
///         2.4) En test  : `config/test/<config_name>.<EXT>`.
///
///     3) L'extension <EXT> utilisée pour ce fichier de configuration est le
///        `yml`. Cette extension peut être modifiée dans les paramètres de la
///        configuration.
pub fn fetch_config<O>(config_name: impl AsRef<str> + 'static, settings: &KernelSettings) -> std::io::Result<O>
where
	O: serde::de::DeserializeOwned,
{
	let filepath = match settings.process_mode {
		| ProcessMode::LOCAL => String::from(config_name.as_ref()),
		| ProcessMode::DEVELOPMENT => String::from("dev/{config_name}"),
		| ProcessMode::PRODUCTION => String::from("prod/{config_name}"),
		| ProcessMode::TEST => String::from("test/{config_name}"),
	};

	if let Some(config_directory) = settings.directory.config() {
		lexa_fs::load(config_directory, filepath, settings.loader_extension)
	} else {
		Err(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"Le répertoire de configuration n'existe pas.",
		))
	}
}

/// Désérialise un fichier de configuration situé dans son répertoire de
/// configuration en une structure de données en fonction du mode d'exécution.
///
/// À savoir que **par défaut** :
///
///     1) Le répertoire de configuration se trouve à la racine du
///     projet `config/`
///
///     2) Le fichier de configuration du logger se trouve dans :
///
///         2.1) En local : `config/<config_name>.<EXT>`.
///         2.2) En dev   : `config/dev/<config_name>.<EXT>`.
///         2.3) En prod  : `config/prod/<config_name>.<EXT>`.
///         2.4) En test  : `config/test/<config_name>.<EXT>`.
///
///     3) L'extension <EXT> utilisée pour ce fichier de configuration est le
///        `yml`. Cette extension peut être modifiée dans les paramètres de la
///        configuration.
pub fn fetch_config_or_prompt<O>(
	config_name: impl AsRef<str> + 'static,
	settings: &KernelSettings,
) -> std::io::Result<O>
where
	O: std::fmt::Debug,
	O: lexa_prompt::Prompt,
	O: serde::ser::Serialize + serde::de::DeserializeOwned,
{
	let filepath = match settings.process_mode {
		| ProcessMode::LOCAL => String::from(config_name.as_ref()),
		| ProcessMode::DEVELOPMENT => String::from("dev/{config_name}"),
		| ProcessMode::PRODUCTION => String::from("prod/{config_name}"),
		| ProcessMode::TEST => String::from("test/{config_name}"),
	};

	if let Some(config_directory) = settings.directory.config() {
		lexa_fs::load_or_prompt(config_directory, filepath, settings.loader_extension)
	} else {
		Err(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"Le répertoire de configuration n'existe pas.",
		))
	}
}
