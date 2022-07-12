use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::LanguageID;

pub const GEN7_ZH_RAW: &str = "蛋妙蛙种子草花小火龙恐喷杰尼龟卡咪水箭绿毛虫铁甲蛹巴大蝶独角壳针蜂波比鸟拉达烈雀嘴阿柏蛇怪皮丘雷穿山鼠王多兰娜后朗力诺可西六尾九胖丁超音蝠走路臭霸派斯特球摩鲁蛾地三喵猫老鸭哥猴暴蒂狗风速蚊香蝌蚪君泳士凯勇基胡腕豪喇叭芽口呆食玛瑙母毒刺拳石隆岩马焰兽磁合一葱嘟利海狮白泥舌贝鬼通耿催眠貘引梦人钳蟹巨霹雳电顽弹椰树嘎啦飞腿郎快头瓦双犀牛钻吉蔓藤袋墨金鱼星宝魔墙偶天螳螂迷唇姐击罗肯泰鲤普百变伊布边菊化盔镰刀翼急冻闪你哈克幻叶月桂竺葵锯鳄蓝立咕夜鹰芭瓢安圆丝蛛叉字灯笼古然咩羊茸美丽露才皇毽棉长手向日蜻蜓乌沼太阳亮黑暗鸦妖未知图腾果翁麒麟奇榛佛托土弟蝎钢千壶赫狃熊圈熔蜗猪珊瑚炮章桶信使翅戴加象顿Ⅱ惊鹿犬无畏战舞娃奶罐幸福公炎帝幼沙班洛亚凤时木守宫森林蜥蜴稚鸡壮跃狼纹直冲茧狩猎盾粉莲童帽乐河橡实鼻狡猾傲骨燕鸥莉奈朵溜糖雨蘑菇斗笠懒獭过动猿请假居忍面者脱妞吼爆幕下掌朝北优雅勾魂眼那恰姆落正拍负萤甜蔷薇溶吞牙鲨鲸驼煤炭跳噗晃斑颚蚁漠仙歌青绵七夕鼬斩饭匙鳅鲶虾兵螯秤念触摇篮羽丑纳飘浮泡隐怨影诅咒巡灵彷徨热带铃勃梭雪冰护豹珍珠樱空棘爱心哑属艾欧盖固坐祈代希苗台猛曼拿儿狸法师箱蟀勒伦琴含羞苞槌城结贵妇绅蜜女帕兹潜兔随卷耳魅东施铛响坦铜镜钟盆聒噪陆尖咬不良骷荧光霓虹自舔狂远Ｚ由卢席恩骑色霏莱谢米尔宙提主暖炒武刃丸剑探步哨约扒酷冷蚀豆鸽高雉幔庞滚蝙螺钉差搬运匠修建蟾蜍投摔打包保足蜈蚣车轮精根裙野蛮鲈混流氓红倒狒殿滑巾征哭具死神棺原肋始祖破灰尘索沫栗德单卵细胞造鹅倍四季萌哎呀败轻蜘坚齿组麻鳗宇烛幽晶斧嚏几何敏捷功夫父赤驹劈司令炸雄秃丫首恶燃烧毕云酋迪耶塔赛里狐呱贺掘彩蓓洁能鞘芳芙妮好鱿贼脚铠垃藻臂枪伞咚碎黏钥朽南瓜嗡哲裴格枭狙射炽咆哮虎漾壬笃啄铳少强锹农胜虻鬃弱坏驴仔重挽滴伪睡罩盗着竹疗环智挥猩掷胆噬堡爷参性：银伴陨枕戈谜拟Ｑ磨舵鳞杖璞・鸣哞鳍科莫迦虚吾肌费束辉纸御机夏蛋妙蛙種子草花小火龍恐噴傑尼龜卡咪水箭綠毛蟲鐵甲蛹巴大蝶獨角殼針蜂波比鳥拉達烈雀嘴阿柏蛇怪皮丘雷穿山鼠王多蘭娜后朗力諾可西六尾九胖丁超音蝠走路臭霸派斯特球摩魯蛾地三喵貓老鴨哥猴爆蒂狗風速蚊香蝌蚪君泳士凱勇基胡腕豪喇叭芽口呆食瑪瑙母毒刺拳石隆岩馬焰獸磁合一蔥嘟利海獅白泥舌貝鬼通耿催眠貘引夢人鉗蟹巨霹靂電頑彈椰樹嘎啦飛腿郎快頭瓦雙犀牛鑽吉蔓藤袋墨金魚星寶魔牆偶天螳螂迷唇姐擊羅肯泰鯉暴普百變伊布邊菊化盔鐮刀翼急凍閃你哈克幻葉月桂竺葵鋸鱷藍立咕夜鷹芭瓢安圓絲蛛叉字燈籠古然咩羊茸美麗露才皇毽棉長手向日蜻蜓烏沼太陽亮黑暗鴉妖未知圖騰果翁麒麟奇榛佛托土弟蠍鋼千壺赫狃熊圈熔蝸豬珊瑚炮章桶信使翅戴加象頓Ⅱ驚鹿犬無畏戰舞娃奶罐幸福公炎帝幼沙班洛亞鳳時木守宮森林蜥蜴稚雞壯躍狼紋直衝繭狩獵盾粉蓮童帽樂河橡實鼻狡猾傲骨燕鷗莉奈朵溜糖雨蘑菇斗笠懶獺過動猿請假居忍面者脫妞吼幕下掌朝北優雅勾魂眼那恰姆落正拍負螢甜薔薇溶吞牙鯊鯨駝煤炭跳噗晃斑顎蟻漠仙歌青綿七夕鼬斬飯匙鰍鯰蝦兵螯秤念觸搖籃羽醜納飄浮泡隱怨影詛咒巡靈彷徨熱帶鈴勃梭雪冰護豹珍珠櫻空棘愛心啞屬艾歐蓋固坐祈代希苗台猛曼拿兒狸法師箱蟀勒倫琴含羞苞槌城結貴婦紳蜜女帕茲潛兔隨捲耳魅東施鐺響坦銅鏡鐘盆聒噪陸尖咬不良骷光霓虹自舔狂遠Ｚ由盧席恩騎色霏萊謝米爾宙提主暖炒武刃丸劍探步哨約扒酷冷蝕豆鴿高雉幔龐滾蝙螺釘差搬運匠修建蟾蜍投摔打包保足蜈蚣車輪毬精根裙野蠻鱸混流氓紅倒狒殿滑巾徵哭具死神棺原肋始祖破灰塵索沫栗德單卵細胞造鵝倍四季萌哎呀敗輕蜘堅齒組麻鰻宇燭幽晶斧嚏幾何敏捷功夫父赤駒劈司令炸雄禿丫首惡燃燒畢雲酋迪耶塔賽里狐呱賀掘彩蓓潔能鞘芳芙妮好魷賊腳鎧垃藻臂槍傘咚碎黏鑰朽南瓜嗡哲裴格梟狙射熾咆哮虎漾壬篤啄銃少強鍬農勝虻鬃弱壞驢仔重挽滴偽睡罩盜著竹療環智揮猩擲膽噬堡爺參性：銀伴隕枕戈謎擬Ｑ磨舵鱗杖璞・鳴哞鰭科莫迦虛吾肌費束輝紙御機夏垒磊砰奥壘磊砰丑奧";
pub const GEN7_ZH_LENGTH: usize = 0x627;

