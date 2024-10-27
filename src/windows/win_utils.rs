use windows::core::PCWSTR;

/// Takes a &str and returns a fixed-length null terminated array of u16s, useful for some win32 apis
pub fn str_to_u16_nul_term_array<const N: usize>(str: &str) -> Result<[u16; N], String> {
    let mut str_data = str.encode_utf16().collect::<Vec<u16>>();

    let str_data_len = str_data.len();

    if str_data_len >= N {
        return Err(format!(
            "Cannot fit string data of size {str_data_len} into array of size {N}"
        ));
    }

    str_data.resize(N, 0);

    str_data
        .try_into()
        .map(|ba: Box<[u16; N]>| *ba)
        .map_err(|_| format!("Cannot coerce str into array of size {N}"))
}

pub struct RetainedPCWSTR {
    pub value: PCWSTR,
    _encoded: Vec<u16>,
}

pub fn str_as_pcwstr(str: &str) -> RetainedPCWSTR {
    let encoded = str.encode_utf16().chain([0]).collect::<Vec<u16>>();

    RetainedPCWSTR {
        value: PCWSTR(encoded.as_ptr()),
        _encoded: encoded,
    }
}
