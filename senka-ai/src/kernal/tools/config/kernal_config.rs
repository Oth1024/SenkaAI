use crate::kernal::tools::config::config_trait::config::IConfig;
use crate::kernal::tools::config::config_trait::config_register::IDefaultConfig;
pub struct KernalConfig
{

}

impl IConfig for KernalConfig
{
    fn get_config_default_path() -> String 
    {
        
    }
}

struct KernalConfigRegister
{

}

impl IDefaultConfig for KernalConfigRegister 
{
    
}