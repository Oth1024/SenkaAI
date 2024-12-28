use std::collections::HashMap;
use crate::kernal::tools::config::config_traits::config_trait::IConfig;
use crate::kernal::tools::config::config_traits::config_register_trait::IConfigRegister;

pub struct ConfigManager
{
    registed_configs: HashMap<&'static str, &'static dyn IConfig>,
}

impl ConfigManager
{
    pub fn new() -> Self
    {
        ConfigManager
        {
            registed_configs: HashMap::<&str, &'static dyn IConfig>::new()
        }
    }

    pub fn regist_config(config_register: impl IConfigRegister)
    {

    }

    pub fn import_config()
    {

    }

    pub fn save_config()
    {

    }

    pub fn read_config()
    {

    }

    fn assert_config_not_registed(config_name: &str) -> bool
    {
        // wait to be implement
        true
    }
}