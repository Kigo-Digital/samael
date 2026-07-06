//! XML deserialization helpers.
//!
//! quick-xml 0.38 changed the serde `Deserializer` to follow the XML Schema
//! `xs:string` whitespace facet (`preserve`): leading and trailing whitespace
//! in element text is no longer stripped, whereas quick-xml <= 0.37 trimmed it.
//!
//! Every text value SAML carries — X.509 certificates, issuer/audience/recipient
//! URIs, `NameID`s, status codes — is whitespace-insensitive, and both this crate
//! (e.g. issuer/audience/status comparisons in `ServiceProvider` validation) and
//! downstream consumers relied on the pre-0.38 trimmed values. Pretty-printed IdP
//! metadata and responses routinely indent these elements, so preserving the
//! surrounding whitespace would break equality checks.
//!
//! Re-enabling `trim_text` at the reader level restores the pre-0.38 behavior for
//! all string-typed fields without annotating each one individually.

use serde::Deserialize;

/// Deserialize a value from an XML string, trimming leading and trailing
/// whitespace from element text (the quick-xml <= 0.37 default).
///
/// Drop-in replacement for [`quick_xml::de::from_str`].
pub fn from_str<'de, T>(s: &'de str) -> Result<T, quick_xml::DeError>
where
    T: Deserialize<'de>,
{
    let mut reader = quick_xml::NsReader::from_str(s);
    reader.config_mut().trim_text(true);
    let mut deserializer = quick_xml::de::Deserializer::borrowing(reader);
    T::deserialize(&mut deserializer)
}
