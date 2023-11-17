// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::fmt::Debug;

// --------- //
// Interface //
// --------- //

/// Interface pour la récupération des variables d'environnement.
pub trait ApplicationEnvInterface
	: Sized
	+ Clone
	+ Debug
	+ serde::de::DeserializeOwned
{
	/// Nom du fichier d'environnement. Ce fichier DOIT se trouver dans le
	/// répertoire des fichiers des variables d'environnement.
	const FILENAME: &'static str;

	/// Ajouter un suffixe au fichier d'environnement. Accès aux paramètres
	/// du kernel.
	fn with_suffix(_: &crate::settings::KernelSettings) -> impl ToString
	{
		return String::default();
	}

	/// Initialise la [structure de champs nommés](Self) en dé-sérialisant un
	/// fichier d'environnement.
	fn fetch_from_file(env_filepath: impl AsRef<std::path::Path>)
		-> Result<Self, crate::KernelError>
	where
		Self: serde::de::DeserializeOwned,
	{
		Ok(lexa_env::from_file(env_filepath)?)
	}
}
