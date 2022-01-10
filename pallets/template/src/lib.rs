#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::sp_runtime::traits::{Hash , Zero, AtLeast32BitUnsigned, Bounded, CheckedAdd, One};
    use frame_support::traits::Randomness;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
	use sp_core::H256;
	use sp_io::hashing::blake2_128;
	use codec::{Encode, Decode};

	// Struct for holding Kitty information.
    #[derive(Clone, Encode, Decode, Default, PartialEq)]
    pub struct Kitty(pub [u8;16]);
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The type of Random we want to specify for runtime.
		type Randomness: Randomness<H256, Self::BlockNumber>;

		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

		/// The type of Kitty ID
		type KittyIndex: Parameter + AtLeast32BitUnsigned + Default + Copy + Bounded + CheckedAdd;
	}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
		KittyCreate(T::AccountId, T::KittyIndex),
        KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
    }

	// Stores the total amount of Kitties in existence.
	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

	// Stores a Kitty: it's unique traits and price.
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

	// Keeps track of what accounts own what Kitty.
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub(super) type Owner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

	// Keeps track of what accounts own what Kitty.
	#[pallet::storage]
	#[pallet::getter(fn price_of)]
	pub(super) type Prices<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, [u8; 16])>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { kitties: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (acct, dna) in &self.kitties {
			}
		}
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Nonce has overflowed past u64 limits
		NonceOverflow,
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		KittyNotExist,
		BuyerIsKittyOwner,
		KittyNoPrice,
		KittyPriceTooLow,
		NotEnoughBalance,
	}

    // Storage items.

    // Keeps track of the Nonce used in the randomness generator.
    #[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique kitty.
        ///
        /// Provides the new Kitty details to the 'mint()'
        /// helper function (sender, kitty hash, Kitty struct).
        ///
        /// Calls mint() and increment_nonce().
        ///
        /// Weight: `O(1)`
        #[pallet::weight(100)]
        pub fn create(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// let kitty_id = match Self::kitties_count() {
			// 	Some(id) => {
			// 		ensure!(id != /* T::KittyIndex::max_value() */9999, Error::<T>::KittiesCountOverflow);
			// 		id
			// 	},
			// 	None => {
			// 		1
			// 	}
			// };

			let new_cnt = Self::get_new_kitties_count();

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(new_cnt, Some(Kitty(dna)));

			Owner::<T>::insert(new_cnt, Some(who.clone()));

			KittiesCount::<T>::put(new_cnt);

			Self::deposit_event(Event::KittyCreate(who, new_cnt));
            Ok(().into())
        }

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, new_owner: T::AccountId, kitty_id: T::KittyIndex) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let owner = Self::owner_of(kitty_id).ok_or(<Error<T>>::NotOwner)?;

			// 确保kitty_id的所有者
			ensure!(owner == who, Error::<T>::NotOwner);
			// 确保kitty_id的所有者
			// ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			// 确保kitty_id的所有者不是将要转移过去的账户
			ensure!(owner != new_owner, Error::<T>::NotOwner);

			// 将kitty更新到新的所有者
			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));
			// 发送事件
			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex);
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex);

			let owner1 = Self::owner_of(kitty_id_1).ok_or(<Error<T>>::NotOwner)?;
			let owner2 = Self::owner_of(kitty_id_2).ok_or(<Error<T>>::NotOwner)?;

			// 确保两只kitty的所有者都是发送者
			ensure!(who == owner1, Error::<T>::NotOwner);
			ensure!(who == owner2, Error::<T>::NotOwner);

			/* let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			}; */

			let new_cnt = Self::get_new_kitties_count();

			let dna_1 = kitty1.unwrap().0;
			let dna_2 = kitty2.unwrap().0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];
			for i in 0..dna_1.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}

			Kitties::<T>::insert(new_cnt, Some(Kitty(new_dna)));
			Owner::<T>::insert(new_cnt, Some(who.clone()));
			KittiesCount::<T>::put(new_cnt);

			Ok(().into())
		}

		#[pallet::weight(1000)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex, price: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;
			let kitty = Self::kitties(kitty_id).ok_or(<Error<T>>::KittyNotExist)?;
			let kitty_price = Self::price_of(kitty_id).ok_or(<Error<T>>::KittyNoPrice)?;
			let owner = Self::owner_of(kitty_id).ok_or(<Error<T>>::NotOwner)?;

			ensure!(owner != buyer, Error::<T>::BuyerIsKittyOwner);
			ensure!(kitty_price <= price, Error::<T>::KittyPriceTooLow);

			ensure!(T::Currency::free_balance(&buyer) >= price, <Error<T>>::NotEnoughBalance);

			let seller = owner.clone();
			T::Currency::transfer(&buyer, &seller, price, ExistenceRequirement::KeepAlive)?;

			Owner::<T>::insert(kitty_id, Some(owner.clone()));

			// Self::deposit_event(Event::Bought(buyer, seller, kitty_id, price));

			Ok(().into())
		}
    }

    //** These are all our **//
    //** helper functions. **//
    impl<T: Config> Pallet<T> {

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn get_new_kitties_count() -> T::KittyIndex {
			let new_cnt = match Self::kitties_count() {
				Some(cnt) => {
					cnt.checked_add(&One::one()).ok_or(<Error<T>>::KittiesCountOverflow).unwrap()
				},
				None => One::one(),
			};
			new_cnt
		}
    }
}
