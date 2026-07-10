use crate::models::product::{ParsedIntent, Product};

/// 加载离线商品数据集
pub fn load_products() -> anyhow::Result<Vec<Product>> {
    let data = include_str!("../../../data/products.json");
    let products: Vec<Product> = serde_json::from_str(data)?;
    Ok(products)
}

const MAX_CANDIDATES: usize = 15;

/// 根据意图粗筛候选商品（简单关键词匹配，控制 token 消耗）
pub fn filter_candidates(products: &[Product], intent: &ParsedIntent) -> Vec<Product> {
    let keywords: Vec<String> = {
        let mut k = vec![];
        if let Some(ref name) = intent.product_name {
            k.push(name.clone());
        }
        if let Some(ref brand) = intent.brand {
            k.push(brand.clone());
        }
        k.extend(intent.features.iter().cloned());
        k
    };

    if keywords.is_empty() {
        return products.iter().take(MAX_CANDIDATES).cloned().collect();
    }

    let mut scored: Vec<(usize, &Product)> = products
        .iter()
        .map(|p| {
            let text = format!(
                "{} {} {} {}",
                p.name, p.category, p.specs, p.features.join(" ")
            );
            let score = keywords
                .iter()
                .filter(|kw| text.contains(kw.as_str()))
                .count();
            (score, p)
        })
        .filter(|(score, _)| *score > 0)
        .collect();

    scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
    scored.into_iter().map(|(_, p)| p.clone()).take(MAX_CANDIDATES).collect()
}

/// 把候选商品列表序列化为 JSON 字符串，喂给 LLM
/// 只保留 LLM 需要的字段，减少 Token 消耗
#[derive(serde::Serialize)]
struct LlmProductView<'a> {
    name: &'a str,
    platform: &'a str,
    price: f64,
    specs: &'a str,
    features: &'a Vec<String>,
}

pub fn products_to_json(products: &[Product]) -> String {
    let views: Vec<LlmProductView> = products
        .iter()
        .map(|p| LlmProductView {
            name: &p.name,
            platform: &p.platform,
            price: p.price,
            specs: &p.specs,
            features: &p.features,
        })
        .collect();
    serde_json::to_string(&views).unwrap_or_else(|_| "[]".into())
}