// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::path;

use crate::process::ProcessMode;

// --------- //
// Structure //
// --------- //

pub struct KernelSettings
{
	/// Répertoires d'application.
	pub directory: KernelSettingsDirectory,
	/// Quelle extension doit-on utiliser pour lire les fichiers de
	/// configuration.
	pub(super) loader_extension: lexa_fs::Extension,
	/// Mode d’exécution de l'application.
	pub(super) process_mode: ProcessMode,
	/// Affiche les informations de l'application dès le démarrage?
	pub(super) startup_info: bool,
}

#[derive(Default)]
pub struct KernelSettingsDirectory
{
	/// Répertoire racine de l'application.
	root: path::PathBuf,
	/// Répertoire de configuration de l'application.
	config: Option<path::PathBuf>,
	/// Répertoire des variables d'environnement de l'application.
	env: Option<path::PathBuf>,
}

// -------------- //
// Implémentation //
// -------------- //

impl KernelSettings
{
	/// Construit les paramètres du kernel.
	pub fn new(dir: impl Into<path::PathBuf>) -> Self
	{
		let process_mode = ProcessMode::from_rustcfg();

		let root_directory = dir.into();

		Self {
			directory: KernelSettingsDirectory {
				config: Some(root_directory.join("config")),
				env: Some(root_directory.join("env")),
				root: root_directory,
			},
			loader_extension: lexa_fs::Extension::YAML,
			process_mode,
			startup_info: true,
		}
	}
}

impl KernelSettingsDirectory
{
	/// Répertoire racine de l'application.
	pub fn root(&self) -> &path::Path
	{
		&self.root
	}

	/// Répertoire de la configuration de l'application.
	pub fn config(&self) -> Option<&path::Path>
	{
		self.config.as_deref()
	}

	/// Répertoire de la configuration de l'application. Cette fonction PEUT
	/// paniquer si aucun répertoire n'est trouvé.
	pub fn config_sudo(&self) -> &path::Path
	{
		self.config.as_deref().expect("Répertoire de la configuration")
	}

	/// Répertoire des variables d'environnement de l'application.
	pub fn env(&self) -> Option<&path::Path>
	{
		self.env.as_deref()
	}

	/// Répertoire des variables d'environnement de l'application. Cette
	/// fonction PEUT paniquer si aucun répertoire n'est trouvé.
	pub fn env_sudo(&self) -> &path::Path
	{
		self.env.as_deref().expect("Répertoire de variables d'environnement")
	}
}

impl KernelSettingsDirectory
{
	/// Remplace le répertoire de configuration actuellement définit par un
	/// nouveau.
	pub fn set_config_directory(&mut self, dir: impl Into<path::PathBuf>)
	{
		self.config.replace(self.root.join(dir.into()));
	}

	/// Remplace le répertoire des variables d'environnement actuellement
	/// définit par un nouveau.
	pub fn set_env_directory(&mut self, dir: impl Into<path::PathBuf>)
	{
		self.env.replace(self.root.join(dir.into()));
	}
}
