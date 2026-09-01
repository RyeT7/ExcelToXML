use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::application::dto::request::tagmappingdto::TagMappingsDTO;
use crate::application::ports::{
    inbound::mapheadersusecase::MapHeadersUseCase, outbound::sessionrepository::SessionRepository,
};
use crate::domain::entities::tagmapping::{TagMapping, TagMappings};
use crate::domain::enums::requiredtags::Tags;

pub struct MapHeadersService {
    session_repository: Arc<dyn SessionRepository>,
}

impl MapHeadersService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        Self { session_repository }
    }
}

impl MapHeadersUseCase for MapHeadersService {
    fn map_headers(&self, session_id: &str, mappings: &TagMappingsDTO) -> Result<(), String> {
        let mut session = self.session_repository.get(session_id)?;
        let table = session.table.as_ref().ok_or("No table found in session")?;

        let mapping_map = &mappings.tag_mappings;

        for tag in Tags::MAPPABLE {
            // Derived tags are computed at conversion time, so the frontend
            // sends no mapping for them and none is required here.
            if tag.is_derived() {
                continue;
            }

            let mapping = mapping_map.get(tag.as_hierarchical_str()).ok_or(format!(
                "Missing mapping for tag '{}'",
                tag.as_literal_str()
            ))?;

            let has_column = mapping.mapped_column.is_some();
            let has_default = mapping.default_value.is_some();

            if !has_column && !has_default {
                return Err(format!(
                    "Tag '{}' must have either a mapped column or a default value",
                    tag.as_literal_str()
                ));
            }

            if let Some(column) = &mapping.mapped_column {
                if !column.is_empty() && !table.headers().contains(column) {
                    return Err(format!("Column '{}' not found in uploaded file", column));
                }
            }
        }

        // Drop anything sent for a derived tag rather than storing it, so a
        // stale value can never be mistaken for the computed one.
        let derived_tags: HashSet<&str> = Tags::MAPPABLE
            .iter()
            .filter(|tag| tag.is_derived())
            .map(|tag| tag.as_hierarchical_str())
            .collect();

        let domain_mappings: HashMap<String, TagMapping> = mapping_map
            .iter()
            .filter(|(k, _)| !derived_tags.contains(k.as_str()))
            .map(|(k, m)| {
                (
                    k.clone(),
                    TagMapping::new(m.mapped_column.clone(), m.default_value.clone()),
                )
            })
            .collect();

        let domain_mapping_all: TagMappings = TagMappings {
            mappings: domain_mappings,
            invoice_number_column: mappings.invoice_number_column.clone(),
            good_service_identifier_column: mappings.good_service_identifier_column.clone(),
        };

        session.tag_mappings = Some(domain_mapping_all);
        self.session_repository.update(session_id, session)?;

        Ok(())
    }
}
