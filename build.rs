use anyhow::Result;
use convert_case::{Case, Casing};
use flate2::write::GzEncoder;
use minify_html::{minify, Cfg};
use minijinja::{Environment, Source};
use serde_json::json;
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

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
    let engine = get_engine()?;

    let icons = [
        ("bootstrap", "icons"),
        ("feather", "icons"),
        ("ionicons", "src/svg"),
        ("octicons", "icons"),
    ];

    for (name, inner_path) in icons.iter() {
        let data = get_icon_data(&get_path(name, inner_path), &cfg)?;

        let name = name.to_case(Case::Snake);
        let bin = encap(&data)?;
        fs::write(gen_dir.join(format!("{name}.bin")), bin)?;

        let mut context = json!({ "name": name });
        for key in data.keys() {
            context[key] = json!(true);
        }
        let writer = File::create(gen_dir.join(format!("{name}.rs")))?;
        let tpl = engine.get_template("lazy.rs")?;
        tpl.render_to_write(context, writer)?;
    }

    let icons = [
        ("font-awesome", "svgs", vec!["regular", "solid"]),
        ("heroicons", "optimized/24", vec!["outline", "solid"]),
    ];

    for (name, inner_path, categories) in icons.iter() {
        let data = get_icon_data_by_category(&get_path(name, inner_path), categories, &cfg)?;
        let bin = encap(&data)?;

        let name = name.to_case(Case::Snake);
        fs::write(gen_dir.join(format!("{name}.bin")), bin)?;

        let mut context = json!({ "name": name });
        for key in data.keys() {
            context[key] = json!(true);
        }
        let writer = File::create(gen_dir.join(format!("{name}.rs")))?;
        let tpl = engine.get_template("lazy.rs")?;
        tpl.render_to_write(context, writer)?;
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

        if path.ends_with("font-awesome/svgs") {
            for entry in fs::read_dir(path.join("brands"))? {
                let entry = entry?;
                let path = entry.path();
                let name = path.file_stem().unwrap().to_str().unwrap();
                let content = fs::read(&path)?;
                map.insert(name.to_owned(), String::from_utf8(minify(&content, cfg))?);
            }
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

fn get_engine() -> Result<Environment<'static>> {
    let mut env = Environment::default();
    let mut source = Source::new();
    source.add_template("lazy.rs", include_str!("templates/lazy.rs.j2"))?;
    env.add_filter("pascal", |v: String| v.to_case(Case::Pascal));

    env.set_source(source);
    Ok(env)
}

fn encap(data: &NestedMap) -> Result<Vec<u8>> {
    let bin = bincode::encode_to_vec(data, bincode::config::standard())?;
    let buf = Vec::new();
    let mut encoder = GzEncoder::new(buf, flate2::Compression::default());
    encoder.write_all(&bin)?;
    Ok(encoder.finish()?)
}
