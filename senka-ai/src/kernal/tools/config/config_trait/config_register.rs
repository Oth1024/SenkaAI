use crate::kernal::tools::config::config_trait::config::IConfig;

pub trait IDefaultConfig
{
    fn default_config() -> impl IConfig;
}