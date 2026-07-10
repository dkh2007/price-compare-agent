use regex::Regex;

use crate::models::product::ParsedIntent;

pub struct RuleEngine {
    categories: Vec<String>,
    features: Vec<String>,
    brands: Vec<String>,
    scenarios: Vec<String>,
}

impl RuleEngine {
    pub fn new(categories: Vec<String>, features: Vec<String>) -> Self {
        Self {
            categories,
            features,
            brands: vec![
                "小米", "华为", "苹果", "OPPO", "vivo", "漫步者", "JBL", "索尼", "Bose",
                "森海塞尔", "铁三角", "Beats", "三星", "联想", "戴尔", "惠普", "华硕",
                "宏碁", "微软", "雷蛇", "罗技", "Cherry", "Filco", "达尔优", "ikbc",
                "杜伽", "AKKO", "Keychron", "佳能", "尼康", "大疆", "科沃斯", "石头",
                "追觅", "云鲸", "飞利浦", "戴森", "松下", "格力", "美的",
                "海尔", "海信", "TCL", "创维", "LG", "雀巢", "德龙", "百胜图",
                "东菱", "柏翠", "极米", "当贝", "明基", "爱普生", "坚果", "峰米",
                "耐克", "阿迪达斯", "安踏", "李宁", "特步", "361°", "优衣库",
                "ZARA", "H&M", "太平鸟", "海澜之家", "森马", "波司登", "雪中飞",
                "雅诗兰黛", "兰蔻", "迪奥", "MAC", "完美日记", "花西子", "资生堂",
                "SK-II", "欧莱雅", "OLAY", "珀莱雅", "薇诺娜", "百雀羚", "自然堂",
                "伊利", "蒙牛", "光明", "三元", "金龙鱼", "福临门", "鲁花",
                "农夫山泉", "怡宝", "百岁山", "青岛", "雪花", "哈尔滨", "百威",
                "南方黑芝麻", "五谷磨房",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            scenarios: vec![
                "运动", "健身", "跑步", "通勤", "办公", "游戏", "电竞", "学习",
                "出差", "旅行", "居家", "日常", "户外", "登山", "骑行", "游泳",
                "商务", "会议", "上课", "网课", "视频会议", "直播", "录音",
                "拍照", "摄影", "美妆", "护肤", "清洁", "做饭", "烹饪", "烘焙",
                "送礼", "送女友", "送男友", "送爸妈", "聚会", "约会", "面试",
                "上班", "上学", "逛街", "休闲", "正装", "秋冬", "春夏",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }

    pub fn parse(&self, input: &str) -> ParsedIntent {
        let input_lower = input.to_lowercase();

        let budget_min = Self::extract_budget_min(&input_lower);
        let budget_max = Self::extract_budget_max(&input_lower);
        let (budget_min, budget_max) = Self::extract_budget_range(&input_lower, budget_min, budget_max);

        let product_name = Self::match_keyword(&input_lower, &self.categories);
        let brand = Self::match_keyword(input, &self.brands);
        let features = Self::match_keywords(input, &self.features);
        let usage_scenario = Self::match_keyword(&input_lower, &self.scenarios);

        let is_complete = product_name.is_some();
        let missing_fields = if is_complete {
            vec![]
        } else {
            vec!["商品类型".into()]
        };

        ParsedIntent {
            product_name,
            brand,
            model: None,
            budget_min,
            budget_max,
            features,
            usage_scenario,
            is_complete,
            missing_fields,
        }
    }

    fn extract_budget_min(input: &str) -> Option<f64> {
        let re = Regex::new(r"(\d+)\s*(?:以上|以上|至少|最低|不低于|大于|高于)").unwrap();
        re.captures(input)
            .and_then(|c| c[1].parse::<f64>().ok())
    }

    fn extract_budget_max(input: &str) -> Option<f64> {
        let re = Regex::new(r"(\d+)\s*(?:元|块)?\s*(?:以内|以下|不超过|最多|最高|低于|小于|以内)").unwrap();
        re.captures(input)
            .and_then(|c| c[1].parse::<f64>().ok())
    }

    fn extract_budget_range(input: &str, def_min: Option<f64>, def_max: Option<f64>) -> (Option<f64>, Option<f64>) {
        let re = Regex::new(r"(\d+)\s*(?:[-~到至])\s*(\d+)").unwrap();
        if let Some(caps) = re.captures(input) {
            let min = caps[1].parse::<f64>().ok().or(def_min);
            let max = caps[2].parse::<f64>().ok().or(def_max);
            return (min, max);
        }
        (def_min, def_max)
    }

    fn match_keyword(input: &str, keywords: &[String]) -> Option<String> {
        keywords
            .iter()
            .find(|kw| {
                let kw_lower = kw.to_lowercase();
                input.contains(&kw_lower)
                    || Self::input_contains_word_in_keyword(input, &kw_lower)
            })
            .cloned()
    }

    fn match_keywords(input: &str, keywords: &[String]) -> Vec<String> {
        keywords
            .iter()
            .filter(|kw| {
                let kw_lower = kw.to_lowercase();
                input.contains(&kw_lower)
                    || Self::input_contains_word_in_keyword(input, &kw_lower)
            })
            .cloned()
            .collect()
    }

    fn input_contains_word_in_keyword(input: &str, keyword: &str) -> bool {
        let word_match = input
            .split(|c: char| !c.is_alphanumeric() && c != '°')
            .filter(|w| w.len() >= 2)
            .any(|w| keyword.contains(w));

        if word_match {
            return true;
        }

        if keyword.chars().count() <= 2 {
            return input.contains(keyword);
        }

        let keyword_chars: std::collections::HashSet<char> = keyword.chars().collect();
        let input_chars: std::collections::HashSet<char> = input.chars().collect();
        let overlap = keyword_chars.intersection(&input_chars).count();
        overlap >= 2 || (keyword_chars.len() <= 3 && overlap >= 1)
    }
}