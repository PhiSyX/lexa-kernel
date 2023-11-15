/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

mod external_crate;

use external_crate::AnyApplicationAdapter;
use lexa_kernel::{ApplicationLoggerExtension, ApplicationStartupExtension};

// -------- //
// Constant //
// -------- //

const APPLICATION_NAME: &'static str = "lexa-app";
const APPLICATION_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPLICATION_ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

// ---- //
// Main //
// ---- //

fn main()
{
	type Application = lexa_kernel::Kernel<AnyApplicationAdapter>;

	let application = Application::new(
		APPLICATION_NAME,
		APPLICATION_VERSION,
		APPLICATION_ROOT_DIR,
	)
		// NOTE: VOIR le fichier `examples/config/logger.yml`
		.define_config_directory("examples/config")
		// NOTE: Initialise le logger avec les paramètres du fichier de
		//       configuration ci-haut.
		.initialize_logger()
	;

	log::info!("My Best Logger");

	application.run();
}
