// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use crate::kernel::Kernel;
use crate::logger::settings::{LoggerSettings, LoggerSettingsPreset};
use crate::process::ProcessMode;

// --------- //
// Interface // -> Extension
// --------- //

/// Extension de l'Application: Logger.
pub trait ApplicationLoggerExtension
	: Sized
{
	/// Active le logger avec les paramètres récupéré depuis le fichier de
	/// configuration de l'application. Ce fichier est trouvé et résolu
	/// automatiquement grâce aux [paramètres de
	/// l'application](crate::kernel::settings::KernelSettings).
	fn initialize_logger(self) -> Self;

	/// Définit un logger avec ses paramètres.
	fn with_logger(self, settings: impl Into<LoggerSettings>) -> Self;
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<A, E, C> ApplicationLoggerExtension for Kernel<A, E, C>
{
	fn initialize_logger(self) -> Self
	{
		let settings = match self.fetch_config(LoggerSettings::FILENAME) {
			| Ok(logger_settings) => logger_settings,
			| Err(err) => {
				self.logger_signal.send_warning(format!(
					"Le fichier de configuration du logger n'a pas pu être chargé correctement.\nRaison « {err} ». \
					 \nLes paramètres par défaut du logger ont été appliqués.",
				));

				LoggerSettings {
					preset: LoggerSettingsPreset::Default,
					..Default::default()
				}
			}
		};

		self.with_logger(settings)
	}

	fn with_logger(self, settings: impl Into<LoggerSettings>) -> Self
	{
		let settings = settings.into();

		#[cfg(not(feature = "tracing"))]
		let level_based_on_process_mode = match self.settings.process_mode {
			| ProcessMode::LOCAL => lexa_logger::LevelFilter::Debug,
			| ProcessMode::DEVELOPMENT => lexa_logger::LevelFilter::Debug,
			| ProcessMode::PRODUCTION => lexa_logger::LevelFilter::Error,
			| ProcessMode::TEST => lexa_logger::LevelFilter::Trace,
		};

		#[cfg(feature = "tracing")]
		let level_based_on_process_mode = match self.settings.process_mode {
			| ProcessMode::LOCAL => tracing::level_filters::LevelFilter::DEBUG,
			| ProcessMode::DEVELOPMENT => tracing::level_filters::LevelFilter::DEBUG,
			| ProcessMode::PRODUCTION => tracing::level_filters::LevelFilter::ERROR,
			| ProcessMode::TEST => tracing::level_filters::LevelFilter::TRACE,
		};

		if let Err(err) = settings.make_builder(level_based_on_process_mode) {
			self.logger_signal
				.send_error(format!("Erreur liée au logger. Raison « {err} »"));
			return self;
		}

		log::debug!("Paramètres du logger « {settings:#?} »");

		self
	}
}
