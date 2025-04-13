use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Card {
    index: u8,
    chinese_name: &'static str,
    english_name: &'static str,
}

impl Card {
    const fn new(index: u8) -> Self {
        match index {
             0_u8 => Card { index:  0, chinese_name: "愚者",     english_name: "fool" },
             1_u8 => Card { index:  1, chinese_name: "魔术师",   english_name: "magician" },
             2_u8 => Card { index:  2, chinese_name: "女祭司",   english_name: "high_priestess" },
             3_u8 => Card { index:  3, chinese_name: "皇后",     english_name: "empress" },
             4_u8 => Card { index:  4, chinese_name: "皇帝",     english_name: "emperor" },
             5_u8 => Card { index:  5, chinese_name: "教皇",     english_name: "hierophant" },
             6_u8 => Card { index:  6, chinese_name: "恋人",     english_name: "lovers" },
             7_u8 => Card { index:  7, chinese_name: "战车",     english_name: "chariot" },
             8_u8 => Card { index:  8, chinese_name: "力量",     english_name: "strength" },
             9_u8 => Card { index:  9, chinese_name: "隐者",     english_name: "hermit" },
            10_u8 => Card { index: 10, chinese_name: "命运之轮", english_name: "wheel_of_fortune" },
            11_u8 => Card { index: 11, chinese_name: "正义",     english_name: "justice" },
            12_u8 => Card { index: 12, chinese_name: "倒吊人",   english_name: "hanged_man" },
            13_u8 => Card { index: 13, chinese_name: "死神",     english_name: "death" },
            14_u8 => Card { index: 14, chinese_name: "节制",     english_name: "temperance" },
            15_u8 => Card { index: 15, chinese_name: "恶魔",     english_name: "devil" },
            16_u8 => Card { index: 16, chinese_name: "高塔",     english_name: "tower" },
            17_u8 => Card { index: 17, chinese_name: "星星",     english_name: "star" },
            18_u8 => Card { index: 18, chinese_name: "月亮",     english_name: "moon" },
            19_u8 => Card { index: 19, chinese_name: "太阳",     english_name: "sun" },
            20_u8 => Card { index: 20, chinese_name: "审判",     english_name: "judgement" },
            21_u8 => Card { index: 21, chinese_name: "世界",     english_name: "world" },
            _ => unreachable!()
        }

    }

    const fn intro(&self) -> &'static str {
        match self.index {
             0 => "愚者是旅程的起点, 象征自由, 冒险与无限可能",
             1 => "魔术师代表意志与创造, 是 manifest 的起点",
             2 => "女祭司象征直觉与潜意识, 是内在智慧的守护者",
             3 => "皇后是丰饶与感性的象征, 掌管生命力与关怀",
             4 => "皇帝代表秩序/权威/理性, 是稳定与规则的化身",
             5 => "教皇象征传统, 信仰与智慧的传承者",
             6 => "恋人牌关于选择, 关系与爱的融合",
             7 => "战车代表意志的驱动与前进的胜利",
             8 => "力量并非暴力, 而是温柔与坚毅的结合",
             9 => "隐者是追求真理的独行者, 象征内省与启示",
            10 => "命运之轮提示循环, 机遇与无法掌控的变化",
            11 => "正义是因果的法则, 象征公平, 责任与真理",
            12 => "倒吊人代表牺牲与换位思考, 追求更高视角",
            13 => "死神不等于终结, 而是深层变革与重生的开始",
            14 => "节制是调和与平衡, 是不同力量的融合之道",
            15 => "恶魔是执念, 欲望与自我束缚的象征",
            16 => "高塔意味着突如其来的崩塌, 唤醒与重建的前奏",
            17 => "星星是希望与疗愈的光芒, 引导你走出黑夜",
            18 => "月亮象征幻觉, 情绪与潜藏的未知",
            19 => "太阳是喜悦, 能量与真理的光照",
            20 => "审判代表觉醒, 复活与心灵的审视",
            21 => "世界是旅程的圆满, 整合与自由的终章",
            _  => "未知之牌, 可能穿越了维度之外",
        }
    }

    const fn next(&self) -> Self {
        Card::new((self.index + 22 + 1) % 22)
    }

    const fn next_n(&self, n: u8) -> Self {
        Card::new((self.index + 22 + n) % 22)
    }

    const fn prev(&self) -> Self {
        Card::new((self.index + 22 -1) % 22)
    }
}

