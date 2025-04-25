use dioxus::prelude::*;
use ferlory_components::*;

#[derive(Copy, Clone)]
struct Card {
    index: u8,
    chinese: &'static str,
    english: &'static str,
    intro: &'static str,
}

impl Card {
    #[rustfmt::skip]
    const fn new(index: u8) -> Self {
        match index {
             0_u8 => Card { index:  0, chinese: "愚者",     english: "fool",             intro: "愚者是旅程的起点, 象征自由, 冒险与无限可能" },
             1_u8 => Card { index:  1, chinese: "魔术师",   english: "magician",         intro: "魔术师代表意志与创造, 是 manifest 的起点" },
             2_u8 => Card { index:  2, chinese: "女祭司",   english: "high_priestess",   intro: "女祭司象征直觉与潜意识, 是内在智慧的守护者"},
             3_u8 => Card { index:  3, chinese: "皇后",     english: "empress",          intro: "皇后是丰饶与感性的象征, 掌管生命力与关怀"},
             4_u8 => Card { index:  4, chinese: "皇帝",     english: "emperor",          intro: "皇帝代表秩序/权威/理性, 是稳定与规则的化身" },
             5_u8 => Card { index:  5, chinese: "教皇",     english: "hierophant",       intro: "教皇象征传统, 信仰与智慧的传承者" },
             6_u8 => Card { index:  6, chinese: "恋人",     english: "lovers",           intro: "恋人牌关于选择, 关系与爱的融合"},
             7_u8 => Card { index:  7, chinese: "战车",     english: "chariot",          intro: "战车代表意志的驱动与前进的胜利" },
             8_u8 => Card { index:  8, chinese: "力量",     english: "strength",         intro: "力量并非暴力, 而是温柔与坚毅的结合"},
             9_u8 => Card { index:  9, chinese: "隐者",     english: "hermit",           intro: "隐者是追求真理的独行者, 象征内省与启示"},
            10_u8 => Card { index: 10, chinese: "命运之轮", english: "wheel_of_fortune", intro: "命运之轮提示循环, 机遇与无法掌控的变化" },
            11_u8 => Card { index: 11, chinese: "正义",     english: "justice",          intro: "正义是因果的法则, 象征公平, 责任与真理" },
            12_u8 => Card { index: 12, chinese: "倒吊人",   english: "hanged_man",       intro: "倒吊人代表牺牲与换位思考, 追求更高视角" },
            13_u8 => Card { index: 13, chinese: "死神",     english: "death",            intro: "死神不等于终结, 而是深层变革与重生的开始" },
            14_u8 => Card { index: 14, chinese: "节制",     english: "temperance",       intro: "节制是调和与平衡, 是不同力量的融合之道" },
            15_u8 => Card { index: 15, chinese: "恶魔",     english: "devil",            intro: "恶魔是执念, 欲望与自我束缚的象征" },
            16_u8 => Card { index: 16, chinese: "高塔",     english: "tower",            intro: "高塔意味着突如其来的崩塌, 唤醒与重建的前奏" },
            17_u8 => Card { index: 17, chinese: "星星",     english: "star",             intro: "星星是希望与疗愈的光芒, 引导你走出黑夜" },
            18_u8 => Card { index: 18, chinese: "月亮",     english: "moon",             intro: "月亮象征幻觉, 情绪与潜藏的未知" },
            19_u8 => Card { index: 19, chinese: "太阳",     english: "sun",              intro: "太阳是喜悦, 能量与真理的光照" },
            20_u8 => Card { index: 20, chinese: "审判",     english: "judgement",        intro: "审判代表觉醒, 复活与心灵的审视" },
            21_u8 => Card { index: 21, chinese: "世界",     english: "world",            intro: "世界是旅程的圆满, 整合与自由的终章" },
            _ => unreachable!()
        }

    }

    const fn next(&self, nth: u8) -> Self {
        Card::new((self.index + 22 + nth) % 22)
    }

    const fn prev(&self, nth: u8) -> Self {
        Card::new((self.index + 22 - nth) % 22)
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        H1 { text: "塔罗牌" }
        blockquote { "命运的涟漪被不断激起\n编织者低语道出「阿尔卡纳」之名\n至此, 一个个故事已然掀开了序幕" }
        Arcana { }

        H1 { id: "清贫", text: "以『清贫』之名" }
        Qing {}

        H1 { id: "钢铁意志", text: "加缪式的钢铁意志\n& Cosplay堂吉柯德" }
        IronWill {}

        H1 { id: "galgame", text: "论美少女恋爱游戏" }
        Galgame {}
    }
}

