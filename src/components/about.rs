use dioxus::prelude::*;

static CARDS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "fool" => "愚者",
    "magician" => "魔术师",
    "high_priestess" => "女祭司",
    "empress" => "女皇",
    "emperor" => "皇帝",
    "hierophant" => "教皇",
    "lovers" => "恋人",
    "chariot" => "战车",
    "strength" => "力量",
    "hermit" => "隐者",
    "wheel_of_fortune" => "命运之轮",
    "justice" => "正义",
    "hanged_man" => "倒吊人",
    "death" => "死神",
    "temperance" => "节制",
    "devil" => "恶魔",
    "tower" => "高塔",
    "star" => "星星",
    "moon" => "月亮",
    "sun" => "太阳",
    "judgement" => "审判",
    "world" => "世界",
};

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "",
            blockquote { "塔罗牌翻起命运的涟漪, 低语着道出某人的名字\n然后, 故事就此开幕" }
            MajorArcana {
                title: "清贫",
                cards: &["justice"],
                span { "与其浊富, 宁比清贫" }
                span { "语出唐代姚崇《冰壶诫》: 与其不义致富, 不如保持美德恪守信条" }
                span { "浊富谓不义致富, 清贫谓穷而恪守底线" }
            }
            MajorArcana {
                title: "苦难",
                cards: &["justice"],
            }
        }
    }
}

#[component]
pub fn MajorArcana(title: &'static str, cards: &'static[&'static str], children: Element) -> Element {
    let cards = cards.iter().flat_map(|card| CARDS.get(card).copied()).collect::<Vec<&'static str>>();
    rsx! {
        div {
            h1 { id: title, Link { to: "about/#清贫", {title} } }
            div { class: "flex flex-col", 
                {children}
            }
        }
    }
}
