//! # xrust-net::resolver
//!
//! Resolve a URL

use std::fs;
use std::path::Path;
use url::Url;
use xrust::xdmerror::{Error, ErrorKind};

pub fn resolve(url: &Url) -> Result<String, Error> {
    match url.scheme() {
        "http" => reqwest::blocking::get(url.to_string())
            .map_err(|_| Error::new(
                ErrorKind::Unknown,
                format!("unable to fetch href URL \"{}\"", url.to_string()),
            ))?
            .text()
            .map_err(|_| Error::new(
                ErrorKind::Unknown,
                "unable to extract module data".to_string(),
            )),
        "file" => fs::read_to_string(Path::new(url.path()))
            .map_err(|er| Error::new(ErrorKind::Unknown, er.to_string())),
        _ => {
            return Result::Err(Error::new(
                ErrorKind::Unknown,
                format!("unable to fetch URL \"{}\"", url.to_string()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;
    use xrust::item::{Node, NodeType, Item, Sequence, SequenceTrait};
    use xrust::evaluate::{Evaluator, StaticContext};
    use xrust::xslt::from_document;
    use xrust::intmuttree::{Document, RNode, NodeBuilder};

    fn make_from_str(s: &str) -> Result<RNode, Error> {
	let e = Document::try_from(s).expect("failed to parse XML").content[0].clone();
	let mut d = NodeBuilder::new(NodeType::Document).build();
	d.push(e).expect("unable to append node");
	Ok(d)
    }

    #[test]
    fn include() {
	let mut sc = StaticContext::new_with_xslt_builtins();

	let src = Rc::new(Item::Node(
	    make_from_str("<Test>one<Level1/>two<Level2/>three<Level3/>four<Level4/></Test>")
		.expect("unable to parse source document")
	));

	let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
  <xsl:include href='included.xsl'/>
  <xsl:template match='child::Test'><xsl:apply-templates/></xsl:template>
  <xsl:template match='child::Level1'>found Level1 element</xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select='.'/></xsl:template>
</xsl:stylesheet>").expect("unable to parse stylesheet");

	// Setup dynamic context with result document
	let pwd = std::env::current_dir().expect("unable to get current directory");
	let pwds = pwd.into_os_string().into_string().expect("unable to convert pwd");
	let ev = from_document(
	    style,
	    &mut sc,
	    Some(Url::parse(format!("file://{}/tests/xsl/including.xsl", pwds.as_str()).as_str()).expect("unable to parse URL")),
	    |s| make_from_str(s),
	    |url| resolve(url),
	)
	    .expect("failed to compile stylesheet");
	println!("Templates:");
	ev.dump_templates();

	let rd = NodeBuilder::new(NodeType::Document).build();

	// Prime the stylesheet evaluation by finding the template for the document root
	// and making the document root the initial context
	let t = ev.find_match(&src, None, &rd)
	    .expect("unable to find match");
	assert!(t.len() >= 1);

	let seq = ev.evaluate(Some(vec![Rc::clone(&src)]), Some(0), &t, &rd)
	    .expect("evaluation failed");

	assert_eq!(seq.to_xml(), "onefound Level1 elementtwofound Level2 elementthreefound Level3 elementfour")
    }

//    #[test]
//    fn import() {
//    }
}
