/// The maximum number of characters for a node name.
pub(crate) const NODE_NAME_MAX_LENGTH: usize = 64;

/// Generate a valid random name for the node
pub fn generate_node_name() -> String {
	loop {
		let node_name = names::Generator::with_naming(names::Name::Numbered)
			.next()
			.expect("RNG is available on all supported platforms; qed");
		let count = node_name.chars().count();

		if count < NODE_NAME_MAX_LENGTH {
			return node_name
		}
	}
}