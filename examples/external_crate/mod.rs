/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use lexa_kernel::{
	ApplicationAdapterInterface,
	ApplicationStartupExtension,
	AsyncApplicationStartupExtension,
};

pub struct AnyApplicationAdapter<E = ()>
{
	pub env: Option<E>,
}

impl<E> ApplicationAdapterInterface for AnyApplicationAdapter<E>
{
	type Settings = ();

	fn new(_: Self::Settings) -> Self
	{
		Self { env: None }
	}
}

impl<E> ApplicationStartupExtension for AnyApplicationAdapter<E>
{
	fn run(self)
	{
		println!("Sync AnyApplicationAdapter");
	}
}

impl<E> AsyncApplicationStartupExtension for AnyApplicationAdapter<E>
{
	async fn run(self)
	{
		println!("Async AnyApplicationAdapter");
	}
}
