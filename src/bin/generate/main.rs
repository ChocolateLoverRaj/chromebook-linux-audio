use async_fs::{copy, create_dir, create_dir_all, remove_dir_all, remove_file};
use chromebook_audio::{
    get_avs_extension_name, get_avs_max98357a_extension_name, get_common_ucm_extension_name,
    get_mt8138_extension_name, EXTENSION_PREFIX, SOF_BOARD_GENERATIONS,
};

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
    let common_ucm_extension_name = &get_common_ucm_extension_name()[..];
    create_extension(common_ucm_extension_name).await;
    create_dir_all(format!(
        "{}/{}/usr/share/alsa/ucm2",
        EXTENSIONS_DIR, common_ucm_extension_name
    ))
    .await
    .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "chromebook-ucm-conf/common",
            format!(
                "{}/{}/usr/share/alsa/ucm2/common",
                EXTENSIONS_DIR, common_ucm_extension_name
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
        let extension_name = &format!("{}-sof-{}", EXTENSION_PREFIX, board_generation)[..];
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

    let avs_extension_name = &get_avs_extension_name()[..];
    create_extension(avs_extension_name).await;
    create_dir_all(format!(
        "{}/{}/usr/share/alsa/ucm2/conf.d",
        EXTENSIONS_DIR, avs_extension_name
    ))
    .await
    .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "chromebook-ucm-conf/avs",
            format!(
                "{}/{}/usr/share/alsa/ucm2/conf.d",
                EXTENSIONS_DIR, avs_extension_name
            ),
        )
        .unwrap();
    create_dir_all(format!(
        "{}/{}/usr/lib/firmware/intel/avs",
        EXTENSIONS_DIR, avs_extension_name
    ))
    .await
    .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "conf/avs/tplg",
            format!(
                "{}/{}/usr/lib/firmware/intel/avs",
                EXTENSIONS_DIR, avs_extension_name
            ),
        )
        .unwrap();
    // updated avs dsp firmware recently got merged upstream but is not packaged in any distro yet
    // FIXME: Just wait until it's packaged unless it's still not packaged until later
    remove_file(format!(
        "{}/{}/usr/lib/firmware/intel/avs/max98357a-tplg.bin",
        EXTENSIONS_DIR, avs_extension_name
    ))
    .await
    .unwrap();
    let avs_max98357a_extension_name = &get_avs_max98357a_extension_name()[..];
    create_extension(avs_max98357a_extension_name).await;
    create_dir_all(format!(
        "{}/{}/usr/lib/firmware/intel/avs",
        EXTENSIONS_DIR, avs_max98357a_extension_name
    ))
    .await
    .unwrap();
    copy(
        "conf/avs/tplg/max98357a-tplg.bin",
        format!(
            "{}/{}/usr/lib/firmware/intel/avs/max98357a-tplg.bin",
            EXTENSIONS_DIR, avs_max98357a_extension_name
        ),
    )
    .await
    .unwrap();

    let mt8138_extension_name = &get_mt8138_extension_name()[..];
    create_extension(mt8138_extension_name).await;
    create_dir_all(format!(
        "{}/{}/usr/share/alsa/ucm2/conf.d",
        EXTENSIONS_DIR, mt8138_extension_name
    ))
    .await
    .unwrap();
    for dir in ["mt8183_da7219_r", "mt8183_mt6358_t"] {
        cp_r::CopyOptions::new()
            .copy_tree(
                format!("chromebook-ucm-conf/mt8183/{}", dir),
                format!(
                    "{}/{}/usr/share/alsa/ucm2/conf.d/{}",
                    EXTENSIONS_DIR, mt8138_extension_name, dir
                ),
            )
            .unwrap();
    }

    for platform in ["stoney", "picasso", "cezanne", "mendocino"] {
        let extension_name = &format!("{}-{}", EXTENSION_PREFIX, platform)[..];
        create_extension(extension_name).await;
        create_dir_all(format!(
            "{}/{}/usr/share/alsa/ucm2/conf.d",
            EXTENSIONS_DIR, extension_name
        ))
        .await
        .unwrap();
        let dirs = match platform {
            "stoney" => vec!["acpd7219m98357"],
            "picasso" => vec!["acp3xalc5682m98", "acp3xalc5682101"],
            "cezanne" => vec!["sof-rt5682s-rt1"],
            "mendocino" => vec!["sof-rt5682s-hs-"],
            _ => panic!(
                "AMD setup not implemented for board generation: {:?}",
                platform
            ),
        };
        for dir in dirs {
            cp_r::CopyOptions::new()
                .copy_tree(
                    format!("chromebook-ucm-conf/{}/{}", platform, dir),
                    format!(
                        "{}/{}/usr/share/alsa/ucm2/conf.d/{}",
                        EXTENSIONS_DIR, extension_name, dir
                    ),
                )
                .unwrap();
        }
    }
    let mendocino_extension = &format!("{}-mendocino", EXTENSION_PREFIX)[..];
    create_dir_all(format!(
        "{}/{}/usr/lib/firmware/amd/sof/community",
        EXTENSIONS_DIR, mendocino_extension
    ))
    .await
    .unwrap();
    create_dir_all(format!(
        "{}/{}/usr/lib/firmware/amd/sof-tplg",
        EXTENSIONS_DIR, mendocino_extension
    ))
    .await
    .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "conf/amd-sof/fw",
            format!(
                "{}/{}/usr/lib/firmware/amd/sof/community",
                EXTENSIONS_DIR, mendocino_extension
            ),
        )
        .unwrap();
    cp_r::CopyOptions::new()
        .copy_tree(
            "conf/amd-sof/tplg",
            format!(
                "{}/{}/usr/lib/firmware/amd/sof-tplg",
                EXTENSIONS_DIR, mendocino_extension
            ),
        )
        .unwrap();
}
