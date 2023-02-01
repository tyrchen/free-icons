use flate2::bufread::GzDecoder;
use std::{borrow::Cow, collections::HashMap, io::Read};

mod gen;

const MAX_ATTRS: usize = 16;
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IconAttrs<'a> {
    data: [(&'a str, Cow<'a, str>); MAX_ATTRS],
    pos: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconType {
    #[cfg(feature = "bootstrap")]
    Bootstrap(Bootstrap),
    #[cfg(feature = "feather")]
    Feather(Feather),
    #[cfg(feature = "font-awesome")]
    FontAwesome(FontAwesome),
    #[cfg(feature = "heroicons")]
    Heroicons(Heroicons),
    #[cfg(feature = "ionicons")]
    Ionicons(Ionicons),
    #[cfg(feature = "octicons")]
    Octicons(Octicons),
}

#[cfg(feature = "bootstrap")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bootstrap {
    Fill,
    Normal,
}

#[cfg(feature = "feather")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Feather {
    Normal,
}

#[cfg(feature = "font-awesome")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontAwesome {
    Regular,
    Solid,
}

#[cfg(feature = "heroicons")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Heroicons {
    Outline,
    Solid,
}

#[cfg(feature = "ionicons")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ionicons {
    Outline,
    Sharp,
    Normal,
}

#[cfg(feature = "octicons")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Octicons {
    Normal,
}

/// retrieve the SVG from incon_type and name
pub fn get(icon_type: IconType, name: &str) -> Option<&'static String> {
    match icon_type {
        #[cfg(feature = "bootstrap")]
        IconType::Bootstrap(icon_type) => match icon_type {
            Bootstrap::Fill => gen::bootstrap::FILL.get(name),
            Bootstrap::Normal => gen::bootstrap::NORMAL.get(name),
        },
        #[cfg(feature = "feather")]
        IconType::Feather(icon_type) => match icon_type {
            Feather::Normal => gen::feather::NORMAL.get(name),
        },
        #[cfg(feature = "font-awesome")]
        IconType::FontAwesome(icon_type) => match icon_type {
            FontAwesome::Regular => gen::font_awesome::REGULAR.get(name),
            FontAwesome::Solid => gen::font_awesome::SOLID.get(name),
        },
        #[cfg(feature = "heroicons")]
        IconType::Heroicons(icon_type) => match icon_type {
            Heroicons::Outline => gen::heroicons::OUTLINE.get(name),
            Heroicons::Solid => gen::heroicons::SOLID.get(name),
        },
        #[cfg(feature = "ionicons")]
        IconType::Ionicons(icon_type) => match icon_type {
            Ionicons::Outline => gen::ionicons::OUTLINE.get(name),
            Ionicons::Sharp => gen::ionicons::SHARP.get(name),
            Ionicons::Normal => gen::ionicons::NORMAL.get(name),
        },
        #[cfg(feature = "octicons")]
        IconType::Octicons(icon_type) => match icon_type {
            Octicons::Normal => gen::octicons::NORMAL.get(name),
        },
    }
}

#[cfg(feature = "bootstrap")]
#[inline(always)]
pub fn bootstrap(name: &str, filled: bool, attrs: IconAttrs) -> Option<String> {
    let svg = if filled {
        gen::bootstrap::FILL.get(name)
    } else {
        gen::bootstrap::NORMAL.get(name)
    };
    attrs.add_to_svg(svg)
}

#[cfg(feature = "feather")]
#[inline(always)]
pub fn feather(name: &str, attrs: IconAttrs) -> Option<String> {
    let svg = gen::feather::NORMAL.get(name);
    attrs.add_to_svg(svg)
}

#[cfg(feature = "font-awesome")]
#[inline(always)]
pub fn font_awesome(name: &str, category: FontAwesome, attrs: IconAttrs) -> Option<String> {
    let svg = match category {
        FontAwesome::Regular => gen::font_awesome::REGULAR.get(name),
        FontAwesome::Solid => gen::font_awesome::SOLID.get(name),
    };
    attrs.add_to_svg(svg)
}

#[cfg(feature = "heroicons")]
#[inline(always)]
pub fn heroicons(name: &str, outline: bool, attrs: IconAttrs) -> Option<String> {
    let svg = if outline {
        gen::heroicons::OUTLINE.get(name)
    } else {
        gen::heroicons::SOLID.get(name)
    };
    attrs.add_to_svg(svg)
}

