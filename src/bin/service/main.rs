mod board_generations;
mod get_board_name;
use async_fs::{create_dir_all, remove_dir_all};
use board_generations::get_board_generations;
use chromebook_audio::get_extension_dir;
use get_board_name::get_board_name;
use std::{os::unix::fs::symlink, vec};
use tokio::join;

async fn setup_sof(board_generation: &str) {
    // TODO: # Force sof driver
    // print_status("Installing modprobe config")
    // cpfile("conf/sof/snd-sof.conf", "/etc/modprobe.d/snd-sof.conf")
    let extension_dir = get_extension_dir();
    create_dir_all(format!("{extension_dir}/usr/share/alsa/ucm2/conf.d"))
        .await
        .unwrap();
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

    for dir in dirs {
        symlink(
            &format!("{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/{board_generation}/{dir}"),
            &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/{dir}"),
        )
        .unwrap();
    }

    // Common dmic split ucm
    symlink(
        &format!("{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/dmic-common"),
        &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/dmic-common"),
    )
    .unwrap();

    // Common hdmi split ucm
    symlink(
        &format!("{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf/hdmi-common"),
        &format!("{extension_dir}/usr/share/alsa/ucm2/conf.d/hdmi-common"),
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    let (board_name, board_generations) = join!(get_board_name(), get_board_generations());
    let board_name = board_name.unwrap();
    // let board_name = String::from("fleex");
    let board_generation = match board_generations.get(&board_name) {
        Some(v) => Some(v as &str),
        None => None,
    };
    let extension_dir = get_extension_dir();
    let _ = remove_dir_all(format!("{extension_dir}/usr/share")).await;
    match board_generation {
        Some(board_generation) => match board_generation {
            "bdw" | "byt" | "bsw" => {
                println!(
                    "bsw audio not implemented yet. Contact dev if you need it for some reason."
                )
            }
            "skl" | "kbl" => println!("AVS not implemented yet"),
            "apl" | "glk" | "cml" | "jsl" | "tgl" | "adl" => setup_sof(board_generation).await,
            _ => {
                println!("Do not know how to setup audio for board generation '{board_generation}'")
            }
        },
        None => {
            println!("Not overlaying any audio modifications because the device {board_name} is not known to be a Chromebook.");
        }
    }
}
