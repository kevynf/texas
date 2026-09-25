use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use anyhow::{Context, Result, anyhow};
use texas_core::directory::Directory;

use crate::{tracing::*, update::ReleaseInfo};

// Keep this aligned with GRAMMAR_RELEASE_TAG in the release workflow.
pub const GRAMMAR_RELEASE_TAG: &str = "v0.4.5+1f9f9dd";

fn get_github_api(url: &str) -> Result<String> {
    let user_agent = format!("Texas/{}", texas_core::meta::VERSION);
    let resp = texas_proxy::get_url(url, Some(user_agent.as_str()))?;
    if !resp.status().is_success() {
        return Err(anyhow!("get release info failed {}", resp.text()?));
    }

    Ok(resp.text()?)
}

pub fn find_grammar_release() -> Result<ReleaseInfo> {
    let releases: Vec<ReleaseInfo> = serde_json::from_str(&get_github_api(
        "https://api.github.com/repos/lapce/tree-sitter-grammars/releases?per_page=100",
    ).context("Failed to retrieve releases for tree-sitter-grammars")?)?;

    select_grammar_release(releases).ok_or_else(|| {
        anyhow!("Couldn't find grammar release {GRAMMAR_RELEASE_TAG}")
    })
}

fn select_grammar_release(releases: Vec<ReleaseInfo>) -> Option<ReleaseInfo> {
    releases
        .into_iter()
        .find(|release| release.tag_name == GRAMMAR_RELEASE_TAG)
}

pub fn fetch_grammars(release: &ReleaseInfo) -> Result<bool> {
    let dir = Directory::grammars_directory()
        .ok_or_else(|| anyhow!("can't get grammars directory"))?;

    let file_name = format!("grammars-{}-{}", env::consts::OS, env::consts::ARCH);

    let updated = download_release(dir, release, &file_name)?;

    trace!(TraceLevel::INFO, "Successfully downloaded grammars");

    Ok(updated)
}

pub fn fetch_queries(release: &ReleaseInfo) -> Result<bool> {
    let dir = Directory::queries_directory()
        .ok_or_else(|| anyhow!("can't get queries directory"))?;

    let file_name = "queries";

    let updated = download_release(dir, release, file_name)?;

    trace!(TraceLevel::INFO, "Successfully downloaded queries");

    Ok(updated)
}

fn download_release(
    dir: PathBuf,
    release: &ReleaseInfo,
    file_name: &str,
) -> Result<bool> {
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    let current_version =
        fs::read_to_string(dir.join("version")).unwrap_or_default();
    let release_version = if release.tag_name == "nightly" {
        format!("nightly-{}", &release.target_commitish[..7])
    } else {
        release.tag_name.clone()
    };

    if release_version == current_version {
        return Ok(false);
    }

    for asset in &release.assets {
        if asset.name.starts_with(file_name) {
            let mut resp = texas_proxy::get_url(&asset.browser_download_url, None)?;
            if !resp.status().is_success() {
                return Err(anyhow!("download file error {}", resp.text()?));
            }

            let file = tempfile::tempfile()?;

            {
                use std::io::{Seek, Write};
                let file = &mut &file;
                resp.copy_to(file)?;
                file.flush()?;
                file.rewind()?;
            }

            if asset.name.ends_with(".zip") {
                let mut archive = zip::ZipArchive::new(file)?;
                archive.extract(&dir)?;
            } else if asset.name.ends_with(".tar.zst") {
                let mut archive =
                    tar::Archive::new(zstd::stream::read::Decoder::new(file)?);
                archive.unpack(&dir)?;
            }

            fs::write(dir.join("version"), &release_version)?;
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{GRAMMAR_RELEASE_TAG, select_grammar_release};
    use crate::update::ReleaseInfo;

    fn release(tag_name: &str) -> ReleaseInfo {
        ReleaseInfo {
            tag_name: tag_name.to_owned(),
            target_commitish: String::new(),
            assets: Vec::new(),
            version: String::new(),
        }
    }

    #[test]
    fn grammar_release_is_selected_independently_of_app_version() {
        let selected = select_grammar_release(vec![
            release("v0.1.0"),
            release(GRAMMAR_RELEASE_TAG),
            release("nightly"),
        ])
        .unwrap();

        assert_eq!(selected.tag_name, GRAMMAR_RELEASE_TAG);
    }
}