#[cfg(feature = "ionicons")]
#[inline(always)]
pub fn ionicons(name: &str, category: Ionicons, attrs: IconAttrs) -> Option<String> {
    let svg = match category {
        Ionicons::Outline => gen::ionicons::OUTLINE.get(name),
        Ionicons::Sharp => gen::ionicons::SHARP.get(name),
        Ionicons::Normal => gen::ionicons::NORMAL.get(name),
    };
    attrs.add_to_svg(svg)
}

#[cfg(feature = "octicons")]
#[inline(always)]
pub fn octicons(name: &str, attrs: IconAttrs) -> Option<String> {
    let svg = gen::octicons::NORMAL.get(name);
    attrs.add_to_svg(svg)
}

pub(crate) fn decap(bytes: &[u8]) -> HashMap<String, HashMap<String, String>> {
    let mut gz = GzDecoder::new(bytes);
    let mut uncompressed = Vec::new();
    gz.read_to_end(&mut uncompressed).expect("should decap");
    let (ret, _) = bincode::decode_from_slice(&uncompressed, bincode::config::standard())
        .expect("should deserialize");
    ret
}

impl<'a> IconAttrs<'a> {
    #[inline(always)]
    pub fn class(self, class: &'a str) -> Self {
        self.with("class", class)
    }

    #[inline(always)]
    pub fn fill(self, fill: &'a str) -> Self {
        self.with("fill", fill)
    }

    #[inline(always)]
    pub fn stroke_color(self, stroke_color: &'a str) -> Self {
        self.with("stroke", stroke_color)
    }

    #[inline(always)]
    pub fn stroke_width(self, stroke_width: &'a str) -> Self {
        self.with("stroke-width", stroke_width)
    }

    #[inline(always)]
    pub fn with(self, attr: &'a str, value: &'a str) -> Self {
        let mut data = self.data;
        let mut pos = self.pos;
        data[pos as usize] = (attr, value.into());
        pos = (pos + 1) % MAX_ATTRS as u8;

        Self { data, pos }
    }
    fn add_to_svg(&self, svg: Option<&String>) -> Option<String> {
        if let Some(svg) = svg {
            let mut svg = svg.to_owned();
            let mut attrs = String::new();
            for i in 0..self.pos {
                let (k, v) = &self.data[i as usize];
                attrs.push_str(&format!(" {k}=\"{v}\""));
            }

            if !attrs.is_empty() {
                svg.insert_str(4, &attrs);
            }
            Some(svg)
        } else {
            None
        }
    }
}

