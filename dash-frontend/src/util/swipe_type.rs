use std::{fs, io, path::PathBuf};

use wlx_common::data_dir;

/// The set of model files required by the swipe-to-type engine.
/// These correspond to the assets that `super-swipe-type` currently
/// downloads via `cached_path` in `SwipeOrchestrator::new()`.
pub struct SwipeTypeModel {
	pub file_name: &'static str,
	pub url: &'static str,
}

pub const SWIPE_TYPE_MODELS: &[SwipeTypeModel] = &[
	SwipeTypeModel {
		file_name: "swipe_encoder_android.onnx",
		url: "https://wayvr.org/files/swipe_type/swipe_encoder_android.onnx",
	},
	SwipeTypeModel {
		file_name: "swipe_decoder_android.onnx",
		url: "https://wayvr.org/files/swipe_type/swipe_decoder_android.onnx",
	},
	SwipeTypeModel {
		file_name: "en_wordlist.fst",
		url: "https://wayvr.org/files/swipe_type/en_wordlist.fst",
	},
	SwipeTypeModel {
		file_name: "en_bigrams.fst",
		url: "https://wayvr.org/files/swipe_type/en_bigrams.fst",
	},
];

pub fn swipe_type_model_folder() -> PathBuf {
	data_dir::get_path("swipe_type")
}

pub fn swipe_type_model_path(file_name: &str) -> PathBuf {
	swipe_type_model_folder().join(file_name)
}

/// Returns true when every required model file is present on disk.
pub fn swipe_type_all_models_downloaded() -> io::Result<bool> {
	let path = swipe_type_model_folder();
	if !path.is_dir() {
		return Ok(false);
	}
	for model in SWIPE_TYPE_MODELS {
		if !path.join(model.file_name).exists() {
			return Ok(false);
		}
	}
	Ok(true)
}

pub fn swipe_type_delete_all_models() -> io::Result<()> {
	let path = swipe_type_model_folder();
	if !path.is_dir() {
		return Ok(());
	}

	for entry in fs::read_dir(path)? {
		let entry = entry?;
		let file_type = entry.file_type()?;

		if file_type.is_file() || file_type.is_symlink() {
			fs::remove_file(entry.path())?;
		}
	}

	Ok(())
}