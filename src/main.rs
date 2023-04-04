use itertools::Itertools;

const MAX_LINE_LENGTH: usize = 19;
const MAX_LINE_COUNT: usize = 14;

const CATEGORY_COLOUR: &str = "#ffaaff";
const MENU_ITEM_COLOUR: &str = "#000000";
const MENU_ITEM_PRICE_COLOUR: &str = "#00ff00";

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct MenuItem {
    category: String,
    menu_item: String,
    sell_price: i32,
}

fn main() {
    let menu_items: Vec<MenuItem> =
        serde_json::from_str(include_str!("../menu_items.json")).unwrap();

    let catagory_menu_items = {
        let mut catagory_menu_items = std::collections::HashMap::new();
        for menu_item in menu_items {
            catagory_menu_items
                .entry(menu_item.category.clone())
                .or_insert_with(Vec::new)
                .push(menu_item);
        }
        catagory_menu_items
    };

    let catagories = catagory_menu_items
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let catagories = inquire::Editor::new("Please put them in the order you want them to appear")
        .with_predefined_text(&format!("{}\nINDEX_PAGE", catagories.join("\n")))
        .prompt()
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut line_count = 0;
    let mut page_count = 0;
    let mut catagory_pages = std::collections::HashMap::new();
    let mut command = "/give @p written_book{pages:[".to_string();
    let page_start = "'[\"\",";
    let page_end = "]',";
    let mut last_was_custom = true;
    for catagory_name in catagories {
        let menu_items = match catagory_menu_items.get(&catagory_name) {
            Some(menu_items) => menu_items,
            None => {
                if catagory_name == "INDEX_PAGE" {
                    if !last_was_custom {
                        command = command.strip_suffix(',').unwrap().to_string();
                        command.push_str(page_end);
                        page_count += 1;
                    }
                    command.push_str(page_start);
                    command.push_str("INDEX_PAGE");
                    command.push_str(page_end);
                    last_was_custom = true;
                } else if catagory_name.trim().is_empty() {
                    continue;
                }
                // Custom page insert
                else {
                    if !last_was_custom {
                        command = command.strip_suffix(',').unwrap().to_string();
                        command.push_str(page_end);
                        page_count += 1;
                    }

                    command.push_str(&catagory_name);

                    command.push(',');
                    page_count += 1;
                    last_was_custom = true;
                }

                continue;
            }
        };

        if last_was_custom {
            command.push_str(page_start);
            line_count = 0;
            last_was_custom = false;
        }

        if line_count + 5 > MAX_LINE_COUNT {
            // println!("NEW PAGE");
            command = command.strip_suffix(',').unwrap().to_string();
            command.push_str(page_end);
            command.push_str(page_start);
            page_count += 1;
            line_count = 0;
        }

        command.push_str(&format!(
            "{{\"text\":\"{}\\\\n\\\\n\",\"color\":\"{}\"}},",
            catagory_name, CATEGORY_COLOUR
        ));
        line_count += 2;

        catagory_pages.insert(catagory_name.clone(), page_count);

        for menu_item in menu_items {
            let line = format!("{}: ¥{}", menu_item.menu_item.trim(), menu_item.sell_price);
            if line.len() > MAX_LINE_LENGTH {
                line_count += 1;
            }
            if line_count > MAX_LINE_COUNT {
                // println!("NEW PAGE");
                command = command.strip_suffix(',').unwrap().to_string();
                command.push_str(page_end);
                command.push_str(page_start);
                page_count += 1;
                line_count = 0;
            }
            command.push_str(&format!(
                "{{\"text\":\"{}: \",\"color\":\"{}\"}},",
                menu_item.menu_item.trim(),
                MENU_ITEM_COLOUR
            ));
            command.push_str(&format!(
                "{{\"text\":\"¥{}\\\\n\",\"color\":\"{}\"}},",
                menu_item.sell_price, MENU_ITEM_PRICE_COLOUR
            ));
            // println!("{}", line);
            line_count += 1;
        }
        // add a new line
        command.push_str(&format!(
            "{{\"text\":\"\\\\n\",\"color\":\"{}\"}},",
            MENU_ITEM_COLOUR
        ));
        line_count += 1;
    }

    let mut index_page = "{\"text\":\"La\",\"italic\":true,\"color\":\"#008C45\"},{\"text\":\" Casa\",\"italic\":true,\"color\":\"gray\"},{\"text\":\" Nostra\",\"italic\":true,\"color\":\"#CD212A\"},{\"text\":\"\\\\n             \",\"color\":\"reset\",\"italic\":true},{\"text\":\"Menu Index\",\"italic\":true,\"color\":\"#355C7D\"},{\"text\":\"\\\\n-=-=-=-=-=-=-=-=-=-\\\\n\",\"color\":\"reset\"}".to_string();
    for (catagory_name, page) in catagory_pages.iter().sorted_by(|a, b| a.1.cmp(b.1)) {
        index_page.push_str(&format!(
            ",{{\"text\":\"{}\",\"color\":\"#51074A\",\"clickEvent\":{{\"action\":\"change_page\",\"value\":{}}}}},{{\"text\":\"\\\\n\",\"color\":\"reset\"}}",
            catagory_name, page+2
        ));
    }
    index_page.push_str(",{\"text\":\"\\\\nRight Click to navigate\",\"color\":\"reset\"}");
    command = command.replace("INDEX_PAGE", &index_page);

    command = command.strip_suffix(',').unwrap().to_string();
    command.push_str("]'],title:\"La Casa Nostra menu\",author:\"zegevlier\"}");

    println!("{}", command);

    println!("Catagory Pages: {:#?}", catagory_pages);
}
