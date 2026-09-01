use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rust_decimal::Decimal;

use crate::{
    application::ports::{
        inbound::convertusecase::ConvertUseCase,
        outbound::{sessionrepository::SessionRepository, xmlwriter::XMLWriter},
    },
    domain::{
        datastructures::table::Table,
        entities::tagmapping::{TagMapping, TagMappings},
        enums::{requiredtags::Tags, xmlattributes::XMLAttributes},
        services::{dates, formula},
    },
};

pub struct ConvertService {
    session_repository: Arc<dyn SessionRepository>,
    xml_writer: Arc<Mutex<dyn XMLWriter>>,
}

impl ConvertService {
    pub fn new(
        session_repository: Arc<dyn SessionRepository>,
        xml_writer: Arc<Mutex<dyn XMLWriter>>,
    ) -> Self {
        Self {
            session_repository: session_repository,
            xml_writer: xml_writer,
        }
    }

    fn write_no_attributes_open_close_tag(
        &mut self,
        tag: &Tags,
        content: Option<&str>,
    ) -> Result<(), String> {
        self.xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
            .new_open_close_tag(tag.as_literal_str(), &[], content);

        Ok(())
    }

    /// Resolves the single value a mapped tag takes for one invoice or one
    /// good/service, from its mapped column or from its default value.
    ///
    /// Derived tags have no mapping and must be computed instead.
    fn resolve_value<'a>(
        tag: &Tags,
        mapping: &'a HashMap<String, TagMapping>,
        table: &'a Table,
    ) -> Result<&'a str, String> {
        let column_or_value = mapping.get(tag.as_hierarchical_str()).ok_or(format!(
            "No mapping found for tag '{}'",
            tag.as_literal_str()
        ))?;

        if let Some(c) = &column_or_value.mapped_column {
            table.get_first(c)
        } else if let Some(v) = &column_or_value.default_value {
            Ok(v.as_str())
        } else {
            Err(format!(
                "Tag '{}' must have either a mapped column or a default value",
                tag.as_literal_str()
            ))
        }
    }

    /// Reads a mapped tag as an amount, for use in a derived tag's formula.
    ///
    /// [`Self::validate_cells`] has already vetted every cell by this point,
    /// so reaching the error arm means the two have drifted apart.
    fn resolve_amount(
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        table: &Table,
    ) -> Result<Decimal, String> {
        let raw = Self::resolve_value(tag, mapping, table)?;

        match formula::parse_amount(raw) {
            Ok(amount) => Ok(amount),
            Err(formula::AmountError::Blank) if tag.blank_means_zero() => Ok(Decimal::ZERO),
            Err(_) => Err(format!(
                "Cannot compute from '{}': '{}' is not a number",
                tag.as_literal_str(),
                raw
            )),
        }
    }

    /// Reads a mapped tag as a date, in the canonical ISO form.
    fn resolve_date(
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        table: &Table,
    ) -> Result<String, String> {
        let raw = Self::resolve_value(tag, mapping, table)?;

        dates::normalize(raw).ok_or(format!(
            "Cannot write '{}': '{}' is not a date",
            tag.as_literal_str(),
            raw
        ))
    }

    /// Describes what is wrong with a cell, or `None` if it is usable.
    fn cell_problem(tag: &Tags, value: &str) -> Option<String> {
        if tag.is_numeric() {
            return Self::amount_problem(tag, value);
        }

        if tag.is_date() {
            return Self::date_problem(value);
        }

        None
    }

    /// Anything reaching here that is not already ISO was text Excel never
    /// recognised as a date, so its day/month order is genuinely unknown.
    fn date_problem(value: &str) -> Option<String> {
        if value.trim().is_empty() {
            return Some("is empty — an invoice needs a date".to_string());
        }

        match dates::parse_iso(value) {
            Some(_) => None,
            None => Some(
                "is not a date — Excel did not store it as one, so which part is the month \
                 cannot be known; format the cell as a date in Excel, or type it as YYYY-MM-DD"
                    .to_string(),
            ),
        }
    }

    /// Describes what is wrong with a cell a formula needs, or `None` if it is
    /// usable.
    fn amount_problem(tag: &Tags, value: &str) -> Option<String> {
        match formula::parse_amount(value) {
            Ok(_) => None,
            Err(formula::AmountError::Blank) => match tag.blank_means_zero() {
                true => None,
                false => Some("is empty — enter 0 if the line really has none".to_string()),
            },
            Err(formula::AmountError::NotANumber) => Some(match formula::amount_hint(value) {
                Some(hint) => format!("is not a number — {hint}"),
                None => "is not a number".to_string(),
            }),
        }
    }

    /// Checks every cell that has to hold a particular kind of value, before
    /// any XML is written.
    ///
    /// Conversion could just fail on the first unusable cell, but a spreadsheet
    /// is corrected as a whole: this walks the file once and reports every bad
    /// cell together, so fixing them is one pass rather than one re-upload per
    /// mistake. The table is still ungrouped here, so each problem can be
    /// pinned to its original row.
    fn validate_cells(table: &Table, mapping: &TagMappings) -> Result<(), String> {
        const MAX_REPORTED: usize = 20;

        // Used only to locate a bad cell, so a missing column is not fatal here
        // — grouping reports that properly.
        let invoice_numbers = table.column(&mapping.invoice_number_column).ok();

        let mut problems: Vec<String> = Vec::new();

        for tag in Tags::MAPPABLE
            .iter()
            .filter(|tag| tag.is_numeric() || tag.is_date())
        {
            let entry = match mapping.mappings.get(tag.as_hierarchical_str()) {
                Some(entry) => entry,
                // A missing mapping is reported by resolve_value, with the
                // wording that case deserves.
                None => continue,
            };

            let column = match &entry.mapped_column {
                Some(column) => column,
                None => {
                    let value = entry.default_value.as_deref().unwrap_or("");

                    if let Some(problem) = Self::cell_problem(tag, value) {
                        problems.push(format!(
                            "{}, default value \"{}\": {}",
                            tag.as_literal_str(),
                            value,
                            problem
                        ));
                    }

                    continue;
                }
            };

            for (index, value) in table.column(column)?.iter().enumerate() {
                let problem = match Self::cell_problem(tag, value) {
                    Some(problem) => problem,
                    None => continue,
                };

                // The header occupies the first row of the sheet, so the first
                // data row is row 2.
                let mut location = format!(
                    "{} (column \"{}\"), row {}",
                    tag.as_literal_str(),
                    column,
                    index + 2
                );

                if let Some(invoice) = invoice_numbers.and_then(|numbers| numbers.get(index)) {
                    location.push_str(&format!(", invoice \"{invoice}\""));
                }

                problems.push(format!("{}: \"{}\" {}", location, value, problem));
            }
        }

        if problems.is_empty() {
            return Ok(());
        }

        let total = problems.len();

        let mut message = format!(
            "{} cell{} in the file cannot be used:",
            total,
            match total == 1 {
                true => "",
                false => "s",
            }
        );

        for problem in problems.iter().take(MAX_REPORTED) {
            message.push_str(&format!("\n  • {problem}"));
        }

        if total > MAX_REPORTED {
            message.push_str(&format!("\n  … and {} more", total - MAX_REPORTED));
        }

        message.push_str("\n\nCorrect these cells in the file, then upload it again.");

        Err(message)
    }

    fn write_no_attributes_open_close_tag_from_invoice(
        &mut self,
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        invoice: &Table,
    ) -> Result<(), String> {
        let content = Self::resolve_value(tag, mapping, invoice)?.to_string();

        self.write_no_attributes_open_close_tag(tag, Some(&content))
    }

    fn write_good_service_detail(
        &mut self,
        tag: &Tags,
        mapping: &HashMap<String, TagMapping>,
        invoice: &Table,
    ) -> Result<(), String> {
        let column_or_value = mapping.get(tag.as_hierarchical_str()).ok_or(format!(
            "No mapping found for tag '{}'",
            tag.as_literal_str()
        ))?;

        if let Some(c) = &column_or_value.mapped_column {
            for content in invoice.column(c)? {
                self.write_no_attributes_open_close_tag(tag, Some(content))?;
            }
        } else if let Some(v) = &column_or_value.default_value {
            self.write_no_attributes_open_close_tag(tag, Some(v))?;
        } else {
            return Err(format!(
                "Tag '{}' must have either a mapped column or a default value",
                tag.as_literal_str()
            ));
        }

        Ok(())
    }

    fn write_good_service_tags(
        &mut self,
        mapping: &TagMappings,
        invoice: &Table,
    ) -> Result<(), String> {
        // Captured before grouping so it can be reported if the identifier
        // turns out to be non-unique.
        let invoice_number = invoice
            .get_first(&mapping.invoice_number_column)?
            .to_string();

        let good_services =
            invoice.group_by(&[mapping.good_service_identifier_column.to_string()])?;

        for good_service in good_services {
            // Each good/service must map to exactly one row. If grouping by the
            // identifier collapses several rows together, the identifier is not
            // unique within this invoice and the generated <GoodService> would
            // contain repeated child tags (a non-atomic result), so reject it.
            let row_count = good_service
                .column(&mapping.good_service_identifier_column)?
                .len();

            if row_count > 1 {
                let identifier_value =
                    good_service.get_first(&mapping.good_service_identifier_column)?;

                return Err(format!(
                    "Non-unique good/service identifier in invoice '{}': column '{}' value '{}' \
                     appears in {} rows. Each good/service must be uniquely identified within an \
                     invoice. Fix the data or choose a different identifier column, then convert again.",
                    invoice_number,
                    mapping.good_service_identifier_column,
                    identifier_value,
                    row_count
                ));
            }

            // Derived amounts, computed before anything is written so the
            // whole good/service is rejected on bad input rather than half
            // emitted. Each one is fed the rounded value of the one before it,
            // so the amounts in the document agree with each other.
            let price = Self::resolve_amount(&Tags::Price, &mapping.mappings, &good_service)?;
            let qty = Self::resolve_amount(&Tags::Qty, &mapping.mappings, &good_service)?;
            let vat_rate = Self::resolve_amount(&Tags::VATRate, &mapping.mappings, &good_service)?;
            let stlg_rate =
                Self::resolve_amount(&Tags::STLGRate, &mapping.mappings, &good_service)?;

            let tax_base = formula::tax_base(price, qty);
            let other_tax_base = formula::other_tax_base(tax_base);
            let vat = formula::vat(other_tax_base, vat_rate);
            let stlg = formula::stlg(other_tax_base, stlg_rate);

            self.xml_writer
                .lock()
                .map_err(|e| {
                    format!(
                        "Failed to acquire XML
                    Writer lock: {e}"
                    )
                })?
                .new_open_tag(Tags::GoodService.as_literal_str(), &[], None);

            self.write_good_service_detail(&Tags::Opt, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::Code, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::Name, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::Unit, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::Price, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::Qty, &mapping.mappings, &good_service)?;

            self.write_good_service_detail(&Tags::TotalDiscount, &mapping.mappings, &good_service)?;

            self.write_no_attributes_open_close_tag(
                &Tags::TaxBase,
                Some(&formula::format_amount(tax_base)),
            )?;

            self.write_no_attributes_open_close_tag(
                &Tags::OtherTaxBase,
                Some(&formula::format_amount(other_tax_base)),
            )?;

            self.write_good_service_detail(&Tags::VATRate, &mapping.mappings, &good_service)?;

            self.write_no_attributes_open_close_tag(
                &Tags::VAT,
                Some(&formula::format_amount(vat)),
            )?;

            self.write_good_service_detail(&Tags::STLGRate, &mapping.mappings, &good_service)?;

            self.write_no_attributes_open_close_tag(
                &Tags::STLG,
                Some(&formula::format_amount(stlg)),
            )?;

            self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
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

        // Invalidate any previously generated XML up front: if this conversion
        // fails (e.g. a non-unique good/service identifier), the stale document
        // must not remain downloadable.
        {
            let mut session = self.session_repository.get(session_id)?;
            session.xml = None;
            self.session_repository.update(session_id, session)?;
        }

        let table = self.session_repository.get_table(session_id)?;

        let mapping = self.session_repository.get_tag_mappings(session_id)?;

        let mapping_map = &mapping.mappings;

        // Vet the whole file up front so every unusable cell is reported at
        // once, while rows can still be identified by their position.
        Self::validate_cells(&table, &mapping)?;

        let invoices = table.group_by(&[mapping.invoice_number_column.to_string()])?;

        self.xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
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
                    },
                ],
                None,
            );

        self.write_no_attributes_open_close_tag(&Tags::TIN, Some(tin))?;

        let seller_idtku = formula::idtku(tin);

        self.xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
            .new_open_tag(Tags::ListOfTaxInvoice.as_literal_str(), &[], None);

        for invoice in invoices {
            // <TaxInvoice>
            self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
                .new_open_tag(Tags::TaxInvoice.as_literal_str(), &[], None);

            // <TaxInvoiceDate></TaxInvoiceDate>, always ISO. A value that came
            // from a real Excel date cell is already in this form; one typed as
            // text is re-rendered so "2026-4-7" is padded to "2026-04-07".
            let tax_invoice_date =
                Self::resolve_date(&Tags::TaxInvoiceDate, mapping_map, &invoice)?;

            self.write_no_attributes_open_close_tag(
                &Tags::TaxInvoiceDate,
                Some(&tax_invoice_date),
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

            // <SellerIDTKU></SellerIDTKU>, derived from the seller's TIN and
            // so the same on every invoice in the document.
            self.write_no_attributes_open_close_tag(&Tags::SellerIDTKU, Some(&seller_idtku))?;

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

            // <BuyerIDTKU></BuyerIDTKU>, derived from the buyer's TIN.
            let buyer_idtku =
                formula::idtku(Self::resolve_value(&Tags::BuyerTin, mapping_map, &invoice)?);

            self.write_no_attributes_open_close_tag(&Tags::BuyerIDTKU, Some(&buyer_idtku))?;

            // <ListOfGoodService>
            self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
                .new_open_tag(Tags::ListOfGoodService.as_literal_str(), &[], None);

            self.write_good_service_tags(&mapping, &invoice)?;

            // </ListOfGoodService>
            self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
                .close_current_tag()?;

            // </TaxInvoice>
            self.xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
                .close_current_tag()?;
        }

        // <ListOfTaxInvoice> and <TaxInvoiceBulk> are still open from the top
        // of the document; close them before extracting the finished XML.
        {
            let mut writer = self
                .xml_writer
                .lock()
                .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?;
            writer.close_current_tag()?; // </ListOfTaxInvoice>
            writer.close_current_tag()?; // </TaxInvoiceBulk>
        }

        let xml = self
            .xml_writer
            .lock()
            .map_err(|e| format!("Failed to acquire XML Writer lock: {e}"))?
            .take_xml();

        let mut session = self.session_repository.get(session_id)?;
        session.xml = Some(xml);
        self.session_repository.update(session_id, session)?;

        Ok(())
    }
}
