use crate::{Source, Tag};
use attohttpc::get;
use serde::Deserialize;

fn default_tag() -> String {
	"latest".to_string()
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DockerHubImage {
	namespace: String,
	image: String,
	#[serde(default = "default_tag")]
	tag: String
}

#[derive(Debug, Default, Deserialize)]
pub struct Response {
	images: Vec<Image>
}

#[derive(Debug, Default, Deserialize)]
pub struct Image {
	digest: String
}

impl Source for DockerHubImage {
	fn get_tags(&self) -> anyhow::Result<Vec<Tag>> {
		println!("-> get tags from docker_hub");
		let resp: Response = get(format!(
			"https://hub.docker.com/v2/namespaces/{}/repositories/{}/tags/{}",
			if self.namespace == "_" {
				"library"
			} else {
				&self.namespace
			},
			&self.image,
			&self.tag
		))
		.send()?
		.error_for_status()?
		.json()?;
		// I do not think dockerhub has a usefull version.
		// Platform specific version are currently not supported by this crates.
		// For now I simple concentrate the digest of all platform,
		// so the user can check for upadets.
		let mut version = String::new();
		for image in resp.images {
			version += &image.digest;
			version += "-"
		}
		version.pop();
		Ok(vec![Tag {
			version,
			prereleases: None,
			yanked: None
		}])
	}
	fn name(&self) -> &str {
		&self.image //TODO
	}
}
