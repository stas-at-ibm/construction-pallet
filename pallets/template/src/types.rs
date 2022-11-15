use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Project<AccountId, BoundedString> {
	pub id: BoundedString,
	pub boq_id: BoundedString,
	pub title: BoundedString,
	pub location: BoundedString,
	pub construction_manager: AccountId,
}
