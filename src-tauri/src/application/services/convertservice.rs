use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{application::ports::{inbound::convertusecase::ConvertUseCase, outbound::{sessionrepository::SessionRepository, xmlwriter::XMLWriter}}, domain::{datastructures::table::Table, entities::tagmapping::{TagMapping, TagMappings}, enums::{requiredtags::Tags, xmlattributes::XMLAttributes}}, model::mapping};

pub struct ConvertService {
    session_repository: Arc<dyn SessionRepository>,
    xml_writer: Arc<Mutex<dyn XMLWriter>>,
}

impl ConvertService {
    pub fn new(
        session_repository: Arc<dyn SessionRepository>,
        xml_writer: Arc<Mutex<dyn XMLWriter>>
    ) -> Self {
        Self {
            session_repository: session_repository,
            xml_writer: xml_writer,
        }
    }

    fn write_no_attributes_open_close_tag(
        &mut self,
        tag: &Tags,
        content: Option<&str>
    ) -> Result<(), String> {
        self.xml_writer
            .lock()
            .map_err(
                |e| format!("Failed to acquire XML Writer lock: {e}")
            )?
            .new_open_close_tag(
                tag.as_literal_str(),
                &[],
                content
            );

        Ok(())
    }

    fn write_no_attributes_open_close_tag_from_invoice(
        &mut self,
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        invoice: &Table,
    ) -> Result<(), String> {
        let column_or_value = mapping.get(tag.as_hierarchical_str())
            .ok_or(format!("No mapping found for tag '{}'", tag.as_literal_str()))?;

        if let Some(c) = &column_or_value.mapped_column {
            let content = invoice.get_first(c)?;
            self.write_no_attributes_open_close_tag(tag, Some(content))?;
        } else if let Some(v) = &column_or_value.default_value {
            self.write_no_attributes_open_close_tag(
                tag,
                Some(v)
            )?;
        } else {
            return Err(format!("Tag '{}' must have either a mapped column or a default value", tag.as_literal_str()));
        }

        Ok(())
    }

    fn write_good_service_detail (
        &mut self,
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        invoice: &Table,
    ) -> Result<(), String> {
        let column_or_value = mapping.get(tag.as_hierarchical_str())
            .ok_or(format!("No mapping found for tag '{}'", tag.as_literal_str()))?;

        if let Some(c) = &column_or_value.mapped_column {
            for content in invoice.column(c)? {
                self.write_no_attributes_open_close_tag(
                    tag,
                    Some(content)
                )?;
            }
        } else if let Some(v) = &column_or_value.default_value {
            self.write_no_attributes_open_close_tag(
                tag,
                Some(v)
            )?;
        } else {
            return Err(format!("Tag '{}' must have either a mapped column or a default value", tag.as_literal_str()));
        }

        Ok(())
    }

    fn write_good_service_tags (
        &mut self,
        mapping: &TagMappings,
        mut invoice: Table
    ) -> Result<(), String> {
        let good_services = invoice.group_by(&[mapping.good_service_identifier_column.to_string()])?;

        for good_service in good_services {
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML 
                    Writer lock: {e}")
                )?
                .new_open_tag(
                    Tags::GoodService.as_literal_str(),
                    &[],
                    None
                );
    
            self.write_good_service_detail(
                &Tags::Opt,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::Code,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::Name,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::Unit,
                &mapping.mappings,
                 &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::Price,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::Qty,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::TotalDiscount,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::TaxBase,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::OtherTaxBase,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::VATRate,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::VAT,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::STLGRate,
                &mapping.mappings,
                &good_service
            )?;
    
            self.write_good_service_detail(
                &Tags::STLG,
                &mapping.mappings,
                &good_service
            )?;
    
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML Writer lock: {e}")
                )?
                .close_current_tag()?;
    
        }
        
        Ok(())
    }
}

