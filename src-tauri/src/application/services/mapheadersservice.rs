use std::sync::Arc;

use crate::application::ports::{
    inbound::mapheadersusecase::MapHeadersUseCase,
    outbound::sessionrepository::SessionRepository
};
use crate::application::dto::request::tagmappingdto::TagMappingDTO;
use crate::domain::enums::requiredtags::Tags;
use crate::domain::entities::tagmapping::TagMapping;

pub struct MapHeadersService {
    session_repository: Arc<dyn SessionRepository>,
}

impl MapHeadersService {
    pub fn new (
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            session_repository: session_repository,
        }
    }
}

impl MapHeadersUseCase for MapHeadersService {
    fn map_headers(&self, session_id: &str, mappings: &[TagMappingDTO]) -> Result<(), String> {
        let mut session = self.session_repository.get(session_id)?;
        let table = session.table.as_ref()
            .ok_or("No table found in session")?;

        // Validate all mappable tags are covered
        for tag in Tags::MAPPABLE {
            let mapping = mappings.iter()
                .find(|m| m.literal == tag.as_literal_str())
                .ok_or(format!("Missing mapping for required tag: {}", tag.as_literal_str()))?;

            // Validate each mapping has either a column or default value
            let has_column = mapping.mapped_column.is_some();
            let has_default = mapping.default_value.as_ref()
                .map_or(false, |v| !v.is_empty());

            if !has_column && !has_default {
                return Err(format!(
                    "Tag '{}' must have either a mapped column or a non-empty default value",
                    tag.as_literal_str()
                ));
            }

            // Validate mapped column exists in table
            if let Some(column) = &mapping.mapped_column {
                if !table.headers().contains(column) {
                    return Err(format!(
                        "Column '{}' not found in uploaded file",
                        column
                    ));
                }
            }
        }

        // Store mappings in session (convert DTO to domain entity)
        let domain_mappings: Vec<TagMapping> = mappings
            .iter()
            .map(|m| TagMapping::new(
                m.literal.clone(),
                m.hierarchical.clone(),
                m.mapped_column.clone(),
                m.default_value.clone(),
            ))
            .collect();

        session.tag_mappings = Some(domain_mappings);
        self.session_repository.update(session_id, session)?;

        Ok(())
    }
}