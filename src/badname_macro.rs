
macro_rules! badname1 {
    ($first:ident, $myblock:stmt) => {{
        if let Some(c) = macro_struct.macrochild.properties.$first.as_mut() {
            $myblock
        }
    }};
}

macro_rules! badname2 {
    ($second:ident) => {{
        if let Some(orig) = c.$second.as_mut() {
            if let Ok(orig) = orig.parse::<f32>() {
                c.$second.replace((orig * trans.scaler).to_string());
            }
        }
    }};
}