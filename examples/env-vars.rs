/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

mod external_crate;

use external_crate::AnyApplicationAdapter;
use lexa_kernel::{
	ApplicationEnvExtension,
	ApplicationEnvInterface,
	ApplicationStartupExtension,
};

// ---- //
// Type //
// ---- //

type Application = lexa_kernel::Kernel<AnyApplicationAdapter, ApplicationEnv>;

// -------- //
// Constant //
// -------- //

const APPLICATION_NAME: &'static str = "lexa-app";
const APPLICATION_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPLICATION_ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct ApplicationEnv
{
	pub secret_number: i32,
	pub secret_string: String,
}

impl ApplicationEnvInterface for ApplicationEnv
{
	fn vars(settings: &lexa_kernel::settings::KernelSettings) -> Self
	{
		Self::fetch_from_file(settings.directory.env_sudo().join(".env"))
			.expect(
				"\nImpossible de construire la structure ApplicationEnv\n\t",
			)
	}
}

// ---- //
// Main //
// ---- //

fn main()
{

	let application = Application::new(
		APPLICATION_NAME,
		APPLICATION_VERSION,
		APPLICATION_ROOT_DIR,
	)
		// NOTE: Par défaut, le répertoire des fichiers des variables
		//       d'environnement est configuré sur
		// 		 APPLICATION_ROOT_DIR + "/env"
		.define_env_directory("examples/env")
		// NOTE: Récupère les variables d'environnement depuis le fichier
		//       `examples/env/.env`.
		.include_env_vars()
		// NOTE: Récupère les variables d'environnement depuis le fichier
		//       `/path/to/env-file`.
		// .with_env_vars("/path/to/env-file")
	;

	dbg!(application.env());

	application.run();
}
