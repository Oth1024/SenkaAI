pub trait IDefaultConfig
{
    fn default_config() -> impl IConfig;
}