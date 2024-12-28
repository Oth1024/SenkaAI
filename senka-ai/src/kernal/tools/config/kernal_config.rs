use crate::kernal::tools::config::config_traits::config_trait::IConfig;
use crate::kernal::tools::config::config_traits::config_register_trait::IConfigRegister;
use crate::kernal::tools::config::enums::Platform;
pub struct KernalConfig
{
    default_work_directory: &'static str,
    config_file_default_path: &'static str,
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
                default_work_directory: "../../",
                config_file_default_path: "../Configs/",
                platform: Platform::Linux,
            }
        }
        else if self.platform == Platform::Windows 
        {
            KernalConfig
            {
                default_work_directory: "..\\..\\",
                config_file_default_path: "../Configs/",
                platform: Platform::Windows,
            }
        }
        else 
        {
            // not support
            KernalConfig
            {
                default_work_directory: "",
                config_file_default_path: "",
                platform: Platform::NotSupport,
            }
        }
    }

    fn get_config_name(self) -> &'static str 
    {
        "KernalConfig"
    }

    fn get_config_default_path(self) -> &'static str 
    {
        // not implement
        ""
    }

    fn set_config_default_path(self) 
    {
        // not implement
    } 
}