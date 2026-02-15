Lets change Fetch from being a weird struct to just being a fetch fn that takes a URL. - malformed URL errors now fall outside of fetch.

Manifest will now have a SINGLE method called load that takes a URL - handle fetch errors by wrapping a ManifestError variant (Fetch)

Move error.rs back into level.rs

We want a LevelMeshLoadError which wraps with GLTF, URL join error, fetch error

Model.rs should also be done inline in level.rs - with ModelUpload being an error wrapper in LevelLoadError. 

Lets rename texture.rs to material.rs - call it MaterialData, and make the build_texture_data an impl fn called load.

We expect 2 additional errors now - URL join issues and fetch

To be clear we're no-longer pre-loading all assets (we can get rid of the assets impl on the manifest).

We can probably inline collider.rs too


