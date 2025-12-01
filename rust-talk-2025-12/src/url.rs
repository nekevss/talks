use boa_engine::{Context, realm::Realm, JsResult, JsString, JsData, Trace, Finalize};
use boa_engine::{boa_class, boa_module, js_error};
use boa_engine::value::Convert;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
pub struct Url(#[unsafe_ignore_trace]url::Url);

impl Url {
    pub fn register(realm: Option<Realm>, context: &mut Context) -> JsResult<()> {
        js_module::boa_register(realm, context)
    }
}

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new(Convert(ref url): Convert<String>, base: Option<Convert<String>>) -> JsResult<Self> {
        if let Some(Convert(ref base)) = base {
            let base_url = url::Url::parse(base)
                .map_err(|e| js_error!(TypeError: "Failed to parse base URL: {}", e))?;
            if base_url.cannot_be_a_base() {
                return Err(js_error!(TypeError: "Base URL {} cannot be a base", base));
            }

            let url = base_url
                .join(url)
                .map_err(|e| js_error!(TypeError: "Failed to parse URL: {}", e))?;
            Ok(Self(url))
        } else {
            let url = url::Url::parse(url)
                .map_err(|e| js_error!(TypeError: "Failed to parse URL: {}", e))?;
            Ok(Self(url))
        }
    }

    #[boa(getter)]
    fn host(&self) -> JsString {
        JsString::from(url::quirks::host(&self.0))
    }

    #[boa(setter)]
    #[boa(rename = "host")]
    fn set_host(&mut self, value: Convert<String>) {
        let _ = url::quirks::set_host(&mut self.0, &value.0);
    }
}

#[boa_module]
pub mod js_module {
  type Url = super::Url;
}

