// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use crate::{ApplicationCLIInterface, Kernel};

// --------- //
// Interface // -> Extension
// --------- //

/// Extension d'application pour la récupération des arguments de CLI.
///
/// `<UserCLI>` :: une structure de champs nommés. Si une application N'A PAS
/// besoin de ces arguments, ce dernier PEUT également être un tuple vide.
pub trait ApplicationCLIExtension<UserCLI>
	: Sized
where
	UserCLI: ApplicationCLIInterface,
{
	/// Les arguments de CLI de l'application. En supposant qu'ils ont été
	/// définie par la fonction d'implémentation
	/// [ApplicationCLIExtension::include_cli_args()].
	fn cli_args(&self) -> UserCLI;

	/// Inclut les variables d'environnement de l'application à partir d'un
	/// fichier d'environnement. Ce fichier est résolu en fonction du mode
	/// d'exécution.
	fn include_cli_args(self) -> Self;
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<A, E, UserCLI> ApplicationCLIExtension<UserCLI> for Kernel<A, E, UserCLI>
where
	UserCLI: ApplicationCLIInterface,
{
	fn cli_args(&self) -> UserCLI
	{
		self.cli_args.clone().expect(
			"Veuillez appeler la méthode « Kernel#include_cli_args » lors de \
			 l'initialisation de l'application.",
		)
	}

	fn include_cli_args(mut self) -> Self
	{
		let arguments = UserCLI::arguments();
		log::debug!("Arguments de la CLI de l'application « {:#?} »", &arguments);
		self.cli_args.replace(arguments);
		self
	}
}
