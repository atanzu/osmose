use osmose_identifier::Identifier;

use std::string::String;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;
use std::collections::HashSet;

use log;

use tinyjson;

use osmose_generated::generated_proto::osmose::Decision as Decision;

#[derive(Debug)]
pub struct RulesDatabase {
    db: HashMap<String, HashSet<String>>,
}

impl RulesDatabase {
    pub fn new(path: &std::path::Path) -> RulesDatabase {
        let path = path.canonicalize().unwrap();
        let mut file = File::open(&path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        log::info!{"Use rules file: {:?}", &path};

        let mut d = HashMap::<String, HashSet<String>>::new();

        let rules: tinyjson::JsonValue = data.parse().unwrap();

        let arr: &Vec<_> = rules.get().expect("Array value");
        for entry in arr.iter() {
            let source_name: &String = entry["source"]["name"].get().unwrap();
            let destinations: &Vec<_> = entry["destinations"]
                .get()
                .expect("Destinations should be an array");
            let destinations_set: HashSet<String> = destinations
                .iter()
                .map(|x| x["name"].get::<String>().unwrap().to_string())
                .collect();

            d.insert(source_name.to_string(), destinations_set);
        }

        return RulesDatabase { db: d };
    }

    pub fn is_call_allowed(&self, from: &Identifier, to: &Identifier) -> Decision {
        match self.db.get(from.get_name()) {
            Some(source) => {
                if source.contains(to.get_name()) {
                    Decision::ALLOW
                } else {
                    Decision::DISALLOWED_DESTINATION
                }
            }
            None => Decision::SOURCE_UNKNOWN,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use osmose_identifier::Identifier;
    use crate::RulesDatabase;
    use osmose_generated::generated_proto::osmose::Decision as Decision;

    fn get_test_config() -> String {
        r#"
[
    {
        "source": {
            "name": "process1"
        },
        "destinations": [
            {
                "name": "process2"
            },
            {
                "name": "process3"
            }
        ]
    },
    {
        "source": {
            "name": "process2"
        },
        "destinations": [
            {
                "name": "process1"
            }
        ]
    }
]
            "#.to_owned()
    }

    fn set_up(config_name: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(config_name)?;
        file.write_all(get_test_config().as_bytes())?;
        Ok(())
    }

    fn tear_down(config_name: &str) -> std::io::Result<()> {
        std::fs::remove_file(config_name)?;
        Ok(())
    }

    fn run_test<T>(test: T, config_name: &str) -> ()
    where T: FnOnce() -> () + std::panic::UnwindSafe
    {
        set_up(config_name)
            .expect("Cannot create test configuration file");
        let result = std::panic::catch_unwind(|| {
            test()
        });
        tear_down(config_name)
            .expect("Cannot remove test configuration file");
        assert!(result.is_ok())
    }

    #[test]
    fn test_create_db() {
        let config_name = "test_create_db_cfg.json";
        run_test(|| {
            let _ = RulesDatabase::new(std::path::Path::new(config_name));
        }, config_name);
    }

    #[test]
    fn test_allowed() {
        let config_name = "test_allowed_cfg.json";
        run_test(|| {
            let id1 = Identifier::from_given("process1", 111);
            let id2 = Identifier::from_given("process2", 222);
            let id3 = Identifier::from_given("process3", 333);
            let db = RulesDatabase::new(std::path::Path::new(config_name));
            assert_eq!(db.is_call_allowed(&id1, &id2), Decision::ALLOW);
            assert_eq!(db.is_call_allowed(&id1, &id3), Decision::ALLOW);
            assert_eq!(db.is_call_allowed(&id2, &id1), Decision::ALLOW);
        }, config_name);
    }

    #[test]
    fn test_not_allowed() {
        let config_name = "test_not_allowed_cfg.json";
        run_test(|| {
            let id1 = Identifier::from_given("process1", 111);
            let id2 = Identifier::from_given("process2", 222);
            let id3 = Identifier::from_given("process3", 333);
            let id4 = Identifier::from_given("process4", 444);
            let db = RulesDatabase::new(std::path::Path::new(config_name));
            assert_eq!(
                db.is_call_allowed(&id1, &id4),
                Decision::DISALLOWED_DESTINATION);
            assert_eq!(
                db.is_call_allowed(&id2, &id3),
                Decision::DISALLOWED_DESTINATION);
            assert_eq!(
                db.is_call_allowed(&id3, &id1),
                Decision::SOURCE_UNKNOWN);
            assert_eq!(
                db.is_call_allowed(&id4, &id1),
                Decision::SOURCE_UNKNOWN);
            assert_eq!(
                db.is_call_allowed(&id4, &id3),
                Decision::SOURCE_UNKNOWN);
        }, config_name);
    }
}
