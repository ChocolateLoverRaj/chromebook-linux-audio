use tokio::fs::try_exists;

pub async fn has_max98357a() -> bool {
    try_exists("/sys/bus/acpi/devices/MX98357A:00")
        .await
        .unwrap()
}

// pub async fn has_max98357a() -> bool {
//     true
// }
