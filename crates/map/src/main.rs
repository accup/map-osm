mod accepted_member_roles;
mod build_database_content;
mod build_extraction_filter;
mod database_metadata;
mod ensure_japan_osm_pbf;
mod japan_osm_pbf_path;
mod japan_osm_pbf_url;
mod japan_routes_database_path;
mod line_classification_rules;
mod point_classification_rules;
mod point_kind;
mod relation_classification_rules;
mod route_kind;
mod simplify_tolerance_meters;

use std::error::Error;
use std::path::Path;

use crate::build_database_content::build_database_content;
use crate::build_extraction_filter::build_extraction_filter;
use crate::ensure_japan_osm_pbf::ensure_japan_osm_pbf;
use crate::japan_routes_database_path::JAPAN_ROUTES_DATABASE_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let pbf_path = ensure_japan_osm_pbf()?;

    println!("{} から地物を抽出している", pbf_path.display());
    let extraction = osm_extract::extract(&pbf_path, &build_extraction_filter())?;

    println!(
        "地点 {} 件・路線 {} 件・関係 {} 件からデータベースの内容を構築している",
        extraction.points.len(),
        extraction.lines.len(),
        extraction.relations.len()
    );
    let content = build_database_content(&extraction);

    geo_sqlite::write_database(Path::new(JAPAN_ROUTES_DATABASE_PATH), &content)?;
    println!("{JAPAN_ROUTES_DATABASE_PATH} へ保存した");

    Ok(())
}
