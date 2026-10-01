use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeripheralInterface {
    pub id: String,
    pub group_id: String,
    pub path: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub release_number: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: Option<String>,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
    pub bus_type: String,
    pub kind: String,
    pub vendor_tag: String,
    pub driver_id: String,
    pub driver_status: String,
    pub capabilities: Vec<String>,
    pub hardware_writes_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub platform: String,
    pub interfaces: Vec<PeripheralInterface>,
    pub note: Option<String>,
    pub scanned_at_unix_ms: u128,
}

fn clean_hid_text(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_control() || ch.is_whitespace())
        .collect::<String>()
        .trim()
        .to_string()
}

fn classify(usage_page: u16, usage: u16, product: &str) -> String {
    if usage_page == 0x01 {
        match usage {
            0x02 => return "mouse".into(),
            0x06 => return "keyboard".into(),
            0x04 | 0x05 => return "controller".into(),
            _ => {}
        }
    }

    if usage_page == 0x0c {
        return "media".into();
    }

    let p = product.to_ascii_lowercase();
    if p.contains("keyboard") || p.contains("keypad") {
        "keyboard".into()
    } else if p.contains("mouse")
        || p.contains("trackpad")
        || p.contains("touchpad")
        || p.contains("trackball")
    {
        "mouse".into()
    } else if usage_page >= 0xff00 {
        "vendor".into()
    } else {
        "hid".into()
    }
}

fn vendor_tag(vid: u16, manufacturer: &str) -> String {
    match vid {
        0x05ac => "Apple".into(),
        0x046d => "Logitech".into(),
        0x1532 => "Razer".into(),
        0x1b1c => "Corsair".into(),
        0x1038 => "SteelSeries".into(),
        0x258a if !manufacturer.trim().is_empty() => manufacturer.trim().to_string(),
        0x258a => "VID 258A".into(),
        _ if !manufacturer.trim().is_empty() => manufacturer.trim().to_string(),
        _ => format!("VID {:04X}", vid),
    }
}

fn driver_match(vid: u16, manufacturer: &str, product: &str) -> (String, String) {
    let identity = format!("{} {}", manufacturer, product).to_ascii_lowercase();

    if vid == 0x05ac && identity.contains("magic keyboard") {
        return ("apple-magic-keyboard".into(), "foundation".into());
    }

    if vid == 0x258a && (identity.contains("glorious") || identity.contains("model o")) {
        return ("glorious-model-o".into(), "foundation".into());
    }

    ("generic-hid".into(), "inspect-only".into())
}

fn capabilities(kind: &str) -> Vec<String> {
    let mut result = vec!["inspect".to_string(), "snapshot".to_string()];
    match kind {
        "keyboard" => result.push("input-test".into()),
        "mouse" => result.push("pointer-test".into()),
        "media" => result.push("media-input".into()),
        "controller" => result.push("controller-input".into()),
        _ => {}
    }
    result
}

