use crate::kernal::tools::config::config_trait::config::IConfig;

pub trait IConfigRegister
{
    fn default_config(self) -> impl IConfig;
    
    fn get_config_name(self) -> String;

    fn get_config_default_path(self) -> String;

    fn set_config_default_path(self);
}