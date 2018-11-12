extern crate toml;

use input::*;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DbCfg {
    pub db_type: Option<String>,
    pub db_uri: Option<String>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CodeGenCfg {
    pub output_dir: String,
    pub project_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OutputCfg {
    pub output_dir: String,
    pub project_name: Option<String>
}


#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Config {
    pub gen_models: Option<bool>,
    pub gen_sql: Option<bool>,
    pub gen_webserver: Option<bool>, 
    pub input_type: InputType,
    pub files: Vec<String>,
    pub directories: Vec<String>,
    pub output: OutputCfg,
    pub output_db: DbCfg,
}

impl Config {

    pub fn get_project_directory_path(self) -> String {
        format!("{}/{}", self.output.output_dir, self.output.project_name.unwrap())
    }

    pub fn get_models_directory_path(self) -> String {
        format!("{}/src/models", self.get_project_directory_path())
    }

    pub fn get_actors_directory_path(self) -> String {
        format!("{}/src/actors", self.get_project_directory_path())
    }

    pub fn get_db_directory_path(self) -> String {
        format!("{}/src/db", self.get_project_directory_path())
    }

    pub fn load(config_str: &str) -> Config {
        match toml::from_str(config_str) {
            Ok(config) => {
                config
            },
            Err(e) => panic!("############################################\n{}", format!("Config ERROR: {}", e))
        }
    }

    pub fn does_project_dir_exist(config: Config) -> Result<bool, String>{
        let project_path_string = config.get_project_directory_path();
        let project_path = Path::new(&project_path_string);
        
        if project_path.exists() {
            if project_path.is_dir() {
                return Ok(true);
            } 
            return Err(format!("The path '{}' provided is a file, not a directory.", project_path_string).clone());  
        } 

        Err(format!("The path '{} does not exist.", project_path_string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn load_input() {
        let test_yaml = r#"
        input_type = 'CSV'
        files = ['a.csv', 'b.csv' ]
        directories = ['~/src/baseballdatabank/core', '~/src/hockeydatabank' ]
        [output]
            output_dir = ".."
            project_name = "baseball_stats_api"

        [output_db]
            db_type = "sqlite"
            db_uri = "./bd_batting.db"
        "#;
        
        let actual = Config::load(test_yaml);

        assert_eq!("CSV", actual.input_type.to_string());
        assert_eq!(2, actual.files.len());
        assert_eq!("a.csv", actual.files[0]);
        assert_eq!("b.csv", actual.files[1]);
        assert_eq!(2, actual.directories.len());
        assert_eq!("~/src/baseballdatabank/core", actual.directories[0]);
        assert_eq!("~/src/hockeydatabank", actual.directories[1]);
    }
}