use stress_test_generator::utils::read_scenarios;

#[test]
fn shipped_scenario_files_parse() {
    let mut files = 0;
    for entry in std::fs::read_dir("data").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|ext| ext == "csv") {
            let scenarios = read_scenarios(path.to_str().unwrap())
                .unwrap_or_else(|err| panic!("{}: {err}", path.display()));
            assert!(!scenarios.is_empty(), "{} has no scenario", path.display());
            files += 1;
        }
    }
    assert!(files > 0, "no scenario file found");
}
