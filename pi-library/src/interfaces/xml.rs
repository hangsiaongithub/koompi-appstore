use crate::structures::xml::*;
use treexml::Element;

impl Application {
    pub fn new(data: Element) -> Application {
        let mut app: Application = Application::default();

        for c in data.children.iter() {
            match c.name.as_ref() {
                "id" => app.id = c.text.clone().unwrap(),
                "name" => match c.attributes.get("xml:lang") {
                    None => {
                        app.name.push(Input {
                            lang: "en_US".to_string(),
                            value: c.text.clone().unwrap(),
                        });
                    }
                    Some(data) => {
                        app.name.push(Input {
                            lang: data.to_string(),
                            value: c.text.clone().unwrap(),
                        });
                    }
                },
                "summary" => match c.attributes.get("xml:lang") {
                    None => {
                        app.summaries.push(Input {
                            lang: "en_US".to_string(),
                            value: c.text.clone().unwrap(),
                        });
                    }
                    Some(data) => {
                        app.summaries.push(Input {
                            lang: data.to_string(),
                            value: c.text.clone().unwrap(),
                        });
                    }
                },
                "description" => match c.attributes.get("xml:lang") {
                    None => {
                        if !c.children.is_empty() {
                            let mut new_description: Description = Description::default();
                            new_description.lang = String::from("en_US");
                            for dom_obj in &c.children {
                                match dom_obj.name.as_ref() {
                                    "ul" => {
                                        let mut new_data: DOM = DOM::default();
                                        for li in dom_obj.children.iter() {
                                            let mut new_li: DOM = DOM::default();
                                            new_li.r#type = li.name.clone().parse().unwrap();
                                            new_li.text = li.text.clone().unwrap();
                                            new_data.chlidren.push(new_li);
                                        }

                                        new_description.data.push(new_data);
                                    }
                                    _ => {
                                        let mut paragraph: DOM = DOM::default();
                                        paragraph.r#type = dom_obj.name.clone().parse().unwrap();
                                        paragraph.text = dom_obj
                                            .text
                                            .clone()
                                            .unwrap_or_else(|| String::from("N/A"));
                                        new_description.data.push(paragraph);
                                    }
                                }
                            }

                            app.descriptions.push(new_description);
                        }
                    }
                    Some(data) => {
                        if !c.children.is_empty() {
                            let mut new_description: Description = Description::default();
                            new_description.lang = String::from(data);
                            for dom_obj in &c.children {
                                match dom_obj.name.as_ref() {
                                    "ul" => {
                                        let mut new_data: DOM = DOM::default();
                                        for li in dom_obj.children.iter() {
                                            let mut new_li: DOM = DOM::default();
                                            new_li.r#type = dom_obj.name.clone().parse().unwrap();
                                            new_li.text = li.text.clone().unwrap();
                                            new_data.chlidren.push(new_li);
                                        }

                                        new_description.data.push(new_data);
                                    }
                                    _ => {
                                        let mut paragraph: DOM = DOM::default();
                                        paragraph.r#type = dom_obj.name.clone().parse().unwrap();
                                        paragraph.text = dom_obj.text.clone().unwrap();
                                        new_description.data.push(paragraph);
                                    }
                                }
                            }

                            app.descriptions.push(new_description);
                        }
                    }
                },
                "screenshots" => {
                    for screenshot in c.children.iter() {
                        let mut shot_group: ScreenShot = ScreenShot::default();
                        for data in screenshot.children.iter() {
                            match data.name.as_ref() {
                                "caption" => {
                                    let caption: Shot = Shot {
                                        r#type: ShotEnum::caption,
                                        text: data.text.clone().unwrap(),
                                    };
                                    shot_group.data.push(caption);
                                }
                                "image" => {
                                    let image: Shot = Shot {
                                        r#type: ShotEnum::image,
                                        text: data
                                            .text
                                            .clone()
                                            .unwrap_or_else(|| String::from("N/A")),
                                    };
                                    shot_group.data.push(image);
                                }
                                _ => continue,
                            }
                        }
                        match screenshot
                            .attributes
                            .get("type")
                            .clone()
                            .unwrap_or(&String::from(""))
                            .as_ref()
                        {
                            "default" => shot_group.r#type = "default".to_string(),
                            _ => shot_group.r#type = "".to_string(),
                        }
                        app.screenshots.push(shot_group)
                    }
                }
                "pkgname" => app.pkgname = c.text.clone().unwrap_or_else(|| String::from("N/A")),
                "categories" => {
                    if !c.children.is_empty() {
                        for child in c.children.iter() {
                            app.categories.push(child.text.clone().unwrap())
                        }
                    }
                }
                "icon" => match c.attributes.get("type") {
                    None => {
                        let mut unknow_icon: Icon = Icon::default();
                        unknow_icon.name = String::from("N/A");
                        app.icons.push(unknow_icon);
                    }
                    Some(data) => match data.as_ref() {
                        "stock" => {
                            let mut stock_icon: Icon = Icon::default();
                            stock_icon.r#type = String::from("stock");
                            stock_icon.name = c.text.clone().unwrap_or_else(|| String::from("N/A"));
                            app.icons.push(stock_icon);
                        }
                        _ => {
                            let mut new_icon: Icon = Icon::default();
                            new_icon.name = c.text.clone().unwrap();
                            new_icon.r#type = c
                                .attributes
                                .get("type")
                                .unwrap()
                                .to_string()
                                .parse()
                                .unwrap();
                            new_icon.width = c.attributes.get("width").unwrap().parse().unwrap();
                            new_icon.height = c
                                .attributes
                                .get("height")
                                .unwrap()
                                .to_string()
                                .parse()
                                .unwrap();

                            app.icons.push(new_icon);
                        }
                    },
                },
                "launchable" => {
                    app.launchable.r#type = c.attributes.get("type").unwrap().to_string();
                    app.launchable.text = c.text.clone().unwrap();
                }
                "mimetypes" => {
                    if !c.children.is_empty() {
                        for child in c.children.iter() {
                            app.mimetypes
                                .push(child.text.clone().unwrap_or_else(|| String::from("N/A")))
                        }
                    }
                }
                "url" => {
                    let mut new_url: Url = Url::default();
                    new_url.r#type = c.attributes.get("type").unwrap().to_string();
                    new_url.text = c.text.clone().unwrap_or_else(|| String::from("N/A"));
                    app.urls.push(new_url);
                }
                "keywords" => match c.attributes.get("xml:lang") {
                    None => {
                        let mut text: Vec<String> = Vec::new();
                        for key in c.children.iter() {
                            let word: String = key.text.clone().unwrap_or_else(|| String::from(""));
                            text.push(word);
                        }
                        app.keywords.push(Keyword {
                            lang: "en_US".to_string(),
                            keys: text,
                        });
                    }
                    Some(data) => {
                        let mut text: Vec<String> = Vec::new();
                        for key in c.children.iter() {
                            let word: String = key.text.clone().unwrap_or_else(|| String::from(""));
                            text.push(word);
                        }
                        app.keywords.push(Keyword {
                            lang: data.to_string(),
                            keys: text,
                        });
                    }
                },
                "releases" => {
                    if !c.children.is_empty() {
                        for r in c.children.iter() {
                            let mut release: Release = Release::default();
                            release.r#type = r
                                .attributes
                                .get("type")
                                .unwrap_or(&String::from(""))
                                .to_string();
                            release.version = r
                                .attributes
                                .get("version")
                                .unwrap_or(&String::from(""))
                                .to_string();
                            release.timestamp = r
                                .attributes
                                .get("timestamp")
                                .unwrap_or(&String::from(""))
                                .to_string();

                            if !r.children.is_empty() {
                                for info in r.children.iter() {
                                    let info_text: String = info
                                        .text
                                        .clone()
                                        .unwrap_or_else(|| String::from("N/A"))
                                        .to_string();
                                    if info_text.is_empty() {
                                        continue;
                                    }
                                    {
                                        release.infos.push(info_text);
                                    }
                                }
                            }
                            app.releases.push(release);
                        }
                    }
                }
                "provides" => {
                    if !c.children.is_empty() {
                        for f in c.children.iter() {
                            let mut provided: Provide = Provide::default();
                            provided.r#type = f.name.clone();
                            provided.text = f.text.clone().unwrap_or_else(|| String::from("N/A"));

                            app.provides.push(provided);
                        }
                    }
                }
                "project_license" => app
                    .project_licenses
                    .push(c.text.clone().unwrap_or_else(|| String::from("N/A"))),
                "developer_name" => {
                    let mut developer: Input = Input::default();
                    match c.attributes.get("xml:lang").as_ref() {
                        Some(data) => {
                            developer.lang = data.to_string();
                            developer.value = c.text.clone().unwrap_or_else(|| String::from("N/A"));
                        }
                        None => {
                            developer.lang = String::from("en_US");
                            developer.value = c.text.clone().unwrap_or_else(|| String::from("N/A"));
                        }
                    }
                    app.developer_name.push(developer);
                }
                "project_group" => app
                    .project_group
                    .push(c.text.clone().unwrap_or_else(|| String::from("N/A"))),
                "languages" => {
                    if !c.children.is_empty() {
                        for lang in c.children.iter() {
                            let mut new_language: Language = Language::default();
                            new_language.name = lang.text.clone().unwrap();
                            new_language.percentage =
                                lang.attributes.get("percentage").unwrap().to_string();
                            app.languages.push(new_language);
                        }
                    }
                }
                _ => continue,
            }
        }
        app
    }
}
