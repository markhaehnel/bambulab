use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Message {
    Print(Print),
    Info(Info),
    Unknown(Option<String>),

    Connecting,
    Connected,
    Reconnecting,
    Disconnected,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Print {
    pub print: PrintData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintData {
    pub upload: Option<PrintUpload>,
    pub nozzle_temper: Option<f64>,
    pub nozzle_target_temper: Option<i64>,
    pub bed_temper: Option<f64>,
    pub bed_target_temper: Option<i64>,
    pub chamber_temper: Option<i64>,
    pub mc_print_stage: Option<String>,
    pub heatbreak_fan_speed: Option<String>,
    pub cooling_fan_speed: Option<String>,
    pub big_fan1_speed: Option<String>,
    pub big_fan2_speed: Option<String>,
    pub mc_percent: Option<i64>,
    pub mc_remaining_time: Option<i64>,
    pub ams_status: Option<i64>,
    pub ams_rfid_status: Option<i64>,
    pub hw_switch_state: Option<i64>,
    pub spd_mag: Option<i64>,
    pub spd_lvl: Option<i64>,
    pub print_error: Option<i64>,
    pub lifecycle: Option<String>,
    pub wifi_signal: Option<String>,
    pub gcode_state: Option<String>,
    pub gcode_file_prepare_percent: Option<String>,
    pub queue_number: Option<i64>,
    pub queue_total: Option<i64>,
    pub queue_est: Option<i64>,
    pub queue_sts: Option<i64>,
    pub project_id: Option<String>,
    pub profile_id: Option<String>,
    pub task_id: Option<String>,
    pub subtask_id: Option<String>,
    pub subtask_name: Option<String>,
    pub gcode_file: Option<String>,
    pub stg: Option<Vec<Value>>,
    pub stg_cur: Option<i64>,
    pub print_type: Option<String>,
    pub home_flag: Option<i64>,
    pub mc_print_line_number: Option<String>,
    pub mc_print_sub_stage: Option<i64>,
    pub sdcard: Option<bool>,
    pub force_upgrade: Option<bool>,
    pub mess_production_state: Option<String>,
    pub layer_num: Option<i64>,
    pub total_layer_num: Option<i64>,
    pub s_obj: Option<Vec<Value>>,
    pub fan_gear: Option<i64>,
    pub hms: Option<Vec<Value>>,
    pub online: Option<PrintOnline>,
    pub ams: Option<PrintAms>,
    pub ipcam: Option<PrintIpcam>,
    pub vt_tray: Option<PrintVtTray>,
    pub lights_report: Option<Vec<PrintLightsReport>>,
    pub upgrade_state: Option<PrintUpgradeState>,
    pub command: Option<String>,
    pub msg: Option<i64>,
    pub sequence_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintUpload {
    pub status: String,
    pub progress: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintOnline {
    pub ahb: bool,
    pub rfid: bool,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintAms {
    pub ams: Option<Vec<PrintAmsData>>,
    pub ams_exist_bits: Option<String>,
    pub tray_exist_bits: Option<String>,
    pub tray_is_bbl_bits: Option<String>,
    pub tray_tar: Option<String>,
    pub tray_now: Option<String>,
    pub tray_pre: Option<String>,
    pub tray_read_done_bits: Option<String>,
    pub tray_reading_bits: Option<String>,
    pub version: Option<i64>,
    pub insert_flag: Option<bool>,
    pub power_on_flag: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintAmsData {
    pub id: String,
    pub humidity: String,
    pub temp: String,
    pub tray: Vec<PrintTray>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintTray {
    pub id: String,
    pub remain: Option<i64>,
    pub k: Option<f64>,
    pub n: Option<f64>,
    pub tag_uid: Option<String>,
    pub tray_id_name: Option<String>,
    pub tray_info_idx: Option<String>,
    pub tray_type: Option<String>,
    pub tray_sub_brands: Option<String>,
    pub tray_color: Option<String>,
    pub tray_weight: Option<String>,
    pub tray_diameter: Option<String>,
    pub tray_temp: Option<String>,
    pub tray_time: Option<String>,
    pub bed_temp_type: Option<String>,
    pub bed_temp: Option<String>,
    pub nozzle_temp_max: Option<String>,
    pub nozzle_temp_min: Option<String>,
    pub xcam_info: Option<String>,
    pub tray_uuid: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintIpcam {
    pub ipcam_dev: Option<String>,
    pub ipcam_record: Option<String>,
    pub timelapse: Option<String>,
    pub mode_bits: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintVtTray {
    pub id: String,
    pub tag_uid: String,
    pub tray_id_name: String,
    pub tray_info_idx: String,
    pub tray_type: String,
    pub tray_sub_brands: String,
    pub tray_color: String,
    pub tray_weight: String,
    pub tray_diameter: String,
    pub tray_temp: String,
    pub tray_time: String,
    pub bed_temp_type: String,
    pub bed_temp: String,
    pub nozzle_temp_max: String,
    pub nozzle_temp_min: String,
    pub xcam_info: String,
    pub tray_uuid: String,
    pub remain: i64,
    pub k: f64,
    pub n: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintLightsReport {
    pub node: String,
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintUpgradeState {
    pub sequence_id: Option<i64>,
    pub progress: Option<String>,
    pub status: Option<String>,
    pub consistency_request: Option<bool>,
    pub dis_state: Option<i64>,
    pub err_code: Option<i64>,
    pub force_upgrade: Option<bool>,
    pub message: Option<String>,
    pub module: Option<String>,
    pub new_version_state: Option<i64>,
    pub new_ver_list: Option<Vec<Value>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub info: InfoData,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoData {
    pub command: String,
    pub sequence_id: String,
    pub module: Vec<InfoModule>,
    pub result: String,
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoModule {
    pub name: String,
    pub project_name: String,
    pub sw_ver: String,
    pub hw_ver: String,
    pub sn: String,
    pub loader_ver: Option<String>,
    pub ota_ver: Option<String>,
}