const GEN7_ZH_OFS: char = '\u{E800}';
const SM_ZH_CHAR_TABLE_SIZE: usize = 0x30F;
const USUM_CHS_SIZE: usize = 0x4;

fn get_is_67_chs_char(idx: usize) -> bool {
    idx < SM_ZH_CHAR_TABLE_SIZE || (idx >= SM_ZH_CHAR_TABLE_SIZE * 2 && idx < (SM_ZH_CHAR_TABLE_SIZE * 2) + USUM_CHS_SIZE)
}

lazy_static! {
    static ref G7_CHS: HashMap<char, char> = {
        get_remapper(true)
    };
    static ref G7_CHT: HashMap<char, char> = {
        get_remapper(false)
    };
}

fn get_remapper(simplified: bool) -> HashMap<char, char> {
    let raw = GEN7_ZH_RAW.chars().collect::<Vec<char>>();
    let mut result = HashMap::with_capacity(788);
    for i in 0..raw.len() {
        let is_chs = get_is_67_chs_char(i);
        if is_chs != simplified {
            continue;
        }
        let c = raw[i];
        result.insert(c,  char::from_u32(GEN7_ZH_OFS as u32 + i as u32).unwrap());
    }
    result
}

pub(crate) fn convert_string_to_bin_g7_zh(chr: char, traditional: bool) -> char {
    let table: &HashMap<char, char> = if traditional { &G7_CHT } else { &G7_CHS };
    if let Some(entry) = table.get(&chr) {
        return *entry;
    }
    chr
}

pub fn is_traditional(input: &[char], lang: usize) -> bool {
    for c in input {
        if G7_CHT.contains_key(c) && !G7_CHS.contains_key(c) {
            return true;
        }
    }

    if lang != LanguageID::ChineseT as usize {
        return false;
    }

    for c in input {
        if G7_CHT.contains_key(c) != G7_CHS.contains_key(c) {
            return true;
        }
    }

    false
}

pub(crate) fn remap_chinese_glyphs_bin_to_string(input: &mut [char]) {
    for c in input {
        let unmap = (*c as u32).wrapping_sub(GEN7_ZH_OFS as u32);
        if unmap < GEN7_ZH_LENGTH as u32 {
            *c = GEN7_ZH_RAW.chars().nth(unmap as usize).unwrap();
        }
    }
}