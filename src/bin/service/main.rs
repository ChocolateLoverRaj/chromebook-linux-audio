mod board_generations;
mod get_board_name;
use async_fs::{create_dir_all, remove_dir_all};
use board_generations::get_board_generations;
use chromebook_audio::get_extension_dir;
use get_board_name::get_board_name;
use std::os::unix::fs::symlink;
use tokio::join;

#[derive(Debug)]
enum SofBoardGeneration {
    GLK,
    CML,
    JSL,
    TGL,
    ADL,
}
async fn setup_sof(board_generation: SofBoardGeneration) {
    // TODO: # Force sof driver
    // print_status("Installing modprobe config")
    // cpfile("conf/sof/snd-sof.conf", "/etc/modprobe.d/snd-sof.conf")

    match board_generation {
        SofBoardGeneration::CML => {
            let extension_dir = get_extension_dir();
            create_dir_all(format!("{extension_dir}/usr/share/alsa/ucm2/conf.d"))
                .await
                .unwrap();
            // TODO: Async symlink
            let cml_files = &[
                "sof-rt5682",
                "sof-cmlda7219ma",
                "sof-cml_rt1011_",
                "sof-cml_max9839",
            ];
            for &cml_file in cml_files {
                symlink(
            &format!("{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/cml/{cml_file}"),
            &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/{cml_file}"),
        )
        .unwrap();
            }

            // Common dmic split ucm
            symlink(
                &format!(
                    "{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/dmic-common"
                ),
                &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/dmic-common"),
            )
            .unwrap();

            // Common hdmi split ucm
            symlink(
                &format!(
                    "{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/hdmi-common"
                ),
                &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/hdmi-common"),
            )
            .unwrap();
        }
        _ => panic!(
            "SOF setup not implemented for board generation: {:?}",
            board_generation
        ),
    }
}

#[tokio::main]
async fn main() {
    let (board_name, board_generations) = join!(get_board_name(), get_board_generations());
    let board_name = board_name.unwrap();
    // let board_name = String::from("jinlon");
    let board_generation: &str = match board_generations.get(&board_name) {
        Some(v) => v,
        None => panic!("Chromebook {board_name} not found in board generations"),
    };
    let extension_dir = get_extension_dir();
    let _ = remove_dir_all(format!("{extension_dir}/usr/share")).await;
    match board_generation {
        "cml" => setup_sof(SofBoardGeneration::CML).await,
        _ => panic!("Do not know how to setup audio for board generation '{board_generation}'"),
    }
}
