use crate::errors::LoxError;
use crate::interpreter::EnvironmentStack;
use crate::object::LoxObject;

pub fn clock_native(_envs: &mut EnvironmentStack, _args: Vec<LoxObject>) -> Result<LoxObject, LoxError> {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH);

    match time {
        Ok(t) => {
            Ok(LoxObject::Number(t.as_secs_f64()))
        }
        Err(_) => {
            Ok(LoxObject::Nil)
        }
    }

}