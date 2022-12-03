use anyhow::Result;
use convert_case::{Case, Casing};
use minify_html::{minify, Cfg};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
};
use tera::{Context, Tera};

type Map = HashMap<String, String>;
type NestedMap = HashMap<&'static str, Map>;

fn main() -> Result<()> {
    if option_env!("BUILD_ICONS").is_none() {
        return Ok(());
    }

    let cfg = Cfg {
        keep_closing_tags: true,
        keep_spaces_between_attributes: true,
        ..Default::default()
    };

    let gen_dir = current_dir()?.join("src/gen");
    let tera = get_tera()?;

    let icons = [
        ("bootstrap", "icons"),
        ("feather", "icons"),
        ("ionicons", "src/svg"),
        ("octicons", "icons"),
    ];

    for (name, inner_path) in icons.iter() {
        let data = get_icon_data(&get_path(name, inner_path), &cfg)?;

        let context = Context::from_serialize(data)?;
        let writer = File::create(gen_dir.join(format!("{}.rs", name.to_case(Case::Snake))))?;
        tera.render_to("general.rs", &context, writer)?;
    }

    let icons = [
        ("font-awesome", "svgs", vec!["brands", "regular", "solid"]),
        ("heroicons", "src/24", vec!["outline", "solid"]),
    ];

    for (name, inner_path, categories) in icons.iter() {
        let data = get_icon_data_by_category(&get_path(name, inner_path), categories, &cfg)?;

        let context = Context::from_serialize(data)?;
        let writer = File::create(gen_dir.join(format!("{}.rs", name.to_case(Case::Snake))))?;
        tera.render_to("general.rs", &context, writer)?;
    }

    Command::new("cargo").arg("fmt").output()?;
    Ok(())
}

fn get_path(name: &str, inner_path: &str) -> PathBuf {
    current_dir()
        .unwrap()
        .join("icon_resources")
        .join(name)
        .join(inner_path)
}

fn get_icon_data_by_category(
    path: &Path,
    categories: &[&'static str],
    cfg: &Cfg,
) -> Result<NestedMap> {
    let mut data = NestedMap::new();

    for category in categories {
        let mut map = Map::new();

        for entry in fs::read_dir(path.join(category))? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let content = fs::read(&path)?;
            map.insert(name.to_owned(), String::from_utf8(minify(&content, cfg))?);
        }

        data.insert(category, map);
    }

    Ok(data)
}

fn get_icon_data(path: &Path, cfg: &Cfg) -> Result<NestedMap> {
    let mut data = NestedMap::new();

    let mut fill = HashMap::new();
    let mut normal = HashMap::new();

    let mut outline = HashMap::new();
    let mut sharp = HashMap::new();

    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let content = String::from_utf8(minify(&fs::read(&path)?, cfg))?;
        if !path.is_file() {
            continue;
        }

        let name = path.file_stem().and_then(|v| v.to_str()).unwrap();

        if name.ends_with("-16") {
            continue;
        }

        if name == "color-fill" {
            normal.insert(name.to_owned(), content);
        } else if name.ends_with("-fill") {
            let icon_name = name.replace("-fill", "");
            fill.insert(icon_name.to_owned(), content);
        } else if name.ends_with("-24") {
            let icon_name = name.replace("-24", "");
            normal.insert(icon_name.to_owned(), content);
        } else if name.ends_with("-outline") {
            let icon_name = name.replace("-outline", "");
            outline.insert(icon_name.to_owned(), content);
        } else if name.ends_with("-sharp") {
            let icon_name = name.replace("-sharp", "");
            sharp.insert(icon_name.to_owned(), content);
        } else {
            normal.insert(name.to_owned(), content);
        }
    }
    data.insert("fill", fill);
    data.insert("normal", normal);
    data.insert("outline", outline);
    data.insert("sharp", sharp);
    Ok(data)
}

fn get_tera() -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_template("general.rs", include_str!("templates/general.rs.j2"))?;
    tera.register_filter(
        "pascal",
        |v: &tera::Value, _args: &HashMap<String, tera::Value>| match v.as_str() {
            Some(s) => Ok(tera::Value::String(s.to_case(Case::Pascal))),
            None => Err(tera::Error::msg("expected a string")),
        },
    );

    Ok(tera)
}
