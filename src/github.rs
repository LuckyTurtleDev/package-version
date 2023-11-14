use crate::Source;
use attohttpc::get;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Release {
	draft: bool,
	prerelease: bool,
	tag_name: String
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GithubRelease {
	repo: String,
	#[serde(default)]
	allow_prerelease: bool
}

impl Source for GithubRelease {
	fn get_tags(&self) -> anyhow::Result<Vec<crate::Tag>> {
		println!("-> get tags from github releases");
		let releases: Vec<Release> = get(format!(
			"https://api.github.com/repos/{}/releases",
			self.repo
		))
		.send()?
		.error_for_status()?
		.json()?;
		Ok(releases
			.into_iter()
			.filter_map(|f| {
				if f.draft || (f.prerelease && !self.allow_prerelease) {
					None
				} else {
					Some(crate::Tag {
						version: f.tag_name,
						prereleases: Some(f.prerelease),
						yanked: None
					})
				}
			})
			.collect())
	}
	fn name(&self) -> &str {
		&self.repo
	}
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GithubTag {
	repo: String
}

#[derive(Debug, Default, Deserialize)]
struct GitTag {
	name: String
}

impl Source for GithubTag {
	fn get_tags(&self) -> anyhow::Result<Vec<crate::Tag>> {
		println!("-> get tags from github tags");
		let tags: Vec<GitTag> =
			get(format!("https://api.github.com/repos/{}/tags", self.repo))
				.send()?
				.error_for_status()?
				.json()?;
		Ok(tags
			.into_iter()
			.map(|f| crate::Tag {
				version: f.name,
				prereleases: None,
				yanked: None
			})
			.collect())
	}

	fn name(&self) -> &str {
		&self.repo
	}
}
