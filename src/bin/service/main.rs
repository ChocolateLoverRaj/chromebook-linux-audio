use crate::{board_generations::get_board_generations, get_board_name::get_board_name};
use chromebook_audio::{
    get_avs_extension_name, get_avs_max98357a_extension_name, get_common_ucm_extension_name,
    get_mt8138_extension_name, EXTENSION_PREFIX,
};
use fuser::{spawn_mount2, BackgroundSession, MountOption};
use parking::Parker;
use std::process::Command;
use tokio::join;
mod board_generations;
mod get_board_name;
mod virtual_file;
use virtual_file::VirtualFile;
mod get_audio_choice;
use get_audio_choice::{get_audio_choice, AudioChoice};
mod has_max98357a;
use has_max98357a::has_max98357a;

#[tokio::main]
async fn main() {
    let (board_name, board_generations) = join!(get_board_name(), get_board_generations());
    let board_name = board_name.unwrap();
    // let board_name = String::from("frostflow");
    let board_generation = match board_generations.get(&board_name) {
        Some(v) => Some(v as &str),
        None => None,
    };

    match board_generation {
        Some(board_generation) => {
            fn spawn_fuse_redirect(source_file: String, target_file: &str) -> BackgroundSession {
                spawn_mount2(
                    VirtualFile { source_file },
                    target_file,
                    &[
                        MountOption::RO,
                        MountOption::AllowOther,
                        MountOption::AutoUnmount,
                    ],
                )
                .unwrap()
            }

            fn enable_overlay(overlay: String) -> BackgroundSession {
                println!("Enabling overlay: {overlay}");
                let target_file = &format!(
                    "/etc/extensions/{overlay}/usr/lib/extension-release.d/extension-release.{overlay}"
                )[..];
                spawn_fuse_redirect(String::from("/etc/os-release"), target_file)
            }

            fn finish(fuses: Vec<BackgroundSession>) {
                Command::new("systemctl")
                    .args(&["restart", "systemd-sysext"])
                    .output()
                    .unwrap();
                let parker = Parker::new();
                let unparker = parker.unparker();
                ctrlc::set_handler(move || {
                    unparker.unpark();
                    println!("Exiting");
                })
                .unwrap();
                parker.park();
                for fuse in fuses {
                    drop(fuse);
                }
            }

            let enable_sof = || {
                println!("Enabling SOF audio");
                let common_fuse = enable_overlay(get_common_ucm_extension_name());
                let board_fuse =
                    enable_overlay(format!("{}-sof-{}", EXTENSION_PREFIX, board_generation));

                let modprobe_fuse = spawn_fuse_redirect(
                    String::from("/usr/share/chromebook-audio/snd-sof.conf"),
                    "/etc/modprobe.d/snd-sof.conf",
                );
                finish(vec![common_fuse, board_fuse, modprobe_fuse]);
            };

            fn enable_avs(use_max98357a: bool) {
                println!("Enabling AVS");
                let mut fuses: Vec<BackgroundSession> = vec![
                    enable_overlay(get_avs_extension_name()),
                    spawn_fuse_redirect(
                        String::from("/usr/share/chromebook-audio/snd-avs.conf"),
                        "/etc/modprobe.d/snd-avs.conf",
                    ),
                    spawn_fuse_redirect(
                        String::from("/usr/share/chromebook-audio/51-avs-dmic.lua"),
                        "/etc/wireplumber/main.lua.d/51-avs-dmic.lua",
                    ),
                ];
                if use_max98357a {
                    println!("Enabling max98357a");
                    fuses.push(enable_overlay(get_avs_max98357a_extension_name()))
                } else {
                    println!("Not enabling max98357a");
                }
                finish(fuses)
            }

            match board_generation {
                "bdw" | "byt" | "bsw" => {
                    println!("Enabling bsw audio");
                    let bsw_sof_fuse = spawn_fuse_redirect(
                        String::from("/usr/share/chromebook-audio/bsw-sof.conf"),
                        "/etc/modprobe.d/bsw-sof.conf",
                    );
                    finish(vec![bsw_sof_fuse]);
                }
                "skl" | "kbl" => {
                    let (has_max98357a, audio_choice) = join!(has_max98357a(), get_audio_choice());
                    let max98357a_chosen = match audio_choice {
                        Some(audio_choice) => match audio_choice {
                            AudioChoice::AvsWithMax98357a => true,
                            _ => false,
                        },
                        None => false,
                    };
                    let use_max98357a = has_max98357a && max98357a_chosen;
                    enable_avs(use_max98357a);
                }
                "apl" => {
                    let (has_max98357a, audio_choice) = join!(has_max98357a(), get_audio_choice());
                    let audio_to_use = match audio_choice {
                        Some(audio_choice) => match audio_choice {
                            AudioChoice::AvsWithMax98357a => match has_max98357a {
                                true => AudioChoice::AvsWithMax98357a,
                                false => AudioChoice::AvsWithoutMax98357a,
                            },
                            _ => audio_choice,
                        },
                        // It's better to have stable audio with speakers working by default than audio with only headphone jack and internal mic working.
                        None => AudioChoice::Sof,
                    };
                    match audio_to_use {
                        AudioChoice::Sof => enable_sof(),
                        AudioChoice::AvsWithoutMax98357a => enable_avs(false),
                        AudioChoice::AvsWithMax98357a => enable_avs(true),
                    }
                }
                "glk" | "cml" | "tgl" | "jsl" | "adl" => enable_sof(),
                "stoney" | "picasso" | "cezanne" | "mendocino" => {
                    println!("Enabling AMD audio");
                    let common_fuse = enable_overlay(get_common_ucm_extension_name());
                    let board_fuse =
                        enable_overlay(format!("{}-{}", EXTENSION_PREFIX, board_generation));
                    finish(vec![common_fuse, board_fuse]);
                }
                "mt8183" => {
                    println!("Enabling mt8183 audio");
                    finish(vec![enable_overlay(get_mt8138_extension_name())]);
                }
                _ => println!("Audio not implemented for {} yet", board_generation),
            };
        }
        None => {
            println!("Not a chromebook. Not overlaying Chromebook audio.")
        }
    }
}
