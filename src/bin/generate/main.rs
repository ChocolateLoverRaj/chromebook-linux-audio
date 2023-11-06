use async_fs::{create_dir, create_dir_all, remove_dir_all};
use chromebook_audio::SOF_BOARD_GENERATIONS;

const EXTENSIONS_DIR: &str = "extensions";

async fn create_extension(name: &str) {
    let extension_dir = &format!("{}/{}", EXTENSIONS_DIR, name)[..];
    create_dir(extension_dir).await.unwrap();
    let extension_release_d_dir = &format!("{}/usr/lib/extension-release.d", extension_dir)[..];
    create_dir_all(extension_release_d_dir).await.unwrap();
    async_fs::write(
        format!("{}/extension-release.{}", extension_release_d_dir, name),
        "",
    )
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    let _ = remove_dir_all(EXTENSIONS_DIR).await;
    create_dir(EXTENSIONS_DIR).await.unwrap();
    create_extension("chromebook-sof-common").await;
    create_dir_all(format!(
        "{}/chromebook-sof-common/usr/share/alsa/ucm2",
        EXTENSIONS_DIR
    ))
    .await
    .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "chromebook-ucm-conf/common",
            format!(
                "{}/chromebook-sof-common/usr/share/alsa/ucm2/common",
                EXTENSIONS_DIR
            ),
        )
        .unwrap();

    for board_generation in SOF_BOARD_GENERATIONS {
        let dirs = match board_generation {
            "glk" => vec!["sof-glkda7219ma", "sof-cs42l42", "sof-glkrt5682ma"],
            "apl" => vec!["sof-bxtda7219ma"],
            "cml" => vec![
                "sof-rt5682",
                "sof-cmlda7219ma",
                "sof-cml_rt1011_",
                "sof-cml_max9839",
            ],
            "tgl" => vec!["sof-rt5682"],
            "jsl" => vec!["sof-rt5682", "sof-da7219max98", "sof-cs42l42"],
            // FIXME: jsl may need to copy a file to /usr/lib/firmware/intel/sof-tplg
            "adl" => vec!["sof-rt5682", "sof-nau8825", "sof-ssp_amp"],
            _ => panic!(
                "SOF setup not implemented for board generation: {:?}",
                board_generation
            ),
        };
        let extension_name = &format!("chromebook-sof-{}", board_generation)[..];
        create_extension(extension_name).await;
        create_dir_all(format!(
            "{}/{}/usr/share/alsa/ucm2/conf.d",
            EXTENSIONS_DIR, extension_name
        ))
        .await
        .unwrap();
        for dir in dirs {
            cp_r::CopyOptions::new()
                .copy_tree(
                    format!("chromebook-ucm-conf/{}/{}", board_generation, dir),
                    format!(
                        "{}/{}/usr/share/alsa/ucm2/conf.d/{}",
                        EXTENSIONS_DIR, extension_name, dir
                    ),
                )
                .unwrap();
        }
    }
}
