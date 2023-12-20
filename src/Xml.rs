use roxmltree::Document;

pub struct Xml {}

impl Xml {
    pub fn extract(tag_name: &str, body: &str) -> Option<String> {
        let doc = Document::parse(body).ok()?;
        let node = doc.descendants().find(|n| n.has_tag_name(tag_name))?;
        Some(node.text()?.to_string())
    }

    pub fn create_login_envelope(username: &str, password: &str) -> String {
        [
            "<se:Envelope xmlns:se='http://schemas.xmlsoap.org/soap/envelope/'>",
            "<se:Header/>",
            "<se:Body>",
            "<login xmlns='urn:partner.soap.sforce.com'>",
            &format!("<username>{}</username>", username),
            &format!("<password>{}</password>", password),
            "</login>",
            "</se:Body>",
            "</se:Envelope>",
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::Xml::Xml;
    use html_escape::decode_html_entities;

    #[tokio::test]
    async fn test_create_envelope() {
        let envelope = Xml::create_login_envelope("u", "p");
        assert_eq!(envelope, "<se:Envelope xmlns:se='http://schemas.xmlsoap.org/soap/envelope/'><se:Header/><se:Body><login xmlns='urn:partner.soap.sforce.com'><username>u</username><password>p</password></login></se:Body></se:Envelope>")
    }

    #[tokio::test]
    async fn test_extra_error_from_body() {
        let response = "&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?&gt;&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns=&quot;urn:partner.soap.sforce.com&quot; xmlns:xsi=&quot;http://www.w3.org/2001/XMLSchema-instance&quot;&gt;&lt;soapenv:Body&gt;&lt;loginResponse&gt;&lt;result&gt;&lt;metadataServerUrl&gt;https://test.salesforce.com/services/Soap/m/21.0/00DDG00000NAWhM&lt;/metadataServerUrl&gt;&lt;passwordExpired&gt;false&lt;/passwordExpired&gt;&lt;sandbox&gt;true&lt;/sandbox&gt;&lt;serverUrl&gt;https://test.salesforce.com/services/Soap/u/21.0/00DDG00000NAWhM&lt;/serverUrl&gt;&lt;sessionId&gt;KamehamehaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA&lt;/sessionId&gt;&lt;userId&gt;005a000000CNVshAAH&lt;/userId&gt;&lt;userInfo&gt;&lt;accessibilityMode&gt;false&lt;/accessibilityMode&gt;&lt;currencySymbol&gt;$&lt;/currencySymbol&gt;&lt;orgAttachmentFileSizeLimit&gt;5242880&lt;/orgAttachmentFileSizeLimit&gt;&lt;orgDefaultCurrencyIsoCode&gt;USD&lt;/orgDefaultCurrencyIsoCode&gt;&lt;orgDisallowHtmlAttachments&gt;false&lt;/orgDisallowHtmlAttachments&gt;&lt;orgHasPersonAccounts&gt;false&lt;/orgHasPersonAccounts&gt;&lt;organizationId&gt;00DDG00000NAWhM2AX&lt;/organizationId&gt;&lt;organizationMultiCurrency&gt;false&lt;/organizationMultiCurrency&gt;&lt;organizationName&gt;AWESOME ORG LLC&lt;/organizationName&gt;&lt;profileId&gt;00ea0000001du5ZAAQ&lt;/profileId&gt;&lt;roleId&gt;00E0c000002TiOnEAK&lt;/roleId&gt;&lt;sessionSecondsValid&gt;28800&lt;/sessionSecondsValid&gt;&lt;userDefaultCurrencyIsoCode xsi:nil=&quot;true&quot;/&gt;&lt;userEmail&gt;goku@carrot.com.invalid&lt;/userEmail&gt;&lt;userFullName&gt;Goku Carrot Cake&lt;/userFullName&gt;&lt;userId&gt;005a000000CNVshAAH&lt;/userId&gt;&lt;userLanguage&gt;en_US&lt;/userLanguage&gt;&lt;userLocale&gt;en_US&lt;/userLocale&gt;&lt;userName&gt;goku@carrot.com.fullsb&lt;/userName&gt;&lt;userTimeZone&gt;America/Los_Angeles&lt;/userTimeZone&gt;&lt;userType&gt;Standard&lt;/userType&gt;&lt;userUiSkin&gt;Theme3&lt;/userUiSkin&gt;&lt;/userInfo&gt;&lt;/result&gt;&lt;/loginResponse&gt;&lt;/soapenv:Body&gt;&lt;/soapenv:Envelope&gt;";
        let decoded_response = decode_html_entities(response).to_string();
        let access_token =
            Xml::extract("sessionId", &decoded_response).unwrap_or_else(|| "".to_string());

        assert_eq!(
            access_token,
            "KamehamehaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        );
    }
}
