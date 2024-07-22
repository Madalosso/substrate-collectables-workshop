#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Learn about Macros used in the `polkadot-sdk`, making pallet development easier.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Learn about the Pallet struct: the structure on which we implement all functions and traits
	// for the Pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Learn about frame_system, and `Config`.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Learn about storage value.
	#[pallet::storage]
	pub(super) type CountForHellos<T: Config> = StorageValue<Value = u64>;

	/// Learn about storage maps.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = ()>;

	// Learn about events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Hello { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		CannotSayHello,
	}

	// Learn about callable functions and dispatch.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn hello_world(origin: OriginFor<T>) -> DispatchResult {
			// Learn about `origin`.
			let who = ensure_signed(origin)?;
			Self::say_hello(who)?;
			Ok(())
		}
	}

	// Learn about internal functions.
	impl<T: Config> Pallet<T> {
		// Learn about `AccountId`.
		fn say_hello(who: T::AccountId) -> DispatchResult {
			let current_count = CountForHellos::<T>::get().unwrap_or(0);
			/* TODO: Update this logic to use safe math. */
			let new_count = current_count.checked_add(1).ok_or(Error::<T>::CannotSayHello)?;
			CountForHellos::<T>::set(Some(new_count));
			Self::deposit_event(Event::<T>::Hello { who });
			Ok(())
		}
	}
}
