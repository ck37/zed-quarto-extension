use zed_extension_api as zed;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(QuartoExtension);
