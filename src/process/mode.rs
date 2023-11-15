// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// ----------- //
// Énumération //
// ----------- //

/// Les modes dans lesquels le programme PEUT s'exécuter.
///
/// Les modes sont utilisés pour:
///
///   1. choisir le fichier d'environnement à utiliser ;
///   2. le système de log ;
///   3. des informations renvoyées aux clients concernant des
///      messages/comportement du programme spécifique à certains modes ;
///   4. ...
///
/// La valeur est définie grâce à la variable d'environnement `PROCESS_ENV`. La
/// valeur par défaut de cette énumération est
/// [LOCAL](ProcessMode::LOCAL).
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub enum ProcessMode
{
	/// Programme lancé en local.
	#[default]
	LOCAL,
	/// Programme lancé en mode développement.
	DEVELOPMENT,
	/// Programme lancé en mode production.
	PRODUCTION,
	/// Programme lancé en mode test.
	TEST,
}

// -------------- //
// Implémentation //
// -------------- //

impl ProcessMode
{
	/// Construit la structure à partir de la configuration Rust.
	pub fn from_rustcfg() -> Self
	{
		if cfg!(test) {
			Self::TEST
		} else if cfg!(debug_assertions) && option_env!("DOCKER").is_some() {
			Self::DEVELOPMENT
		} else if cfg!(debug_assertions) {
			Self::LOCAL
		} else {
			Self::PRODUCTION
		}
	}
}
