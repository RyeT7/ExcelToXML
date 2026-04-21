use std::{collections::HashMap, ops::ControlFlow};

use crate::{datastructures::table::{Table, TableTrait}, model::mapping::HeaderMapping, parser::tags::Tags, xml::{attributes::XMLAttributes, writer::{XMLWriter, XMLWriterTrait}}};

pub struct Parser<'a> {
    pub table: Table,
    xml_writer: XMLWriter<'a>,
}

pub trait ParserTrait<'a> {
    fn new(
        table: Table,
        xml_writer: XMLWriter<'a>
    ) -> Result<Parser<'a>, String>;
    fn parse(&mut self, mapping: HeaderMapping, tin: &str) -> Result<(), String>;
}

impl Parser<'_> {
    fn map_header(&mut self, mapping: HashMap<String, String>) -> Result<(), String> {
        self.table.rename_key(mapping)
    }

    fn write_no_attributes_open_close_tag(
        &mut self,
        tag: &Tags,
        content: Option<&str>
    ) {
        self.xml_writer.new_open_close_tag(
            tag.as_literal_str(),
            &[],
            content
        );
    }

    fn write_no_attributes_open_close_tag_from_invoice(
        &mut self,
        tag: &Tags,
        invoice: &Table
    ) -> Result<(), String> {
        let content = invoice.get_first(tag.as_literal_str())?;
        self.write_no_attributes_open_close_tag(tag, Some(content));

        Ok(())
    }

    fn write_good_service_detail (
        &mut self,
        tag: &Tags,
        invoice: &Table
    ) -> Result<(), String> {
        let key = tag.as_hierarchical_str();

        let content: Table = invoice.get(key)?;

        for content in content.column(key)? {
            self.write_no_attributes_open_close_tag(
                tag,
                Some(content)
            );
        }

        Ok(())
    }

    fn write_good_service_tags (
        &mut self,
        invoice: &Table
    ) -> Result<(), String> {
        self.xml_writer.new_open_tag(
            Tags::GoodService.as_literal_str(),
            &[],
            None
        );

        self.write_good_service_detail(
            &Tags::Opt,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::Code,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::Name,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::Unit,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::Price,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::Qty,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::TotalDiscount,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::TaxBase,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::OtherTaxBase,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::VATRate,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::VAT,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::STLGRate,
            &invoice
        )?;

        self.write_good_service_detail(
            &Tags::STLG,
            &invoice
        )?;

        self.xml_writer.close_current_tag()?;

        Ok(())
    }
}

impl<'a> ParserTrait<'a> for Parser<'a> {
    fn new(table: Table, xml_writer: XMLWriter<'a>) -> Result<Parser<'a>, String> {
        Ok(Parser {
            table,
            xml_writer,
        })
    }
    
    fn parse(&mut self, mapping: HeaderMapping, tin: &str) -> Result<(), String> {
        self.map_header(mapping.mapping)?;

        let invoices = self.table.group_by(&[mapping.invoice_number_column])?;

        self.xml_writer.new_open_tag(
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
        );

        self.xml_writer.new_open_tag(
            Tags::ListOfTaxInvoice.as_literal_str(),
            &[],
            None
        );
        
        for invoice in invoices {
            // <TaxInvoice>
            self.xml_writer.new_open_tag(
                Tags::TaxInvoice.as_literal_str(),
                &[],
                None
            );

            // <TaxInvoiceDate></TaxInvoiceDate>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TaxInvoiceDate,
                &invoice,
            )?;

            // <TaxInvoiceOpt></TaxInvoiceOpt>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TaxInvoiceOpt,
                &invoice,
            )?;


            // <TrxCode></TrxCode>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::TrxCode,
                &invoice,
            )?;


            // <AddInfo></AddInfo>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::AddInfo,
                &invoice,
            )?;


            // <CustomDoc></CustomDoc>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::CustomDoc,
                &invoice,
            )?;


            // <RefDesc></RefDesc>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::RefDesc,
                &invoice,
            )?;


            // <FacilityStamp></FacilityStamp>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::FacilityStamp,
                &invoice,
            )?;

            // <SellerIDTKU></SellerIDTKU>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::SellerIDTKU,
                &invoice,
            )?;

            // <BuyerTin></BuyerTin>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerTin,
                &invoice,
            )?;

            // <BuyerDocument></BuyerDocument>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerDocument,
                &invoice,
            )?;

            // <BuyerCountry></BuyerCountry>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerCountry,
                &invoice,
            )?;

            // <BuyerDocumentNumber></BuyerDocumentNumber>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerDocumentNumber,
                &invoice,
            )?;

            // <BuyerName></BuyerName>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerName,
                &invoice,
            )?;

            // <BuyerAddress></BuyerAddress>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerAddress,
                &invoice,
            )?;

            // <BuyerEmail></BuyerEmail>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerEmail,
                &invoice,
            )?;

            // <BuyerIDTKU></BuyerIDTKU>
            self.write_no_attributes_open_close_tag_from_invoice(
                &Tags::BuyerIDTKU,
                &invoice,
            )?;

            // <ListOfGoodService>
            self.xml_writer.new_open_tag(
                Tags::ListOfGoodService.as_literal_str(),
                &[],
                None
            );

            // TODO: GoodService tags here
            self.write_good_service_tags(&invoice)?;

            // </ListOfGoodService>
            self.xml_writer.close_current_tag()?;

            // </TaxInvoice>
            self.xml_writer.close_current_tag()?;
        }

        Ok(())
    }
}