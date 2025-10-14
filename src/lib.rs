use zed_extension_api as zed;

struct QuartoExtension;

impl zed::Extension for QuartoExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(QuartoExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed::Extension;

    #[test]
    fn test_extension_can_be_created() {
        // Verify the extension can be instantiated
        let _extension = QuartoExtension::new();
        // Extension creation should succeed without panicking
    }
}
