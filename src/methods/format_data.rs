use crate::DataFormat;
use crate::Table;
use crate::Val;
use alloc::borrow::Cow;
use alloc::string::String;

impl Table {
    #[inline(always)]
    pub(crate) fn acr_apply_format(
        fmt_opt: Option<&DataFormat>,
        val: Val,
        normalize: bool,
        ignore_fmt_errs: bool,
    ) -> Result<Val, String> {
        let Some(fmt) = fmt_opt else {
            return Ok(val);
        };

        match fmt.try_format_val(&val, normalize) {
            Ok(Cow::Owned(v)) => Ok(v),
            Ok(Cow::Borrowed(_)) => Ok(val),
            Err(e) => {
                if ignore_fmt_errs {
                    Ok(val)
                } else {
                    Err(e)
                }
            }
        }
    }
}
