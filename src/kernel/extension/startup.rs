// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use console::style;

use crate::Kernel;

// --------- //
// Interface //
// --------- //

/// Interface de lancement d'application.
pub trait ApplicationStartupExtension
	: Sized
{
	/// Démarre l'application.
	fn run(self);
}

/// Interface de lancement d'application asynchrone.
pub trait AsyncApplicationStartupExtension
	: Sized
{
	/// Démarre l'application.
	async fn run(self);
}

// -------------- //
// Implémentation //
// -------------- //

impl<A, E, C> Kernel<A, E, C>
{
	fn display_startup_information(&self)
	{
		println!(
			"Démarrage de l'application {}@v{} en mode {:?}.",
			style(&self.application_name).red(),
			style(&self.application_version).red().underlined(),
			style(&self.settings.process_mode).white().on_cyan(),
		);

		println!();

		println!(
			"\tRépertoire racine de l'application: {:?}",
			self.settings.directory.root()
		);
		println!(
			"\tRépertoire de la configuration: {:?}",
			self.settings.directory.config()
		);
		println!(
			"\tRépertoire des variables d'environnement: {:?}",
			self.settings.directory.env()
		);
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<A, E, C> ApplicationStartupExtension for Kernel<A, E, C>
where
	A: ApplicationStartupExtension,
{
	fn run(self)
	{
		if self.settings.startup_info {
			self.display_startup_information();
		}

		self.application_adapter.run();
	}
}

impl<A, E, C> AsyncApplicationStartupExtension for Kernel<A, E, C>
where
	A: AsyncApplicationStartupExtension,
{
	async fn run(self)
	{
		if self.settings.startup_info {
			self.display_startup_information();
		}

		self.application_adapter.run().await;
	}
}
