pub fn get_env_variable(name: String) -> String{
    return std::env::var(name).expect("{name} are not in your .env file!");
}
