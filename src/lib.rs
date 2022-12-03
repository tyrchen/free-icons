mod gen;

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
    Brands,
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

pub fn get(icon_type: IconType, name: &str) -> Option<&'static str> {
    match icon_type {
        #[cfg(feature = "bootstrap")]
        IconType::Bootstrap(icon_type) => match icon_type {
            Bootstrap::Fill => gen::bootstrap::FILL.get(name).copied(),
            Bootstrap::Normal => gen::bootstrap::NORMAL.get(name).copied(),
        },
        #[cfg(feature = "feather")]
        IconType::Feather(icon_type) => match icon_type {
            Feather::Normal => gen::feather::NORMAL.get(name).copied(),
        },
        #[cfg(feature = "font-awesome")]
        IconType::FontAwesome(icon_type) => match icon_type {
            FontAwesome::Brands => gen::font_awesome::BRANDS.get(name).copied(),
            FontAwesome::Regular => gen::font_awesome::REGULAR.get(name).copied(),
            FontAwesome::Solid => gen::font_awesome::SOLID.get(name).copied(),
        },
        #[cfg(feature = "heroicons")]
        IconType::Heroicons(icon_type) => match icon_type {
            Heroicons::Outline => gen::heroicons::OUTLINE.get(name).copied(),
            Heroicons::Solid => gen::heroicons::SOLID.get(name).copied(),
        },
        #[cfg(feature = "ionicons")]
        IconType::Ionicons(icon_type) => match icon_type {
            Ionicons::Outline => gen::ionicons::OUTLINE.get(name).copied(),
            Ionicons::Sharp => gen::ionicons::SHARP.get(name).copied(),
            Ionicons::Normal => gen::ionicons::NORMAL.get(name).copied(),
        },
        #[cfg(feature = "octicons")]
        IconType::Octicons(icon_type) => match icon_type {
            Octicons::Normal => gen::octicons::NORMAL.get(name).copied(),
        },
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
            Some(
                expected(include_str!(
                    "../icon_resources/bootstrap/icons/alarm-fill.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "bootstrap")]
    #[test]
    fn bootstrap_icon_should_work() {
        assert_eq!(
            get(IconType::Bootstrap(Bootstrap::Normal), "alarm"),
            Some(expected(include_str!("../icon_resources/bootstrap/icons/alarm.svg")).as_str())
        );
    }

    #[cfg(feature = "feather")]
    #[test]
    fn feather_icon_should_work() {
        assert_eq!(
            get(IconType::Feather(Feather::Normal), "activity"),
            Some(expected(include_str!("../icon_resources/feather/icons/activity.svg")).as_str())
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_brands_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Brands), "500px"),
            Some(
                expected(include_str!(
                    "../icon_resources/font-awesome/svgs/brands/500px.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_regular_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Regular), "address-book"),
            Some(
                expected(include_str!(
                    "../icon_resources/font-awesome/svgs/regular/address-book.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "font-awesome")]
    #[test]
    fn font_awesome_icon_solid_should_work() {
        assert_eq!(
            get(IconType::FontAwesome(FontAwesome::Solid), "address-book"),
            Some(
                expected(include_str!(
                    "../icon_resources/font-awesome/svgs/solid/address-book.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_icon_outline_should_work() {
        assert_eq!(
            get(IconType::Heroicons(Heroicons::Outline), "academic-cap"),
            Some(
                expected(include_str!(
                    "../icon_resources/heroicons/src/24/outline/academic-cap.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "heroicons")]
    #[test]
    fn heroicons_icon_solid_should_work() {
        assert_eq!(
            get(IconType::Heroicons(Heroicons::Solid), "academic-cap"),
            Some(
                expected(include_str!(
                    "../icon_resources/heroicons/src/24/solid/academic-cap.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_outline_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Outline), "alarm"),
            Some(
                expected(include_str!(
                    "../icon_resources/ionicons/src/svg/alarm-outline.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_sharp_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Sharp), "alarm"),
            Some(
                expected(include_str!(
                    "../icon_resources/ionicons/src/svg/alarm-sharp.svg"
                ))
                .as_str()
            )
        );
    }

    #[cfg(feature = "ionicons")]
    #[test]
    fn ionicons_icon_should_work() {
        assert_eq!(
            get(IconType::Ionicons(Ionicons::Normal), "alarm"),
            Some(expected(include_str!("../icon_resources/ionicons/src/svg/alarm.svg")).as_str())
        );
    }

    #[cfg(feature = "octicons")]
    #[test]
    fn octicons_icon_should_work() {
        assert_eq!(
            get(IconType::Octicons(Octicons::Normal), "alert"),
            Some(
                expected(include_str!(
                    "../icon_resources/octicons/icons/alert-24.svg"
                ))
                .as_str()
            )
        );
    }

    #[test]
    fn icon_should_not_exist() {
        assert_eq!(get(IconType::Feather(Feather::Normal), "not_exist"), None);
    }

    fn expected(s: &str) -> String {
        String::from_utf8(minify(s.as_bytes(), &CFG)).unwrap()
    }
}
