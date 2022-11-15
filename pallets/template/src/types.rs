pub struct Project<AccountId, BoundedString> {
	pub id: BoundedString,
	pub boq_id: BoundedString,
	pub title: BoundedString,
	pub location: BoundedString,
	pub construction_manager: AccountId,
}
