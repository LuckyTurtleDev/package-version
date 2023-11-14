use crate::{Source, Tag};
use attohttpc::get;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Versions {
	versions: Vec<Release>
}

#[derive(Debug, Default, Deserialize)]
struct Release {
	/// version version
	num: String,
	yanked: bool
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CratesIoRelease {
	#[serde(rename = "crate")]
	krate: String,
	#[serde(default)]
	allow_yanked: bool
}

impl Source for CratesIoRelease {
	fn get_tags(&self) -> anyhow::Result<Vec<Tag>> {
		println!("-> get tags from crates.io");
		let versions: Versions = get(format!(
			"https://crates.io/api/v1/crates/{}/versions",
			self.krate
		))
		.send()?
		.error_for_status()?
		.json()?;
		Ok(versions
			.versions
			.into_iter()
			.filter_map(|f| {
				if self.allow_yanked && f.yanked {
					None
				} else {
					Some(Tag {
						version: f.num,
						prereleases: None,
						yanked: Some(f.yanked)
					})
				}
			})
			.collect())
	}
	fn name(&self) -> &str {
		&self.krate
	}
}
