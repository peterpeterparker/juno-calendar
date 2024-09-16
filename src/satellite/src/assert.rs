use junobuild_satellite::AssertDeleteDocContext;
use crate::answers::count_answers;

pub fn assert_no_answers(context: &AssertDeleteDocContext) -> Result<(), String> {
    let count = count_answers(&context.data.key)?;

    if count > 0 {
        Err(format!("Events has {} answer(s).", count))
    } else {
        Ok(())
    }
}