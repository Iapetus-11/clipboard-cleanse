#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum WmCommand {
    MenuToggleAutoStart = 1,
    MenuOpenConfigFile = 2,
    MenuQuit = 3,
}

impl TryFrom<usize> for WmCommand {
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WmCommand::MenuToggleAutoStart),
            2 => Ok(WmCommand::MenuOpenConfigFile),
            3 => Ok(WmCommand::MenuQuit),
            _ => Err(value),
        }
    }
}

impl From<WmCommand> for usize {
    fn from(val: WmCommand) -> Self {
        val as usize
    }
}
