use lsp_types::ServerCapabilities;

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_capabilities_allow_goto_definition() {
        assert!(server_capabilities().definition_provider.is_some());
    }
}
