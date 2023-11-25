pub enum Command {
    /// Get the version of the printer.
    GetVersion,
    /// Pause the current print.
    Pause,
    /// Resume the current print.
    Resume,
    /// Stop the current print.
    Stop,
    /// Get all device information.
    PushAll,
    StartPush,
    SetChamberLight(bool),
    SetSpeedProfile(String),
    SendGCodeTemplate(String),
    GetAccessories,
}

impl Command {
    pub(crate) fn get_payload(&self) -> String {
        match self {
            Self::GetVersion => GET_VERSION_PAYLOD.into(),
            Self::Pause => PAUSE_PAYLOAD.into(),
            Self::Resume => RESUME_PAYLOAD.into(),
            Self::Stop => STOP_PAYLOAD.into(),
            Self::PushAll => PUSHALL_PAYLOAD.into(),
            Self::StartPush => START_PUSH_PAYLOAD.into(),
            Self::SetChamberLight(on) => {
                SET_CHAMBER_LIGHT_PAYLOAD.replace("<LED_STATUS>", if *on { "on" } else { "off" })
            }
            Self::SetSpeedProfile(profile) => {
                SET_SPEED_PROFILE_PAYLOAD.replace("<PROFILE>", profile)
            }
            Self::SendGCodeTemplate(gcode) => SEND_GCODE_TEMPLATE_PAYLOAD.replace("<GCODE>", gcode),
            Self::GetAccessories => GET_ACCESSORIES_PAYLOAD.into(),
        }
    }
}

static GET_VERSION_PAYLOD: &str = r#"{"info": {"sequence_id": "0", "command": "get_version"}}"#;
static PAUSE_PAYLOAD: &str = r#"{"print": {"sequence_id": "0", "command": "pause"}}"#;
static RESUME_PAYLOAD: &str = r#"{"print": {"sequence_id": "0", "command": "resume"}}"#;
static STOP_PAYLOAD: &str = r#"{"print": {"sequence_id": "0", "command": "stop"}}"#;
static PUSHALL_PAYLOAD: &str = r#"{"pushing": {"sequence_id": "0", "command": "pushall"}}"#;
static START_PUSH_PAYLOAD: &str = r#"{"pushing": {"sequence_id": "0", "command": "start"}}"#;
static SET_CHAMBER_LIGHT_PAYLOAD: &str = r#"{"system": {"sequence_id": "0", "command": "ledctrl", "led_node": "chamber_light", "led_mode": "<LED_STATUS>", "led_on_time": 500, "led_off_time": 500, "loop_times": 0, "interval_time": 0}}"#;
static SET_SPEED_PROFILE_PAYLOAD: &str =
    r#"{"print": {"sequence_id": "0", "command": "print_speed", "param": "<PROFILE>"}}"#;
static SEND_GCODE_TEMPLATE_PAYLOAD: &str =
    r#"{"print": {"sequence_id": "0", "command": "gcode_line", "param": "<GCODE>"}}"#;
static GET_ACCESSORIES_PAYLOAD: &str =
    r#"{"system": {"sequence_id": "0", "command": "get_accessories", "accessory_type": "none"}}"#;