#[component]
fn Arcana() -> Element {
    let random_index = rand::random_range(0_u8..=21_u8);
    let mut card_signal = use_signal(move || Card::new(random_index));
    let card = *card_signal.read();

    rsx! {
        div {
            class: "border-2 border-cyan-600 px-2 pt-2 pb-1 flex flex-col w-fit",
            class: "sm:flex-row sm:w-fit sm:mx-auto",
            img {
                class: "w-full sm:h-150 sm:w-auto",
                ontouchend: move |_data| *card_signal.write() = card.next(1),
                src: format!("/assets/images/arcana/bilibili/{}.avif", card.english)
            }
            div { class: "flex flex-col w-full",
                div { class: "text-4xl mx-auto",
                    button {
                        class: "inline-block text-4xl! text-purple-400",
                        class: "transition duration-200 ease-in-out hover:scale-130",
                        onclick: move |_data| *card_signal.write() = card.prev(1),
                        "<-"
                    }
                    "{card.chinese}"
                    button {
                        class: "inline-block text-4xl! text-purple-400",
                        class: "transition duration-200 ease-in-out hover:scale-130",
                        onclick: move |_data| *card_signal.write() = card.next(1),
                        "->"
                    }
                }
                div {
                    class: "w-fit mx-auto text-sm italic",
                    class: "sm:text-2xl",
                    {card.intro}
                }
                div {
                    class: "hidden",
                    class: "sm:flex sm:flex-row mt-auto *:mt-auto",
                    img { class: "h-120",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next(1).english)
                    }
                    img { class: "h-90 max-[50rem]:hidden",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next(2).english)
                    }
                    img { class: "h-65 max-[55rem]:hidden",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next(3).english)
                    }
                    img { class: "h-40 max-[60rem]:hidden",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next(4).english)
                    }
                    img { class: "h-25 max-[65rem]:hidden",
                        src: format!("/assets/images/arcana/bilibili/{}.avif", card.next(5).english)
                    }
                }
            }
        }
    }
}

#[component]
fn Qing() -> Element {
    rsx! {
        div {
            blockquote { "宁可清贫自乐, 不作浊富多忧" }
            "我的网名之一是『清贫』, 还记得当时看的一本叫作《剑娘》的小说里面, 清贫剑真的超级可爱!" br {}
            "与此同时, 这也来自一句诗: \"与其浊富, 宁比清贫\"" br {}
            br {}
            "我厌恶世间许许多多的不正不义" br {}
            br {}
            "倘若污浊沾染我身, 那我想必会很难过很屈辱, 然后奋起反抗吧" br {}
            "亮丽且卑劣, 蒙尘且荣耀, 但我永远也做不到纯粹的二选一" br {}
            "怀着向往荣誉之心, 尽量努力让自己避免卑劣, 仅此而已" br {}
            br {}
            "曾经..." br {}
            "我和你一样, 也是个冒险者..." br {}
            "『直到我的膝盖中了一剑』" br {}
            br {}
            "以『清贫』之名, 我仍存在于此时此刻"
        }
    }
}

#[component]
fn IronWill() -> Element {
    rsx! {
        div {
            blockquote { "The struggle itself toward the heights is enough to fill a man’s heart. One must imagine Sisyphus happy.
                                ————Albert Camus, The Myth of Sisyphus" }
            blockquote { "于攀登中的挣扎本身就足以充实人的心灵, 我们必须想象西西弗斯是幸福的
                                ————加缪,《西西弗斯》" }
            "尽管世界荒诞, 空洞, 虚无, 毫无意义" br {}
            "但西西弗斯, 他选择了反抗, 选择了继续推石头" br {}
            "这就是人类对荒诞的回应与自我胜利" br {}
            br {}
            "人无法像神一般永不疲倦, 人总是会累的" br {}
            "所以呐,『钢铁意志』, 启动! ! !"
        }
    }
}

#[component]
fn Galgame() -> Element {
    rsx! {
        div {
            blockquote { "诸君, 我喜欢玩美少女恋爱游戏呐! !(?" }
            "我喜欢 galgame" br {}
            "我喜欢幼驯染线" br {}
            "我喜欢青梅竹马不战而败的展开" br {}
            "我喜欢告白失败转生修罗场" br {}
            "我喜欢日常温柔推进" br {}
            "我喜欢病娇崩坏暴走" br {}
            "我喜欢 ntr 的心碎" br {}
            "我喜欢 be 的绝望" br {}
            "我喜欢从存档里拯救她的执念" br {}
            "我爱那千回百转的选择分支" br {}
            br {}
            "我喜欢 GALGAME" br {}
            br {}
            "我喜欢每一次点击的紧张" br {}
            "喜欢通宵刷文本的沉浸" br {}
            "喜欢全CG收集时的满足" br {}
            "喜欢通关后空虚的余韵" br {}
            br {}
            "它不是游戏" br {}
            "是命运的模拟器" br {}
            br {}
            "————《The Ultimate Anthem of GALGAME》"
        }
    }
}