impl ConvertUseCase for ConvertService {
    fn convert(&mut self, session_id: &str, tin: &str) -> Result<(), String> {
        // Discard any state left over from a previous conversion so this
        // document starts from an empty buffer.
        self.xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
            .take_xml();

        let mut table = self.session_repository
            .get_table(session_id)?;

        let mapping = self.session_repository.get_tag_mappings(session_id)?;

        let mapping_map = &mapping.mappings;

        let invoices = table.group_by(&[mapping.invoice_number_column.to_string()])?;

        self.xml_writer
            .lock()
            .map_err(
                |e| format!("Failed to acquire XML Writer lock: {e}")
            )?
            .new_open_tag(
            Tags::TaxInvoiceBulk.as_literal_str(),
            &[
                XMLAttributes {
                    attribute_name: "xmlns:xsd".to_string(),
                    attribute_value: "http://www.w3.org/2001/XMLSchema".to_string(),
                },
                XMLAttributes {
                    attribute_name: "xmlns:xsi".to_string(),
                    attribute_value: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
                }
            ],
            None
        );

        self.write_no_attributes_open_close_tag(
            &Tags::TIN,
            Some(tin)
        )?;

        self.xml_writer
        .lock()
            .map_err(
                |e| format!("Failed to acquire XML Writer lock: {e}")
            )?
            .new_open_tag(
            Tags::ListOfTaxInvoice.as_literal_str(),
            &[],
            None
        );
        
        for invoice in invoices {
            // <TaxInvoice>
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML Writer lock: {e}")
                )?
                .new_open_tag(
                    Tags::TaxInvoice.as_literal_str(),
                    &[],
                    None
                );

            // <TaxInvoiceDate></TaxInvoiceDate>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TaxInvoiceDate,
                mapping_map,
                &invoice,
            )?;

            // <TaxInvoiceOpt></TaxInvoiceOpt>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TaxInvoiceOpt,
                mapping_map,
                &invoice,
            )?;


            // <TrxCode></TrxCode>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TrxCode,
                mapping_map,
                &invoice,
            )?;


            // <AddInfo></AddInfo>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::AddInfo,
                mapping_map,
                &invoice,
            )?;


            // <CustomDoc></CustomDoc>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::CustomDoc,
                mapping_map,
                &invoice,
            )?;


            // <RefDesc></RefDesc>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::RefDesc,
                mapping_map,
                &invoice,
            )?;


            // <FacilityStamp></FacilityStamp>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::FacilityStamp,
                mapping_map,
                &invoice,
            )?;

            // <SellerIDTKU></SellerIDTKU>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::SellerIDTKU,
                mapping_map,
                &invoice,
            )?;

            // <BuyerTin></BuyerTin>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerTin,
                mapping_map,
                &invoice,
            )?;

            // <BuyerDocument></BuyerDocument>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerDocument,
                mapping_map,
                &invoice,
            )?;

            // <BuyerCountry></BuyerCountry>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerCountry,
                mapping_map,
                &invoice,
            )?;

            // <BuyerDocumentNumber></BuyerDocumentNumber>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerDocumentNumber,
                mapping_map,
                &invoice,
            )?;

            // <BuyerName></BuyerName>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerName,
                mapping_map,
                &invoice,
            )?;

            // <BuyerAddress></BuyerAddress>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerAddress,
                mapping_map,
                &invoice,
            )?;

            // <BuyerEmail></BuyerEmail>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerEmail,
                mapping_map,
                &invoice,
            )?;

            // <BuyerIDTKU></BuyerIDTKU>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerIDTKU,
                mapping_map,
                &invoice,
            )?;

            // <ListOfGoodService>
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML Writer lock: {e}")
                )?
                .new_open_tag(
                    Tags::ListOfGoodService.as_literal_str(),
                    &[],
                    None
                );

            self.write_good_service_tags(&mapping, invoice.clone())?;

            // </ListOfGoodService>
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML Writer lock: {e}")
                )?
                .close_current_tag()?;

            // </TaxInvoice>
            self.xml_writer
                .lock()
                .map_err(
                    |e| format!("Failed to acquire XML Writer lock: {e}")
                )?
                .close_current_tag()?;
        }

        // <ListOfTaxInvoice> and <TaxInvoiceBulk> are still open from the top
        // of the document; close them before extracting the finished XML.
        {
            let mut writer = self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?;
            writer.close_current_tag()?; // </ListOfTaxInvoice>
            writer.close_current_tag()?; // </TaxInvoiceBulk>
        }

        let xml = self.xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
            .take_xml();

        let mut session = self.session_repository.get(session_id)?;
        session.xml = Some(xml);
        self.session_repository.update(session_id, session)?;

        Ok(())
    }
}
