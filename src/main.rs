use crate::xac::xac_parser::xacparse;

mod xac;
mod xmf;
mod xsm;

fn main() {
    let file_location = "/home/ridwan/IdeaProjects/orsha-parser/berkas/Warrior_m_centurion01.xac";

    let _filepath = xacparse(file_location);
    /*
    println!("{:#?}", filepath.header);
    println!("{:#?}", filepath.metadata);
    println!("{:#?}", filepath.export_date);
    println!("{:#?}", filepath.original_filename);
    println!("{:#?}", filepath.source_app);

     */
    /*
    let mut output_file = std::fs::File::create(
        Path::new(&file_location)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + ".json",
    )
    .unwrap();

    serde_json::to_writer_pretty(&mut output_file, &filepath).unwrap();

     */
}
