use async_fs::read_to_string;
use async_process::Command;
use tokio::join;
use toml::Table;

async fn get_unique_hash() -> String {
    String::from_utf8(
        Command::new("/usr/bin/chromebook-unique-id.sh")
            .output()
            .await
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned()
}

pub enum AudioChoice {
    Sof,
    AvsWithoutMax98357a,
    AvsWithMax98357a,
}

async fn get_audio_choices() -> Table {
    let table: Table = read_to_string("/etc/chromebook-audio/audio-choices.toml")
        .await
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    table
}

pub async fn get_audio_choice() -> Option<AudioChoice> {
    let (audio_choices, unique_hash) = join!(get_audio_choices(), get_unique_hash());
    match audio_choices.get(&unique_hash) {
        Some(audio_choice) => Some(match audio_choice.as_str().unwrap() {
            "sof" => AudioChoice::Sof,
            "avs-without-max98357a" => AudioChoice::AvsWithoutMax98357a,
            "avs-with-max98357a" => AudioChoice::AvsWithMax98357a,
            _ => panic!("{}", audio_choice),
        }),
        None => None,
    }
}
