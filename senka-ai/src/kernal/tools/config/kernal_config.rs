use crate::kernal::tools::config::config_trait::config::IConfig;
use crate::kernal::tools::config::config_trait::config_register::IConfigRegister;
use crate::kernal::tools::config::enums::Platform;
pub struct KernalConfig
{
    default_work_directory: String,
    config_file_default_path: String,
    platform: Platform,
}

impl IConfig for KernalConfig
{

}

struct KernalConfigRegister
{
    platform: Platform,
}

impl KernalConfigRegister
{
    fn new(platform: Platform) -> KernalConfigRegister
    {
        KernalConfigRegister
        {
            platform
        }
    }
}

impl IConfigRegister for KernalConfigRegister 
{
    fn default_config(self) -> impl IConfig 
    {
        if self.platform == Platform::Linux
        {
            KernalConfig
            {
                default_work_directory: String::from("../../"),
                config_file_default_path: String::from("../Configs/"),
                platform: Platform::Linux,
            }
        }
        else if self.platform == Platform::Windows 
        {
            KernalConfig
            {
                default_work_directory: String::from("..\\..\\"),
                config_file_default_path: String::from("../Configs/"),
                platform: Platform::Windows,
            }
        }
        else 
        {
            // not support
            KernalConfig
            {
                default_work_directory: String::from(""),
                config_file_default_path: String:: from(""),
                platform: Platform::NotSupport,
            }
        }
    }

    fn get_config_name(self) -> String 
    {
        String::from("KernalConfig")
    }

    fn get_config_default_path(self) -> String 
    {
        // not implement
        String::from("")
    }

    fn set_config_default_path(self) 
    {
        // not implement
    } 
}