fn stable_hash(value: &str) -> String {
    // FNV-1a is deterministic and sufficient for non-security local IDs.
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(target_os = "linux")]
fn linux_physical_key(path: &str) -> Option<String> {
    use std::fs;
    use std::path::{Path, PathBuf};

    let node = Path::new(path).file_name()?.to_str()?;
    if !node.starts_with("hidraw") {
        return None;
    }

    let mut current = fs::canonicalize(format!("/sys/class/hidraw/{node}/device")).ok()?;
    loop {
        if current.join("idVendor").exists() && current.join("idProduct").exists() {
            return Some(format!("linux:{}", current.to_string_lossy()));
        }
        let parent: PathBuf = current.parent()?.to_path_buf();
        if parent == current {
            break;
        }
        current = parent;
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn linux_physical_key(_path: &str) -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
fn windows_physical_key(path: &str) -> Option<String> {
    let lowered = path.to_ascii_lowercase();
    let mut parts = lowered.split('#');
    let prefix = parts.next()?;
    let hardware = parts.next()?;
    let instance = parts.next()?;

    let normalized_hardware = hardware
        .split('&')
        .filter(|part| !part.starts_with("mi_") && !part.starts_with("col"))
        .collect::<Vec<_>>()
        .join("&");

    Some(format!("windows:{prefix}#{normalized_hardware}#{instance}"))
}

#[cfg(not(target_os = "windows"))]
fn windows_physical_key(_path: &str) -> Option<String> {
    None
}

fn physical_group_key(
    path: &str,
    vid: u16,
    pid: u16,
    serial: Option<&str>,
    manufacturer: &str,
    product: &str,
) -> String {
    if let Some(serial) = serial.filter(|s| !s.trim().is_empty()) {
        return format!("serial:{vid:04x}:{pid:04x}:{}", serial.trim());
    }

    if let Some(key) = linux_physical_key(path) {
        return key;
    }

    if let Some(key) = windows_physical_key(path) {
        return key;
    }

    // Do not merge identical serial-less devices unless the platform exposes a
    // reliable physical parent. A false split is safer than a false merge.
    format!(
        "fallback:{vid:04x}:{pid:04x}:{}:{}:{path}",
        manufacturer.trim(),
        product.trim()
    )
}

#[cfg(target_os = "linux")]
fn linux_scan_note(interface_count: usize) -> Option<String> {
    use std::fs;

    if interface_count > 0 {
        return None;
    }

    let hidraw_nodes = fs::read_dir("/dev")
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("hidraw"))
        .count();

    if hidraw_nodes > 0 {
        Some(format!(
            concat!(
                "Linux has {hidraw_nodes} hidraw node(s), but HIDAPI returned no interfaces. ",
                "Check device permissions/udev rules if this persists."
            )
        ))
    } else {
        Some(
            "No HID interfaces were discovered. Connect a keyboard or mouse and scan again."
                .into(),
        )
    }
}

#[cfg(not(target_os = "linux"))]
fn linux_scan_note(interface_count: usize) -> Option<String> {
    if interface_count == 0 {
        Some("No HID interfaces were discovered. Connect a keyboard or mouse and scan again.".into())
    } else {
        None
    }
}

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
#[tauri::command]
fn scan_devices() -> Result<ScanResult, String> {
    use hidapi::HidApi;
    use std::collections::HashSet;
    use std::time::{SystemTime, UNIX_EPOCH};

    let api = HidApi::new().map_err(|e| format!("Could not initialize HID API: {e}"))?;
    let mut interfaces = Vec::new();
    let mut seen = HashSet::new();

    for d in api.device_list() {
        let manufacturer = clean_hid_text(d.manufacturer_string().unwrap_or(""));
        let product_raw = clean_hid_text(d.product_string().unwrap_or(""));
        let product = if product_raw.is_empty() {
            "Unknown HID device".to_string()
        } else {
            product_raw
        };
        let usage_page = d.usage_page();
        let usage = d.usage();
        let path = d.path().to_string_lossy().into_owned();
        let serial_number = d
            .serial_number()
            .map(clean_hid_text)
            .filter(|s| !s.is_empty());
        let kind = classify(usage_page, usage, &product);
        let tag = vendor_tag(d.vendor_id(), &manufacturer);
        let (driver_id, status) = driver_match(d.vendor_id(), &manufacturer, &product);
        let bus_type = format!("{:?}", d.bus_type()).to_ascii_lowercase();

        let interface_key = format!(
            "{}:{:04x}:{:04x}:{}:{}:{}",
            path,
            d.vendor_id(),
            d.product_id(),
            d.interface_number(),
            usage_page,
            usage
        );

        // Some platform backends may surface the same interface more than once.
        // Keep the first stable representation so counts and diagnostics stay sane.
        if !seen.insert(interface_key.clone()) {
            continue;
        }

        let group_key = physical_group_key(
            &path,
            d.vendor_id(),
            d.product_id(),
            serial_number.as_deref(),
            &manufacturer,
            &product,
        );
        let group_id = format!("grp-{}", stable_hash(&group_key));

        interfaces.push(PeripheralInterface {
            id: format!("if-{}", stable_hash(&interface_key)),
            group_id,
            path,
            vendor_id: d.vendor_id(),
            product_id: d.product_id(),
            release_number: d.release_number(),
            manufacturer,
            product,
            serial_number,
            usage_page,
            usage,
            interface_number: d.interface_number(),
            bus_type,
            kind: kind.clone(),
            vendor_tag: tag,
            driver_id,
            driver_status: status,
            capabilities: capabilities(&kind),
            hardware_writes_enabled: false,
        });
    }

    interfaces.sort_by(|a, b| {
        a.group_id
            .cmp(&b.group_id)
            .then(a.interface_number.cmp(&b.interface_number))
            .then(a.usage_page.cmp(&b.usage_page))
            .then(a.usage.cmp(&b.usage))
            .then(a.id.cmp(&b.id))
    });

    let note = linux_scan_note(interfaces.len());

    Ok(ScanResult {
        platform: std::env::consts::OS.to_string(),
        interfaces,
        note,
        scanned_at_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default(),
    })
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
#[tauri::command]
fn scan_devices() -> Result<ScanResult, String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    Ok(ScanResult {
        platform: std::env::consts::OS.to_string(),
        interfaces: Vec::new(),
        note: Some("HID scanning is not implemented on this platform yet.".into()),
        scanned_at_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default(),
    })
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverDescriptor {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub family: String,
    pub support_level: String,
    pub read_capabilities: Vec<String>,
    pub write_capabilities: Vec<String>,
    pub notes: String,
}

#[tauri::command]
fn driver_catalog() -> Vec<DriverDescriptor> {
    vec![
        DriverDescriptor {
            id: "generic-hid".into(),
            name: "Generic HID Inspector".into(),
            vendor: "Any".into(),
            family: "Standard HID".into(),
            support_level: "inspect-only".into(),
            read_capabilities: vec!["identify".into(), "interfaces".into(), "diagnostics".into()],
            write_capabilities: vec![],
            notes: "Safe fallback for devices without a model-specific driver.".into(),
        },
        DriverDescriptor {
            id: "apple-magic-keyboard".into(),
            name: "Apple Magic Keyboard".into(),
            vendor: "Apple".into(),
            family: "Magic Keyboard".into(),
            support_level: "foundation".into(),
            read_capabilities: vec!["identify".into(), "interfaces".into(), "diagnostics".into()],
            write_capabilities: vec![],
            notes:
                "Model-specific write operations stay disabled until exact tested reports are documented."
                    .into(),
        },
        DriverDescriptor {
            id: "glorious-model-o".into(),
            name: "Glorious Model O".into(),
            vendor: "Glorious".into(),
            family: "Model O".into(),
            support_level: "foundation".into(),
            read_capabilities: vec!["identify".into(), "interfaces".into(), "diagnostics".into()],
            write_capabilities: vec![],
            notes:
                "Exact revisions must be tested before DPI, polling, debounce, or lighting writes are enabled."
                    .into(),
        },
    ]
}

#[tauri::command]
fn app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "GearPilot",
        "version": env!("CARGO_PKG_VERSION"),
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "phase": concat!(
            "Driver catalog, polished device workflow, portable profiles ",
            "and safe HID discovery"
        ),
        "supportedPlatforms": ["windows", "linux", "macos"],
        "driverCount": 3,
        "hardwareWritesEnabled": false
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_devices, driver_catalog, app_info])
        .run(tauri::generate_context!())
        .expect("error while running GearPilot");
}
