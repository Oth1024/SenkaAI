pub trait IConfig 
{
    fn get_config_name() -> String;

    fn get_config_default_path() -> String;

    fn set_config_default_path() -> String;
}