#[cfg(feature = "json")]
impl<'a> From<&'a serde_json::Value> for IconAttrs<'a> {
    fn from(value: &'a serde_json::Value) -> Self {
        let mut attrs = IconAttrs::default();
        if let serde_json::Value::Object(map) = value {
            for (k, v) in map {
                let s = match v {
                    serde_json::Value::String(s) => s.into(),
                    serde_json::Value::Number(n) => n.to_string().into(),
                    serde_json::Value::Bool(b) => b.to_string().into(),
                    _ => continue,
                };

                attrs.data[attrs.pos as usize] = (k, s);
                attrs.pos = (attrs.pos + 1) % MAX_ATTRS as u8;
            }
        }
        attrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minify_html::{minify, Cfg};
    const CFG: Cfg = Cfg {
        keep_closing_tags: true,
        do_not_minify_doctype: false,
        ensure_spec_compliant_unquoted_attribute_values: false,
        keep_html_and_head_opening_tags: false,
        keep_spaces_between_attributes: true,
        keep_comments: false,
        minify_css: false,
        minify_js: false,
        remove_bangs: false,
        remove_processing_instructions: false,
    };

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_icon_fill_should_work() {
        assert_eq!(
            get(IconType::Bootstrap(Bootstrap::Fill), "alarm"),
            Some(&expected(include_str!(
                "../icon_resources/bootstrap/icons/alarm-fill.svg"
            )))
        );
    }

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_icon_should_work() {
        assert_eq!(
            get(IconType::Bootstrap(Bootstrap::Normal), "alarm"),
            Some(&expected(include_str!(
                "../icon_resources/bootstrap/icons/alarm.svg"
            )))
        );
    }

    #[cfg(feature = "feather")]
    #[test]
    fn feather_icon_should_work() {
        assert_eq!(
            get(IconType::Feather(Feather::Normal), "activity"),
            Some(&expected(include_str!(
                "../icon_resources/feather/icons/activity.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_brands_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Regular), "500px"),
            Some(&expected(include_str!(
                "../icon_resources/font-awesome/svgs/brands/500px.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_regular_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Regular), "address-book"),
            Some(&expected(include_str!(
                "../icon_resources/font-awesome/svgs/regular/address-book.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_solid_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Solid), "address-book"),
            Some(&expected(include_str!(
                "../icon_resources/font-awesome/svgs/solid/address-book.svg"
            )))
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_icon_outline_should_work() {
        assert_eq!(
            get(IconType::Heroicons(Heroicons::Outline), "academic-cap"),
            Some(&expected(include_str!(
                "../icon_resources/heroicons/optimized/24/outline/academic-cap.svg"
            )))
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_icon_solid_should_work() {
        assert_eq!(
            get(IconType::Heroicons(Heroicons::Solid), "academic-cap"),
            Some(&expected(include_str!(
                "../icon_resources/heroicons/optimized/24/solid/academic-cap.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_outline_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Outline), "alarm"),
            Some(&expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm-outline.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_sharp_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Sharp), "alarm"),
            Some(&expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm-sharp.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Normal), "alarm"),
            Some(&expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm.svg"
            )))
        );
    }

    #[cfg(feature = "octicons")]
    #[test]
    fn octicons_icon_should_work() {
        assert_eq!(
            get(IconType::Octicons(Octicons::Normal), "alert"),
            Some(&expected(include_str!(
                "../icon_resources/octicons/icons/alert-24.svg"
            )))
        );
    }

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_not_filled_should_work() {
        assert_eq!(
            bootstrap("alarm", false, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/bootstrap/icons/alarm.svg"
            )))
        );
    }

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_filled_should_work() {
        assert_eq!(
            bootstrap("alarm", true, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/bootstrap/icons/alarm-fill.svg"
            )))
        );
    }

    #[cfg(feature = "feather")]
    #[test]
    fn feather_should_work() {
        assert_eq!(
            feather("activity", IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/feather/icons/activity.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_brands_should_work() {
        assert_eq!(
            font_awesome("500px", FontAwesome::Solid, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/font-awesome/svgs/brands/500px.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_regular_should_work() {
        assert_eq!(
            font_awesome("address-book", FontAwesome::Regular, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/font-awesome/svgs/regular/address-book.svg"
            )))
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_solid_should_work() {
        assert_eq!(
            font_awesome("address-book", FontAwesome::Solid, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/font-awesome/svgs/solid/address-book.svg"
            )))
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_outline_should_work() {
        assert_eq!(
            heroicons("academic-cap", true, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/heroicons/optimized/24/outline/academic-cap.svg"
            )))
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_solid_should_work() {
        assert_eq!(
            heroicons("academic-cap", false, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/heroicons/optimized/24/solid/academic-cap.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_outline_should_work() {
        assert_eq!(
            ionicons("alarm", Ionicons::Outline, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm-outline.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_sharp_should_work() {
        assert_eq!(
            ionicons("alarm", Ionicons::Sharp, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm-sharp.svg"
            )))
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_should_work() {
        assert_eq!(
            ionicons("alarm", Ionicons::Normal, IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/ionicons/src/svg/alarm.svg"
            )))
        );
    }

    #[cfg(feature = "octicons")]
    #[test]
    fn octicons_should_work() {
        assert_eq!(
            octicons("alert", IconAttrs::default()),
            Some(expected(include_str!(
                "../icon_resources/octicons/icons/alert-24.svg"
            )))
        );
    }

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_with_class_should_work() {
        let attrs = IconAttrs::default()
            .class("h-8 w-8 text-white")
            .fill("none")
            .stroke_color("currentColor");

        let icon = bootstrap("alarm", false, attrs).expect("exists");
        assert_eq!(&icon[..32], "<svg class=\"h-8 w-8 text-white\" ");
    }

    #[cfg(all(feature = "heroicons", feature = "json"))]
    #[test]
    fn json_attribute_should_work() {
        let attrs = &serde_json::json!({
            "class": "h-8 w-8 text-white",
            "fill": "none",
            "stroke_color": "currentColor",
        });

        let icon = heroicons("academic-cap", true, attrs.into()).expect("exists");
        assert_eq!(&icon[..32], "<svg class=\"h-8 w-8 text-white\" ");
    }

    #[test]
    fn icon_should_not_exist() {
        assert_eq!(get(IconType::Feather(Feather::Normal), "not_exist"), None);
    }

    fn expected(s: &str) -> String {
        String::from_utf8(minify(s.as_bytes(), &CFG)).unwrap()
    }
}
