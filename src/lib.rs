pub use crates_io::CratesIoRelease;
use dockerhub::DockerHubImage;
pub use github::{GithubRelease, GithubTag};
pub use manual::Manual;
use serde::Deserialize;

mod crates_io;
mod dockerhub;
mod github;
mod manual;

#[derive(Clone, Debug, Deserialize)]
pub struct Tag {
	pub version: String,
	/// Is `Some(true)` if the tag is a prereleases.
	/// Is `None` if prereleases not supported by source.
	pub prereleases: Option<bool>,
	/// Is `Some(true)` if the tag, was yanked and should not be used.
	/// Is `None` if yank not supported by source.
	pub yanked: Option<bool>
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Sources {
	CratesIo(CratesIoRelease),
	GithubRelease(GithubRelease),
	GithubTag(GithubTag),
	DockerHub(DockerHubImage),
	Manual(Manual)
}

impl Source for Sources {
	fn get_tags(&self) -> anyhow::Result<Vec<Tag>> {
		{
			match self {
				Self::CratesIo(value) => value.get_tags(),
				Self::GithubRelease(value) => value.get_tags(),
				Self::GithubTag(value) => value.get_tags(),
				Self::DockerHub(value) => value.get_tags(),
				Self::Manual(value) => value.get_tags()
			}
		}
	}
	fn name(&self) -> &str {
		match self {
			Self::CratesIo(value) => value.name(),
			Self::GithubRelease(value) => value.name(),
			Self::GithubTag(value) => value.name(),
			Self::DockerHub(value) => value.name(),
			Self::Manual(value) => value.name()
		}
	}
}

pub trait Source: for<'a> Deserialize<'a> {
	fn get_tags(&self) -> anyhow::Result<Vec<Tag>>;
	fn name(&self) -> &str;
}
