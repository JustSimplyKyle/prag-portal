pub mod component;

use std::path::PathBuf;

use dioxus_logger::tracing::info;
use rust_lib::api::{
    backend_exclusive::{errors::ManifestProcessingError, vanilla::version::VersionMetadata},
    shared_resources::{
        collection::{CollectionError, ModLoader, ModLoaderType},
        entry,
    },
};
use snafu::prelude::*;

use crate::get_random_collection_picture;

#[derive(Snafu, Debug)]
pub enum CollectionBuilderError {
    #[snafu(display("Invalid version id {id}"))]
    InvalidVersionId { id: String },
    #[snafu(display("Failed to parse version id {id}"))]
    VersionIdParsing {
        id: String,
        source: ManifestProcessingError,
    },
    #[snafu(transparent)]
    CollectionError { source: CollectionError },
}

/// # Errors
///
/// This function will return an error if:
///
/// - The provided `version_id` is not a valid Minecraft version ID.
/// - Manifest of `version_id` failed to parse
/// - An error occurs while creating the collection entry.
/// - An error occurs while adding mods from Modrinth.
/// - An error occurs while downloading the mods.
pub async fn collection_builder(
    picture_path: impl Into<Option<PathBuf>> + Send,
    version_id: impl Into<String> + Send,
) -> Result<(), CollectionBuilderError> {
    let version_id = version_id.into();
    let version = VersionMetadata::from_id(&version_id)
        .await
        .context(VersionIdParsingSnafu { id: &version_id })?
        .context(InvalidVersionIdSnafu { id: &version_id })?;

    let mut collection = entry::create_collection(
        "新的收藏",
        picture_path
            .into()
            .unwrap_or_else(|| get_random_collection_picture().into()),
        version,
        ModLoader::new(ModLoaderType::Fabric, None),
        None,
    )
    .await?;
    info!("Adding mods...");
    collection
        .add_multiple_modrinth_mod(
            vec![
                "fabric-api",
                "sodium",
                "modmenu",
                "ferrite-core",
                "lazydfu",
                "create-fabric",
                "iris",
                "indium",
            ],
            vec![],
            None,
        )
        .await?;
    collection.download_mods().await?;
    info!("Finished downloading mods");
    Ok(())
}
