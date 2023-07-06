use scraper::Selector;
use scraper::Html;
use indexmap::IndexMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let url = "https://www.worldometers.info/coronavirus/";
    let response = reqwest::blocking::get(url).expect("Could not load URL.");
    let raw_html_string = response.text().unwrap();
    let html_fragment = Html::parse_fragment(&raw_html_string);

    let table_selector_string = "#main_table_countries_today, #main_table_countries_yesterday, #main_table_countries_yesterday2";
    let table_selector = Selector::parse(table_selector_string).unwrap();

    let head_elements_selector = Selector::parse("thead > tr > th").unwrap();
    let row_elements_selector = Selector::parse("tbody > tr").unwrap();
    let row_element_data_selector = Selector::parse("td, th").unwrap();

    let mut tables_data: IndexMap<String, Vec<IndexMap<String, String>>> = IndexMap::new();

    for table in html_fragment.select(&table_selector) {
        let head_elements = table.select(&head_elements_selector);
        let mut head: Vec<String> = Vec::new();

        for head_element in head_elements {
            let element = head_element.text().collect::<Vec<_>>().join(" ");
            let cleaned_element = element.trim().replace("\n", " ");
            head.push(cleaned_element);
        }

        let row_elements = table.select(&row_elements_selector);
        let mut rows: Vec<Vec<String>> = Vec::new();

        for row_element in row_elements {
            let mut row: Vec<String> = Vec::new();
            
            for td_element in row_element.select(&row_element_data_selector) {
                let element = td_element.text().collect::<Vec<_>>().join(" ");
                let cleaned_element = element.trim().replace("\n", " ");
                row.push(cleaned_element);
            }
            
            rows.push(row);
        }

        let mut table_data: Vec<IndexMap<String, String>> = Vec::new();

        for row in rows {
            let zipped_array = head.iter().zip(row.iter()).map(|(a, b)| (a.to_string(), b.to_string())).collect::<Vec<_>>();
            let mut item_hash: IndexMap<String, String> = IndexMap::new();

            for pair in zipped_array {
                if !pair.1.is_empty() {
                    item_hash.insert(pair.0, pair.1);
                }
            }

            table_data.push(item_hash);
        }

        tables_data.insert(table_selector_string.to_string(), table_data);
    }

    let final_table_object = FinalTableObject { tables: tables_data };
    let serialized = serde_json::to_string_pretty(&final_table_object).unwrap();
    let path = "out.json";

    let mut output = File::create(path).unwrap();
    output.write_all(serialized.as_bytes()).unwrap();

    println!("Successfully wrote to {}", path);
}

#[derive(serde::Serialize)]
struct FinalTableObject {
    tables: IndexMap<String, Vec<IndexMap<String, String>>>,
}
