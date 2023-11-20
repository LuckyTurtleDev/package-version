use crate::{Source, Tag};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manual {
	version: String
}

impl Source for Manual {
	fn get_tags(&self) -> anyhow::Result<Vec<Tag>> {
		Ok(vec![Tag {
			version: self.version.to_owned(),
			prereleases: None,
			yanked: None
		}])
	}
	fn name(&self) -> &str {
		"TODO!"
	}
}
