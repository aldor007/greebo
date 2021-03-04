use serde::Deserialize;
use serde::Deserializer;

pub trait Hashable {
    fn hash(&self) -> String;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clicks {
    pub user_agent: String,
    tracked_by: String,

    #[serde(default)]
    pub geo: Geo,
    time: Time,
    #[serde(default)]
    element: Element,
    keen: Keen,
    tech: Tech,
    url: Url,
    referrer: Referrer,
    local_time_full: String,
    pub ip_address: String,
    page: Page,
    user: User,
    #[serde(skip_deserializing, skip_serializing)]
    pub hash: String,
}

impl Hashable for Clicks {
    fn hash(&self) -> String {
        self.hash.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pageviews {
    pub user_agent: String,
    tracked_by: String,
    referrer: Referrer,
    #[serde(default)]
    pub geo: Geo,
    keen: Keen,
    #[serde(default)]
    tech: Tech,
    url: Url,
    time: Time,
    local_time_full: String,
    pub ip_address: String,
    page: Page,
    user: User,
    #[serde(skip_deserializing, skip_serializing)]
    pub hash: String,
}

impl Hashable for Pageviews {
    fn hash(&self) -> String {
        self.hash.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    #[serde(default)]
    pub province: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub coordinates: Vec<f64>,
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub continent: String,
}

impl Default for Geo {
    fn default() -> Self {
        Geo {
            province: "".to_string(),
            city: "".to_string(),
            country: "".to_string(),
            coordinates: vec![0., 0.],
            postal_code: "".to_string(),
            country_code: "".to_string(),
            continent: "".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Time {
    #[serde(default)]
    utc: Utc,
    #[serde(default)]
    local: Local,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Utc {
    #[serde(default)]
    millisecond: f64,
    #[serde(default)]
    day_of_week_string: String,
    #[serde(default)]
    hour: f64,
    #[serde(default)]
    timezone_offset: f64,
    #[serde(default)]
    day_of_month: f64,
    #[serde(default)]
    day_of_week: f64,
    #[serde(default)]
    day_of_year: f64,
    #[serde(default)]
    second: f64,
    #[serde(default)]
    week: f64,
    #[serde(default)]
    year: f64,
    #[serde(default)]
    month: f64,
    #[serde(default)]
    minute: f64,
    #[serde(default)]
    quarter_of_year: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Local {
    #[serde(default)]
    millisecond: f64,
    #[serde(default)]
    day_of_week_string: String,
    #[serde(default)]
    hour: f64,
    #[serde(default)]
    timezone_offset: f64,
    #[serde(default)]
    day_of_month: f64,
    #[serde(default)]
    day_of_week: f64,
    #[serde(default)]
    day_of_year: f64,
    #[serde(default)]
    second: f64,
    #[serde(default)]
    week: f64,
    #[serde(default)]
    year: f64,
    #[serde(default)]
    month: f64,
    #[serde(default)]
    minute: f64,
    #[serde(default)]
    quarter_of_year: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Element {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    name: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    title: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    text: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    node_name: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    href: String,
    #[serde(default)]
    #[serde(deserialize_with = "parse_number_or_null")]
    x_position: f64,
    #[serde(default)]
    #[serde(deserialize_with = "parse_number_or_null")]
    y_position: f64,
    #[serde(default)]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[cfg(feature = "string-null-none")]
    id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    selector: String,
    #[cfg(feature = "string-null-none")]
    #[serde(default)]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(rename = "type")]
    type_field: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "parse_string_or_null")]
    #[serde(default)]
    class: String,
}

fn parse_number_or_null<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(0.0))
}

fn parse_string_or_null<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Keen {
    #[serde(default)]
    timestamp: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tech {
    #[serde(default)]
    device: Device,
    #[serde(default)]
    profile: Profile,
    #[serde(default)]
    os: Os,
    #[serde(default)]
    browser: Browser,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Device {
    family: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Profile {
    cookies: bool,
    name: String,
    language: String,
    screen: Screen,
    platform: String,
    window: Window,
    version: String,
    online: bool,
    useragent: String,
    #[serde(rename = "codeName")]
    code_name: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Screen {
    orientation: Orientation,
    width: f64,
    #[serde(rename = "availHeight")]
    avail_height: f64,
    height: f64,
    #[serde(rename = "availWidth")]
    avail_width: f64,
    #[serde(rename = "colorDepth")]
    color_depth: f64,
    #[serde(rename = "pixelDepth")]
    pixel_depth: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Orientation {
    #[serde(rename = "type")]
    type_field: String,
    angle: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Window {
    width: f64,
    ratio: Ratio,
    #[serde(rename = "scrollHeight")]
    scroll_height: f64,
    height: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Ratio {
    width: f64,
    height: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Os {
    major: String,
    patch_minor: String,
    minor: String,
    family: String,
    patch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Browser {
    major: String,
    minor: String,
    family: String,
    patch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Url {
    info: Info,
    full: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Info {
    #[serde(default)]
    path: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    protocol: String,
    #[serde(default)]
    anchor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Referrer {
    info: Info,
    full: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Page {
    #[serde(default)]
    scroll_state: ScrollState,
    title: String,
    description: String,
    time_on_page: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ScrollState {
    pixel_max: f64,
    ratio: f64,
    pixel: f64,
    ratio_max: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    uuid: String,
}