// #[rustfmt::skip]
// static CARDS: phf::Map<u8, Card> = phf::phf_map! {
//      0_u8 => Card::new( 0, "愚者",     "fool"),
//      1_u8 => Card::new( 1, "魔术师",   "magician"),
//      2_u8 => Card::new( 2, "女祭司",   "high_priestess"),
//      3_u8 => Card::new( 3, "皇后",     "empress"),
//      4_u8 => Card::new( 4, "皇帝",     "emperor"),
//      5_u8 => Card::new( 5, "教皇",     "hierophant"),
//      6_u8 => Card::new( 6, "恋人",     "lovers"),
//      7_u8 => Card::new( 7, "战车",     "chariot"),
//      8_u8 => Card::new( 8, "力量",     "strength"),
//      9_u8 => Card::new( 9, "隐者",     "hermit"),
//     10_u8 => Card::new(10, "命运之轮", "wheel_of_fortune"),
//     11_u8 => Card::new(11, "正义",     "justice"),
//     12_u8 => Card::new(12, "倒吊人",   "hanged_man"),
//     13_u8 => Card::new(13, "死神",     "death"),
//     14_u8 => Card::new(14, "节制",     "temperance"),
//     15_u8 => Card::new(15, "恶魔",     "devil"),
//     16_u8 => Card::new(16, "高塔",     "tower"),
//     17_u8 => Card::new(17, "星星",     "star"),
//     18_u8 => Card::new(18, "月亮",     "moon"),
//     19_u8 => Card::new(19, "太阳",     "sun"),
//     20_u8 => Card::new(20, "审判",     "judgement"),
//     21_u8 => Card::new(21, "世界",     "world")
// };

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "h-2000",
            blockquote { "命运的涟漪被不断激起\n编织者低语道出「阿尔卡纳」之名\n至此, 一个个故事已然掀开了序幕" }

            RandomMajorArcana { }
            
            // MajorArcana {
            //     title: "清贫",
            //     cards: &["justice"],
            //     span { "与其浊富, 宁比清贫" }
            //     span { "语出唐代姚崇《冰壶诫》: 与其不义致富, 不如保持美德恪守信条" }
            //     span { "浊富谓不义致富, 清贫谓穷而恪守底线" }
            // }
        }
    }
}

#[component]
fn RandomMajorArcana() -> Element {
    let random_index = rand::random_range(0_u8..=21_u8);
    let mut card_signal = use_signal(move ||  Card::new(random_index));
    let card = *card_signal.read();
    
   
    rsx! {
        div {
            class: "border-3 border-slate-400 flex flex-col w-fit",
            class: "sm:flex-row sm:w-auto sm:mx-8",
            img {
                class: "w-full sm:h-180 sm:w-auto",
                ontouchend: move |_data| *card_signal.write() = card.next(),
                src: format!("/assets/images/arcana/bilibili/{}.avif", card.english_name)
            }
            div { class: "flex flex-col",
                div { class: "text-4xl mx-auto",
                    button {
                        class: "inline-block text-4xl! text-sky-300",
                        class: "transition duration-200 ease-in-out hover:scale-130",
                        onclick: move |_data| *card_signal.write() = card.prev(),
                        "<-"
                    }
                    {card.chinese_name}
                    button {
                        class: "inline-block text-4xl! text-sky-300",
                        class: "transition duration-200 ease-in-out hover:scale-130",
                        onclick: move |_data| *card_signal.write() = card.next(),
                        "->"
                    }
                    div {
                        class: "w-fit mx-auto text-sm",
                        class: "sm:text-xl",
                        {card.intro()}
                    }
                }
                div {
                    class: "hidden",
                    class: "sm:flex sm:flex-row mt-auto *:mt-auto",
                    img {
                        class: "h-130",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next_n(1).english_name)
                    }
                    img {
                        class: "h-90",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next_n(2).english_name)
                    }
                    img {
                        class: "h-60",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next_n(3).english_name)
                    }
                    img {
                        class: "h-30",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next_n(4).english_name)
                    }
                }
            }
            // for (_index, card) in CARDS {
            //     div { class: "h-120 w-fit flex flex-row m-4", 
            //         img {class: "h-120",  src: format!("/assets/images/arcana/bilibili/{}.avif", card.english_name)}
            //         div { class: "flex flex-col",
            //             "{card.chinese_name}"
            //             "{card.intro()}"
            //         }
            //     }
            // }

        }
    }
}
