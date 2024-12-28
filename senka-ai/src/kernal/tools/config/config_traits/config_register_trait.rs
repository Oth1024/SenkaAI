use crate::kernal::tools::config::config_traits::config_trait::IConfig;

pub trait IConfigRegister<'struct_life_time>
{
    fn default_config(self) -> impl IConfig;
    
    fn get_config_name(self) -> &'struct_life_time str;

    fn get_config_default_path(self) -> &'struct_life_time str;

    fn set_config_default_path(self);
}