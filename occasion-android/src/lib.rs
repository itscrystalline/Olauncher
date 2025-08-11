use jni::{
    JNIEnv,
    objects::{JClass, JString},
    sys::{jboolean, jstring},
};
use occasion::{config::Config, output_of};

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_olauncher_helper_OccasionNative_outputOf<'j>(
    mut env: JNIEnv<'j>,
    _: JClass<'j>,
    config_json: JString<'j>,
) -> jstring {
    let config_json: String = env
        .get_string(&config_json)
        .expect("cannot get java string")
        .into();
    let config: Result<Config, serde_json::Error> = serde_json::from_str(&config_json);
    match config {
        Ok(c) => env
            .new_string(output_of(&c))
            .expect("cannot create string")
            .into_raw(),
        Err(e) => {
            env.throw_new("java/lang/Exception", format!("{e}"))
                .expect_err("cannot throw exception");
            env.new_string("").expect("cannot create string").into_raw()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_olauncher_helper_OccasionNative_defaultConfig<'j>(
    mut env: JNIEnv<'j>,
    _: JClass<'j>,
) -> jstring {
    match default_config() {
        Ok(s) => env.new_string(s).expect("cannot create string").into_raw(),
        Err(e) => {
            env.throw_new("java/lang/Exception", e)
                .expect_err("cannot throw exception");
            env.new_string("").expect("cannot create string").into_raw()
        }
    }
}

fn default_config() -> Result<String, String> {
    let mut json = serde_json::to_value(Config::default()).map_err(|e| format!("{e}"))?;
    let map_json = json
        .as_object_mut()
        .ok_or("cannot convert json to object??")?;
    let json_pretty = serde_json::to_string(map_json).map_err(|e| format!("{e}"))?;
    Ok(json_pretty)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_olauncher_helper_OccasionNative_validate<'j>(
    mut env: JNIEnv<'j>,
    _: JClass<'j>,
    config_json: JString<'j>,
) -> jboolean {
    let config_json: String = env
        .get_string(&config_json)
        .expect("cannot get java string")
        .into();
    let config: Result<Config, serde_json::Error> = serde_json::from_str(&config_json);
    config.is_ok() as u8
}
