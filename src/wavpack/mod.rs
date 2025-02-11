//! WavPack specific items
mod properties;
mod read;

#[cfg(feature = "ape")]
use crate::ape::tag::ApeTag;
#[cfg(feature = "id3v1")]
use crate::id3::v1::tag::ID3v1Tag;

use lofty_attr::LoftyFile;

// Exports
pub use properties::WavPackProperties;

/// A WavPack file
#[derive(LoftyFile, Default)]
#[lofty(read_fn = "read::read_from")]
#[lofty(internal_write_module_do_not_use_anywhere_else)]
pub struct WavPackFile {
	/// An ID3v1 tag
	#[cfg(feature = "id3v1")]
	#[lofty(tag_type = "ID3v1")]
	pub(crate) id3v1_tag: Option<ID3v1Tag>,
	/// An APEv1/v2 tag
	#[cfg(feature = "ape")]
	#[lofty(tag_type = "APE")]
	pub(crate) ape_tag: Option<ApeTag>,
	/// The file's audio properties
	pub(crate) properties: WavPackProperties,
}
