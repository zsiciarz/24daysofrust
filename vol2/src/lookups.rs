
use std::collections::HashMap;
use phf;
use super::Color;

pub fn find_color(name: &str) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "air force blue (raf)" => Some(Color { r: 93, g: 138, b: 168 }),
        "air force blue (usaf)" => Some(Color { r: 0, g: 48, b: 143 }),
        "air superiority blue" => Some(Color { r: 114, g: 160, b: 193 }),
        "alabama crimson" => Some(Color { r: 163, g: 38, b: 56 }),
        "alice blue" => Some(Color { r: 240, g: 248, b: 255 }),
        "alizarin crimson" => Some(Color { r: 227, g: 38, b: 54 }),
        "alloy orange" => Some(Color { r: 196, g: 98, b: 16 }),
        "almond" => Some(Color { r: 239, g: 222, b: 205 }),
        "amaranth" => Some(Color { r: 229, g: 43, b: 80 }),
        "amber" => Some(Color { r: 255, g: 191, b: 0 }),
        "amber (sae/ece)" => Some(Color { r: 255, g: 126, b: 0 }),
        "american rose" => Some(Color { r: 255, g: 3, b: 62 }),
        "amethyst" => Some(Color { r: 153, g: 102, b: 204 }),
        "android green" => Some(Color { r: 164, g: 198, b: 57 }),
        "anti-flash white" => Some(Color { r: 242, g: 243, b: 244 }),
        "antique brass" => Some(Color { r: 205, g: 149, b: 117 }),
        "antique fuchsia" => Some(Color { r: 145, g: 92, b: 131 }),
        "antique ruby" => Some(Color { r: 132, g: 27, b: 45 }),
        "antique white" => Some(Color { r: 250, g: 235, b: 215 }),
        "ao (english)" => Some(Color { r: 0, g: 128, b: 0 }),
        "apple green" => Some(Color { r: 141, g: 182, b: 0 }),
        "apricot" => Some(Color { r: 251, g: 206, b: 177 }),
        "aqua" => Some(Color { r: 0, g: 255, b: 255 }),
        "aquamarine" => Some(Color { r: 127, g: 255, b: 212 }),
        "army green" => Some(Color { r: 75, g: 83, b: 32 }),
        "arsenic" => Some(Color { r: 59, g: 68, b: 75 }),
        "arylide yellow" => Some(Color { r: 233, g: 214, b: 107 }),
        "ash grey" => Some(Color { r: 178, g: 190, b: 181 }),
        "asparagus" => Some(Color { r: 135, g: 169, b: 107 }),
        "atomic tangerine" => Some(Color { r: 255, g: 153, b: 102 }),
        "auburn" => Some(Color { r: 165, g: 42, b: 42 }),
        "aureolin" => Some(Color { r: 253, g: 238, b: 0 }),
        "aurometalsaurus" => Some(Color { r: 110, g: 127, b: 128 }),
        "avocado" => Some(Color { r: 86, g: 130, b: 3 }),
        "azure" => Some(Color { r: 0, g: 127, b: 255 }),
        "azure mist/web" => Some(Color { r: 240, g: 255, b: 255 }),
        "baby blue" => Some(Color { r: 137, g: 207, b: 240 }),
        "baby blue eyes" => Some(Color { r: 161, g: 202, b: 241 }),
        "baby pink" => Some(Color { r: 244, g: 194, b: 194 }),
        "ball blue" => Some(Color { r: 33, g: 171, b: 205 }),
        "banana mania" => Some(Color { r: 250, g: 231, b: 181 }),
        "banana yellow" => Some(Color { r: 255, g: 225, b: 53 }),
        "barn red" => Some(Color { r: 124, g: 10, b: 2 }),
        "battleship grey" => Some(Color { r: 132, g: 132, b: 130 }),
        "bazaar" => Some(Color { r: 152, g: 119, b: 123 }),
        "beau blue" => Some(Color { r: 188, g: 212, b: 230 }),
        "beaver" => Some(Color { r: 159, g: 129, b: 112 }),
        "beige" => Some(Color { r: 245, g: 245, b: 220 }),
        "big dip o’ruby" => Some(Color { r: 156, g: 37, b: 66 }),
        "bisque" => Some(Color { r: 255, g: 228, b: 196 }),
        "bistre" => Some(Color { r: 61, g: 43, b: 31 }),
        "bittersweet" => Some(Color { r: 254, g: 111, b: 94 }),
        "bittersweet shimmer" => Some(Color { r: 191, g: 79, b: 81 }),
        "black" => Some(Color { r: 0, g: 0, b: 0 }),
        "black bean" => Some(Color { r: 61, g: 12, b: 2 }),
        "black leather jacket" => Some(Color { r: 37, g: 53, b: 41 }),
        "black olive" => Some(Color { r: 59, g: 60, b: 54 }),
        "blanched almond" => Some(Color { r: 255, g: 235, b: 205 }),
        "blast-off bronze" => Some(Color { r: 165, g: 113, b: 100 }),
        "bleu de france" => Some(Color { r: 49, g: 140, b: 231 }),
        "blizzard blue" => Some(Color { r: 172, g: 229, b: 238 }),
        "blond" => Some(Color { r: 250, g: 240, b: 190 }),
        "blue" => Some(Color { r: 0, g: 0, b: 255 }),
        "blue bell" => Some(Color { r: 162, g: 162, b: 208 }),
        "blue (crayola)" => Some(Color { r: 31, g: 117, b: 254 }),
        "blue gray" => Some(Color { r: 102, g: 153, b: 204 }),
        "blue-green" => Some(Color { r: 13, g: 152, b: 186 }),
        "blue (munsell)" => Some(Color { r: 0, g: 147, b: 175 }),
        "blue (ncs)" => Some(Color { r: 0, g: 135, b: 189 }),
        "blue (pigment)" => Some(Color { r: 51, g: 51, b: 153 }),
        "blue (ryb)" => Some(Color { r: 2, g: 71, b: 254 }),
        "blue sapphire" => Some(Color { r: 18, g: 97, b: 128 }),
        "blue-violet" => Some(Color { r: 138, g: 43, b: 226 }),
        "blush" => Some(Color { r: 222, g: 93, b: 131 }),
        "bole" => Some(Color { r: 121, g: 68, b: 59 }),
        "bondi blue" => Some(Color { r: 0, g: 149, b: 182 }),
        "bone" => Some(Color { r: 227, g: 218, b: 201 }),
        "boston university red" => Some(Color { r: 204, g: 0, b: 0 }),
        "bottle green" => Some(Color { r: 0, g: 106, b: 78 }),
        "boysenberry" => Some(Color { r: 135, g: 50, b: 96 }),
        "brandeis blue" => Some(Color { r: 0, g: 112, b: 255 }),
        "brass" => Some(Color { r: 181, g: 166, b: 66 }),
        "brick red" => Some(Color { r: 203, g: 65, b: 84 }),
        "bright cerulean" => Some(Color { r: 29, g: 172, b: 214 }),
        "bright green" => Some(Color { r: 102, g: 255, b: 0 }),
        "bright lavender" => Some(Color { r: 191, g: 148, b: 228 }),
        "bright maroon" => Some(Color { r: 195, g: 33, b: 72 }),
        "bright pink" => Some(Color { r: 255, g: 0, b: 127 }),
        "bright turquoise" => Some(Color { r: 8, g: 232, b: 222 }),
        "bright ube" => Some(Color { r: 209, g: 159, b: 232 }),
        "brilliant lavender" => Some(Color { r: 244, g: 187, b: 255 }),
        "brilliant rose" => Some(Color { r: 255, g: 85, b: 163 }),
        "brink pink" => Some(Color { r: 251, g: 96, b: 127 }),
        "british racing green" => Some(Color { r: 0, g: 66, b: 37 }),
        "bronze" => Some(Color { r: 205, g: 127, b: 50 }),
        "brown (traditional)" => Some(Color { r: 150, g: 75, b: 0 }),
        "brown (web)" => Some(Color { r: 165, g: 42, b: 42 }),
        "bubble gum" => Some(Color { r: 255, g: 193, b: 204 }),
        "bubbles" => Some(Color { r: 231, g: 254, b: 255 }),
        "buff" => Some(Color { r: 240, g: 220, b: 130 }),
        "bulgarian rose" => Some(Color { r: 72, g: 6, b: 7 }),
        "burgundy" => Some(Color { r: 128, g: 0, b: 32 }),
        "burlywood" => Some(Color { r: 222, g: 184, b: 135 }),
        "burnt orange" => Some(Color { r: 204, g: 85, b: 0 }),
        "burnt sienna" => Some(Color { r: 233, g: 116, b: 81 }),
        "burnt umber" => Some(Color { r: 138, g: 51, b: 36 }),
        "byzantine" => Some(Color { r: 189, g: 51, b: 164 }),
        "byzantium" => Some(Color { r: 112, g: 41, b: 99 }),
        "cadet" => Some(Color { r: 83, g: 104, b: 114 }),
        "cadet blue" => Some(Color { r: 95, g: 158, b: 160 }),
        "cadet grey" => Some(Color { r: 145, g: 163, b: 176 }),
        "cadmium green" => Some(Color { r: 0, g: 107, b: 60 }),
        "cadmium orange" => Some(Color { r: 237, g: 135, b: 45 }),
        "cadmium red" => Some(Color { r: 227, g: 0, b: 34 }),
        "cadmium yellow" => Some(Color { r: 255, g: 246, b: 0 }),
        "cafe au lait" => Some(Color { r: 166, g: 123, b: 91 }),
        "cafe noir" => Some(Color { r: 75, g: 54, b: 33 }),
        "cal poly green" => Some(Color { r: 30, g: 77, b: 43 }),
        "cambridge blue" => Some(Color { r: 163, g: 193, b: 173 }),
        "camel" => Some(Color { r: 193, g: 154, b: 107 }),
        "cameo pink" => Some(Color { r: 239, g: 187, b: 204 }),
        "camouflage green" => Some(Color { r: 120, g: 134, b: 107 }),
        "canary yellow" => Some(Color { r: 255, g: 239, b: 0 }),
        "candy apple red" => Some(Color { r: 255, g: 8, b: 0 }),
        "candy pink" => Some(Color { r: 228, g: 113, b: 122 }),
        "capri" => Some(Color { r: 0, g: 191, b: 255 }),
        "caput mortuum" => Some(Color { r: 89, g: 39, b: 32 }),
        "cardinal" => Some(Color { r: 196, g: 30, b: 58 }),
        "caribbean green" => Some(Color { r: 0, g: 204, b: 153 }),
        "carmine" => Some(Color { r: 150, g: 0, b: 24 }),
        "carmine (m&p)" => Some(Color { r: 215, g: 0, b: 64 }),
        "carmine pink" => Some(Color { r: 235, g: 76, b: 66 }),
        "carmine red" => Some(Color { r: 255, g: 0, b: 56 }),
        "carnation pink" => Some(Color { r: 255, g: 166, b: 201 }),
        "carnelian" => Some(Color { r: 179, g: 27, b: 27 }),
        "carolina blue" => Some(Color { r: 153, g: 186, b: 221 }),
        "carrot orange" => Some(Color { r: 237, g: 145, b: 33 }),
        "catalina blue" => Some(Color { r: 6, g: 42, b: 120 }),
        "ceil" => Some(Color { r: 146, g: 161, b: 207 }),
        "celadon" => Some(Color { r: 172, g: 225, b: 175 }),
        "celadon blue" => Some(Color { r: 0, g: 123, b: 167 }),
        "celadon green" => Some(Color { r: 47, g: 132, b: 124 }),
        "celeste (colour)" => Some(Color { r: 178, g: 255, b: 255 }),
        "celestial blue" => Some(Color { r: 73, g: 151, b: 208 }),
        "cerise" => Some(Color { r: 222, g: 49, b: 99 }),
        "cerise pink" => Some(Color { r: 236, g: 59, b: 131 }),
        "cerulean" => Some(Color { r: 0, g: 123, b: 167 }),
        "cerulean blue" => Some(Color { r: 42, g: 82, b: 190 }),
        "cerulean frost" => Some(Color { r: 109, g: 155, b: 195 }),
        "cg blue" => Some(Color { r: 0, g: 122, b: 165 }),
        "cg red" => Some(Color { r: 224, g: 60, b: 49 }),
        "chamoisee" => Some(Color { r: 160, g: 120, b: 90 }),
        "champagne" => Some(Color { r: 250, g: 214, b: 165 }),
        "charcoal" => Some(Color { r: 54, g: 69, b: 79 }),
        "charm pink" => Some(Color { r: 230, g: 143, b: 172 }),
        "chartreuse (traditional)" => Some(Color { r: 223, g: 255, b: 0 }),
        "chartreuse (web)" => Some(Color { r: 127, g: 255, b: 0 }),
        "cherry" => Some(Color { r: 222, g: 49, b: 99 }),
        "cherry blossom pink" => Some(Color { r: 255, g: 183, b: 197 }),
        "chestnut" => Some(Color { r: 205, g: 92, b: 92 }),
        "china pink" => Some(Color { r: 222, g: 111, b: 161 }),
        "china rose" => Some(Color { r: 168, g: 81, b: 110 }),
        "chinese red" => Some(Color { r: 170, g: 56, b: 30 }),
        "chocolate (traditional)" => Some(Color { r: 123, g: 63, b: 0 }),
        "chocolate (web)" => Some(Color { r: 210, g: 105, b: 30 }),
        "chrome yellow" => Some(Color { r: 255, g: 167, b: 0 }),
        "cinereous" => Some(Color { r: 152, g: 129, b: 123 }),
        "cinnabar" => Some(Color { r: 227, g: 66, b: 52 }),
        "cinnamon" => Some(Color { r: 210, g: 105, b: 30 }),
        "citrine" => Some(Color { r: 228, g: 208, b: 10 }),
        "classic rose" => Some(Color { r: 251, g: 204, b: 231 }),
        "cobalt" => Some(Color { r: 0, g: 71, b: 171 }),
        "cocoa brown" => Some(Color { r: 210, g: 105, b: 30 }),
        "coffee" => Some(Color { r: 111, g: 78, b: 55 }),
        "columbia blue" => Some(Color { r: 155, g: 221, b: 255 }),
        "congo pink" => Some(Color { r: 248, g: 131, b: 121 }),
        "cool black" => Some(Color { r: 0, g: 46, b: 99 }),
        "cool grey" => Some(Color { r: 140, g: 146, b: 172 }),
        "copper" => Some(Color { r: 184, g: 115, b: 51 }),
        "copper (crayola)" => Some(Color { r: 218, g: 138, b: 103 }),
        "copper penny" => Some(Color { r: 173, g: 111, b: 105 }),
        "copper red" => Some(Color { r: 203, g: 109, b: 81 }),
        "copper rose" => Some(Color { r: 153, g: 102, b: 102 }),
        "coquelicot" => Some(Color { r: 255, g: 56, b: 0 }),
        "coral" => Some(Color { r: 255, g: 127, b: 80 }),
        "coral pink" => Some(Color { r: 248, g: 131, b: 121 }),
        "coral red" => Some(Color { r: 255, g: 64, b: 64 }),
        "cordovan" => Some(Color { r: 137, g: 63, b: 69 }),
        "corn" => Some(Color { r: 251, g: 236, b: 93 }),
        "cornell red" => Some(Color { r: 179, g: 27, b: 27 }),
        "cornflower blue" => Some(Color { r: 100, g: 149, b: 237 }),
        "cornsilk" => Some(Color { r: 255, g: 248, b: 220 }),
        "cosmic latte" => Some(Color { r: 255, g: 248, b: 231 }),
        "cotton candy" => Some(Color { r: 255, g: 188, b: 217 }),
        "cream" => Some(Color { r: 255, g: 253, b: 208 }),
        "crimson" => Some(Color { r: 220, g: 20, b: 60 }),
        "crimson glory" => Some(Color { r: 190, g: 0, b: 50 }),
        "cyan" => Some(Color { r: 0, g: 255, b: 255 }),
        "cyan (process)" => Some(Color { r: 0, g: 183, b: 235 }),
        "daffodil" => Some(Color { r: 255, g: 255, b: 49 }),
        "dandelion" => Some(Color { r: 240, g: 225, b: 48 }),
        "dark blue" => Some(Color { r: 0, g: 0, b: 139 }),
        "dark brown" => Some(Color { r: 101, g: 67, b: 33 }),
        "dark byzantium" => Some(Color { r: 93, g: 57, b: 84 }),
        "dark candy apple red" => Some(Color { r: 164, g: 0, b: 0 }),
        "dark cerulean" => Some(Color { r: 8, g: 69, b: 126 }),
        "dark chestnut" => Some(Color { r: 152, g: 105, b: 96 }),
        "dark coral" => Some(Color { r: 205, g: 91, b: 69 }),
        "dark cyan" => Some(Color { r: 0, g: 139, b: 139 }),
        "dark electric blue" => Some(Color { r: 83, g: 104, b: 120 }),
        "dark goldenrod" => Some(Color { r: 184, g: 134, b: 11 }),
        "dark gray" => Some(Color { r: 169, g: 169, b: 169 }),
        "dark green" => Some(Color { r: 1, g: 50, b: 32 }),
        "dark imperial blue" => Some(Color { r: 0, g: 65, b: 106 }),
        "dark jungle green" => Some(Color { r: 26, g: 36, b: 33 }),
        "dark khaki" => Some(Color { r: 189, g: 183, b: 107 }),
        "dark lava" => Some(Color { r: 72, g: 60, b: 50 }),
        "dark lavender" => Some(Color { r: 115, g: 79, b: 150 }),
        "dark magenta" => Some(Color { r: 139, g: 0, b: 139 }),
        "dark midnight blue" => Some(Color { r: 0, g: 51, b: 102 }),
        "dark olive green" => Some(Color { r: 85, g: 107, b: 47 }),
        "dark orange" => Some(Color { r: 255, g: 140, b: 0 }),
        "dark orchid" => Some(Color { r: 153, g: 50, b: 204 }),
        "dark pastel blue" => Some(Color { r: 119, g: 158, b: 203 }),
        "dark pastel green" => Some(Color { r: 3, g: 192, b: 60 }),
        "dark pastel purple" => Some(Color { r: 150, g: 111, b: 214 }),
        "dark pastel red" => Some(Color { r: 194, g: 59, b: 34 }),
        "dark pink" => Some(Color { r: 231, g: 84, b: 128 }),
        "dark powder blue" => Some(Color { r: 0, g: 51, b: 153 }),
        "dark raspberry" => Some(Color { r: 135, g: 38, b: 87 }),
        "dark red" => Some(Color { r: 139, g: 0, b: 0 }),
        "dark salmon" => Some(Color { r: 233, g: 150, b: 122 }),
        "dark scarlet" => Some(Color { r: 86, g: 3, b: 25 }),
        "dark sea green" => Some(Color { r: 143, g: 188, b: 143 }),
        "dark sienna" => Some(Color { r: 60, g: 20, b: 20 }),
        "dark slate blue" => Some(Color { r: 72, g: 61, b: 139 }),
        "dark slate gray" => Some(Color { r: 47, g: 79, b: 79 }),
        "dark spring green" => Some(Color { r: 23, g: 114, b: 69 }),
        "dark tan" => Some(Color { r: 145, g: 129, b: 81 }),
        "dark tangerine" => Some(Color { r: 255, g: 168, b: 18 }),
        "dark taupe" => Some(Color { r: 72, g: 60, b: 50 }),
        "dark terra cotta" => Some(Color { r: 204, g: 78, b: 92 }),
        "dark turquoise" => Some(Color { r: 0, g: 206, b: 209 }),
        "dark violet" => Some(Color { r: 148, g: 0, b: 211 }),
        "dark yellow" => Some(Color { r: 155, g: 135, b: 12 }),
        "dartmouth green" => Some(Color { r: 0, g: 112, b: 60 }),
        "davy's grey" => Some(Color { r: 85, g: 85, b: 85 }),
        "debian red" => Some(Color { r: 215, g: 10, b: 83 }),
        "deep carmine" => Some(Color { r: 169, g: 32, b: 62 }),
        "deep carmine pink" => Some(Color { r: 239, g: 48, b: 56 }),
        "deep carrot orange" => Some(Color { r: 233, g: 105, b: 44 }),
        "deep cerise" => Some(Color { r: 218, g: 50, b: 135 }),
        "deep champagne" => Some(Color { r: 250, g: 214, b: 165 }),
        "deep chestnut" => Some(Color { r: 185, g: 78, b: 72 }),
        "deep coffee" => Some(Color { r: 112, g: 66, b: 65 }),
        "deep fuchsia" => Some(Color { r: 193, g: 84, b: 193 }),
        "deep jungle green" => Some(Color { r: 0, g: 75, b: 73 }),
        "deep lilac" => Some(Color { r: 153, g: 85, b: 187 }),
        "deep magenta" => Some(Color { r: 204, g: 0, b: 204 }),
        "deep peach" => Some(Color { r: 255, g: 203, b: 164 }),
        "deep pink" => Some(Color { r: 255, g: 20, b: 147 }),
        "deep ruby" => Some(Color { r: 132, g: 63, b: 91 }),
        "deep saffron" => Some(Color { r: 255, g: 153, b: 51 }),
        "deep sky blue" => Some(Color { r: 0, g: 191, b: 255 }),
        "deep tuscan red" => Some(Color { r: 102, g: 66, b: 77 }),
        "denim" => Some(Color { r: 21, g: 96, b: 189 }),
        "desert" => Some(Color { r: 193, g: 154, b: 107 }),
        "desert sand" => Some(Color { r: 237, g: 201, b: 175 }),
        "dim gray" => Some(Color { r: 105, g: 105, b: 105 }),
        "dodger blue" => Some(Color { r: 30, g: 144, b: 255 }),
        "dogwood rose" => Some(Color { r: 215, g: 24, b: 104 }),
        "dollar bill" => Some(Color { r: 133, g: 187, b: 101 }),
        "drab" => Some(Color { r: 150, g: 113, b: 23 }),
        "duke blue" => Some(Color { r: 0, g: 0, b: 156 }),
        "earth yellow" => Some(Color { r: 225, g: 169, b: 95 }),
        "ebony" => Some(Color { r: 85, g: 93, b: 80 }),
        "ecru" => Some(Color { r: 194, g: 178, b: 128 }),
        "eggplant" => Some(Color { r: 97, g: 64, b: 81 }),
        "eggshell" => Some(Color { r: 240, g: 234, b: 214 }),
        "egyptian blue" => Some(Color { r: 16, g: 52, b: 166 }),
        "electric blue" => Some(Color { r: 125, g: 249, b: 255 }),
        "electric crimson" => Some(Color { r: 255, g: 0, b: 63 }),
        "electric cyan" => Some(Color { r: 0, g: 255, b: 255 }),
        "electric green" => Some(Color { r: 0, g: 255, b: 0 }),
        "electric indigo" => Some(Color { r: 111, g: 0, b: 255 }),
        "electric lavender" => Some(Color { r: 244, g: 187, b: 255 }),
        "electric lime" => Some(Color { r: 204, g: 255, b: 0 }),
        "electric purple" => Some(Color { r: 191, g: 0, b: 255 }),
        "electric ultramarine" => Some(Color { r: 63, g: 0, b: 255 }),
        "electric violet" => Some(Color { r: 143, g: 0, b: 255 }),
        "electric yellow" => Some(Color { r: 255, g: 255, b: 0 }),
        "emerald" => Some(Color { r: 80, g: 200, b: 120 }),
        "english lavender" => Some(Color { r: 180, g: 131, b: 149 }),
        "eton blue" => Some(Color { r: 150, g: 200, b: 162 }),
        "fallow" => Some(Color { r: 193, g: 154, b: 107 }),
        "falu red" => Some(Color { r: 128, g: 24, b: 24 }),
        "fandango" => Some(Color { r: 181, g: 51, b: 137 }),
        "fashion fuchsia" => Some(Color { r: 244, g: 0, b: 161 }),
        "fawn" => Some(Color { r: 229, g: 170, b: 112 }),
        "feldgrau" => Some(Color { r: 77, g: 93, b: 83 }),
        "fern green" => Some(Color { r: 79, g: 121, b: 66 }),
        "ferrari red" => Some(Color { r: 255, g: 40, b: 0 }),
        "field drab" => Some(Color { r: 108, g: 84, b: 30 }),
        "fire engine red" => Some(Color { r: 206, g: 32, b: 41 }),
        "firebrick" => Some(Color { r: 178, g: 34, b: 34 }),
        "flame" => Some(Color { r: 226, g: 88, b: 34 }),
        "flamingo pink" => Some(Color { r: 252, g: 142, b: 172 }),
        "flavescent" => Some(Color { r: 247, g: 233, b: 142 }),
        "flax" => Some(Color { r: 238, g: 220, b: 130 }),
        "floral white" => Some(Color { r: 255, g: 250, b: 240 }),
        "fluorescent orange" => Some(Color { r: 255, g: 191, b: 0 }),
        "fluorescent pink" => Some(Color { r: 255, g: 20, b: 147 }),
        "fluorescent yellow" => Some(Color { r: 204, g: 255, b: 0 }),
        "folly" => Some(Color { r: 255, g: 0, b: 79 }),
        "forest green (traditional)" => Some(Color { r: 1, g: 68, b: 33 }),
        "forest green (web)" => Some(Color { r: 34, g: 139, b: 34 }),
        "french beige" => Some(Color { r: 166, g: 123, b: 91 }),
        "french blue" => Some(Color { r: 0, g: 114, b: 187 }),
        "french lilac" => Some(Color { r: 134, g: 96, b: 142 }),
        "french lime" => Some(Color { r: 204, g: 255, b: 0 }),
        "french raspberry" => Some(Color { r: 199, g: 44, b: 72 }),
        "french rose" => Some(Color { r: 246, g: 74, b: 138 }),
        "fuchsia" => Some(Color { r: 255, g: 0, b: 255 }),
        "fuchsia (crayola)" => Some(Color { r: 193, g: 84, b: 193 }),
        "fuchsia pink" => Some(Color { r: 255, g: 119, b: 255 }),
        "fuchsia rose" => Some(Color { r: 199, g: 67, b: 117 }),
        "fulvous" => Some(Color { r: 228, g: 132, b: 0 }),
        "fuzzy wuzzy" => Some(Color { r: 204, g: 102, b: 102 }),
        "gainsboro" => Some(Color { r: 220, g: 220, b: 220 }),
        "gamboge" => Some(Color { r: 228, g: 155, b: 15 }),
        "ghost white" => Some(Color { r: 248, g: 248, b: 255 }),
        "ginger" => Some(Color { r: 176, g: 101, b: 0 }),
        "glaucous" => Some(Color { r: 96, g: 130, b: 182 }),
        "glitter" => Some(Color { r: 230, g: 232, b: 250 }),
        "gold (metallic)" => Some(Color { r: 212, g: 175, b: 55 }),
        "gold (web) (golden)" => Some(Color { r: 255, g: 215, b: 0 }),
        "golden brown" => Some(Color { r: 153, g: 101, b: 21 }),
        "golden poppy" => Some(Color { r: 252, g: 194, b: 0 }),
        "golden yellow" => Some(Color { r: 255, g: 223, b: 0 }),
        "goldenrod" => Some(Color { r: 218, g: 165, b: 32 }),
        "granny smith apple" => Some(Color { r: 168, g: 228, b: 160 }),
        "gray" => Some(Color { r: 128, g: 128, b: 128 }),
        "gray-asparagus" => Some(Color { r: 70, g: 89, b: 69 }),
        "gray (html/css gray)" => Some(Color { r: 128, g: 128, b: 128 }),
        "gray (x11 gray)" => Some(Color { r: 190, g: 190, b: 190 }),
        "green (color wheel) (x11 green)" => Some(Color { r: 0, g: 255, b: 0 }),
        "green (crayola)" => Some(Color { r: 28, g: 172, b: 120 }),
        "green (html/css green)" => Some(Color { r: 0, g: 128, b: 0 }),
        "green (munsell)" => Some(Color { r: 0, g: 168, b: 119 }),
        "green (ncs)" => Some(Color { r: 0, g: 159, b: 107 }),
        "green (pigment)" => Some(Color { r: 0, g: 165, b: 80 }),
        "green (ryb)" => Some(Color { r: 102, g: 176, b: 50 }),
        "green-yellow" => Some(Color { r: 173, g: 255, b: 47 }),
        "grullo" => Some(Color { r: 169, g: 154, b: 134 }),
        "guppie green" => Some(Color { r: 0, g: 255, b: 127 }),
        "halaya be" => Some(Color { r: 102, g: 56, b: 84 }),
        "han blue" => Some(Color { r: 68, g: 108, b: 207 }),
        "han purple" => Some(Color { r: 82, g: 24, b: 250 }),
        "hansa yellow" => Some(Color { r: 233, g: 214, b: 107 }),
        "harlequin" => Some(Color { r: 63, g: 255, b: 0 }),
        "harvard crimson" => Some(Color { r: 201, g: 0, b: 22 }),
        "harvest gold" => Some(Color { r: 218, g: 145, b: 0 }),
        "heart gold" => Some(Color { r: 128, g: 128, b: 0 }),
        "heliotrope" => Some(Color { r: 223, g: 115, b: 255 }),
        "hollywood cerise" => Some(Color { r: 244, g: 0, b: 161 }),
        "honeydew" => Some(Color { r: 240, g: 255, b: 240 }),
        "honolulu blue" => Some(Color { r: 0, g: 127, b: 191 }),
        "hooker's green" => Some(Color { r: 73, g: 121, b: 107 }),
        "hot magenta" => Some(Color { r: 255, g: 29, b: 206 }),
        "hot pink" => Some(Color { r: 255, g: 105, b: 180 }),
        "hunter green" => Some(Color { r: 53, g: 94, b: 59 }),
        "iceberg" => Some(Color { r: 113, g: 166, b: 210 }),
        "icterine" => Some(Color { r: 252, g: 247, b: 94 }),
        "imperial blue" => Some(Color { r: 0, g: 35, b: 149 }),
        "inchworm" => Some(Color { r: 178, g: 236, b: 93 }),
        "india green" => Some(Color { r: 19, g: 136, b: 8 }),
        "indian red" => Some(Color { r: 205, g: 92, b: 92 }),
        "indian yellow" => Some(Color { r: 227, g: 168, b: 87 }),
        "indigo" => Some(Color { r: 111, g: 0, b: 255 }),
        "indigo (dye)" => Some(Color { r: 0, g: 65, b: 106 }),
        "indigo (web)" => Some(Color { r: 75, g: 0, b: 130 }),
        "international klein blue" => Some(Color { r: 0, g: 47, b: 167 }),
        "international orange (aerospace)" => Some(Color { r: 255, g: 79, b: 0 }),
        "international orange (engineering)" => Some(Color { r: 186, g: 22, b: 12 }),
        "international orange (golden gate bridge)" => Some(Color { r: 192, g: 54, b: 44 }),
        "iris" => Some(Color { r: 90, g: 79, b: 207 }),
        "isabelline" => Some(Color { r: 244, g: 240, b: 236 }),
        "islamic green" => Some(Color { r: 0, g: 144, b: 0 }),
        "ivory" => Some(Color { r: 255, g: 255, b: 240 }),
        "jade" => Some(Color { r: 0, g: 168, b: 107 }),
        "jasmine" => Some(Color { r: 248, g: 222, b: 126 }),
        "jasper" => Some(Color { r: 215, g: 59, b: 62 }),
        "jazzberry jam" => Some(Color { r: 165, g: 11, b: 94 }),
        "jet" => Some(Color { r: 52, g: 52, b: 52 }),
        "jonquil" => Some(Color { r: 250, g: 218, b: 94 }),
        "june bud" => Some(Color { r: 189, g: 218, b: 87 }),
        "jungle green" => Some(Color { r: 41, g: 171, b: 135 }),
        "kelly green" => Some(Color { r: 76, g: 187, b: 23 }),
        "kenyan copper" => Some(Color { r: 124, g: 28, b: 5 }),
        "khaki (html/css) (khaki)" => Some(Color { r: 195, g: 176, b: 145 }),
        "khaki (x11) (light khaki)" => Some(Color { r: 240, g: 230, b: 140 }),
        "ku crimson" => Some(Color { r: 232, g: 0, b: 13 }),
        "la salle green" => Some(Color { r: 8, g: 120, b: 48 }),
        "languid lavender" => Some(Color { r: 214, g: 202, b: 221 }),
        "lapis lazuli" => Some(Color { r: 38, g: 97, b: 156 }),
        "laser lemon" => Some(Color { r: 254, g: 254, b: 34 }),
        "laurel green" => Some(Color { r: 169, g: 186, b: 157 }),
        "lava" => Some(Color { r: 207, g: 16, b: 32 }),
        "lavender blue" => Some(Color { r: 204, g: 204, b: 255 }),
        "lavender blush" => Some(Color { r: 255, g: 240, b: 245 }),
        "lavender (floral)" => Some(Color { r: 181, g: 126, b: 220 }),
        "lavender gray" => Some(Color { r: 196, g: 195, b: 208 }),
        "lavender indigo" => Some(Color { r: 148, g: 87, b: 235 }),
        "lavender magenta" => Some(Color { r: 238, g: 130, b: 238 }),
        "lavender mist" => Some(Color { r: 230, g: 230, b: 250 }),
        "lavender pink" => Some(Color { r: 251, g: 174, b: 210 }),
        "lavender purple" => Some(Color { r: 150, g: 123, b: 182 }),
        "lavender rose" => Some(Color { r: 251, g: 160, b: 227 }),
        "lavender (web)" => Some(Color { r: 230, g: 230, b: 250 }),
        "lawn green" => Some(Color { r: 124, g: 252, b: 0 }),
        "lemon" => Some(Color { r: 255, g: 247, b: 0 }),
        "lemon chiffon" => Some(Color { r: 255, g: 250, b: 205 }),
        "lemon lime" => Some(Color { r: 227, g: 255, b: 0 }),
        "licorice" => Some(Color { r: 26, g: 17, b: 16 }),
        "light apricot" => Some(Color { r: 253, g: 213, b: 177 }),
        "light blue" => Some(Color { r: 173, g: 216, b: 230 }),
        "light brown" => Some(Color { r: 181, g: 101, b: 29 }),
        "light carmine pink" => Some(Color { r: 230, g: 103, b: 113 }),
        "light coral" => Some(Color { r: 240, g: 128, b: 128 }),
        "light cornflower blue" => Some(Color { r: 147, g: 204, b: 234 }),
        "light crimson" => Some(Color { r: 245, g: 105, b: 145 }),
        "light cyan" => Some(Color { r: 224, g: 255, b: 255 }),
        "light fuchsia pink" => Some(Color { r: 249, g: 132, b: 239 }),
        "light goldenrod yellow" => Some(Color { r: 250, g: 250, b: 210 }),
        "light gray" => Some(Color { r: 211, g: 211, b: 211 }),
        "light green" => Some(Color { r: 144, g: 238, b: 144 }),
        "light khaki" => Some(Color { r: 240, g: 230, b: 140 }),
        "light pastel purple" => Some(Color { r: 177, g: 156, b: 217 }),
        "light pink" => Some(Color { r: 255, g: 182, b: 193 }),
        "light red ochre" => Some(Color { r: 233, g: 116, b: 81 }),
        "light salmon" => Some(Color { r: 255, g: 160, b: 122 }),
        "light salmon pink" => Some(Color { r: 255, g: 153, b: 153 }),
        "light sea green" => Some(Color { r: 32, g: 178, b: 170 }),
        "light sky blue" => Some(Color { r: 135, g: 206, b: 250 }),
        "light slate gray" => Some(Color { r: 119, g: 136, b: 153 }),
        "light taupe" => Some(Color { r: 179, g: 139, b: 109 }),
        "light thulian pink" => Some(Color { r: 230, g: 143, b: 172 }),
        "light yellow" => Some(Color { r: 255, g: 255, b: 224 }),
        "lilac" => Some(Color { r: 200, g: 162, b: 200 }),
        "lime (color wheel)" => Some(Color { r: 191, g: 255, b: 0 }),
        "lime green" => Some(Color { r: 50, g: 205, b: 50 }),
        "lime (web) (x11 green)" => Some(Color { r: 0, g: 255, b: 0 }),
        "limerick" => Some(Color { r: 157, g: 194, b: 9 }),
        "lincoln green" => Some(Color { r: 25, g: 89, b: 5 }),
        "linen" => Some(Color { r: 250, g: 240, b: 230 }),
        "lion" => Some(Color { r: 193, g: 154, b: 107 }),
        "little boy blue" => Some(Color { r: 108, g: 160, b: 220 }),
        "liver" => Some(Color { r: 83, g: 75, b: 79 }),
        "lust" => Some(Color { r: 230, g: 32, b: 32 }),
        "magenta" => Some(Color { r: 255, g: 0, b: 255 }),
        "magenta (dye)" => Some(Color { r: 202, g: 31, b: 123 }),
        "magenta (process)" => Some(Color { r: 255, g: 0, b: 144 }),
        "magic mint" => Some(Color { r: 170, g: 240, b: 209 }),
        "magnolia" => Some(Color { r: 248, g: 244, b: 255 }),
        "mahogany" => Some(Color { r: 192, g: 64, b: 0 }),
        "maize" => Some(Color { r: 251, g: 236, b: 93 }),
        "majorelle blue" => Some(Color { r: 96, g: 80, b: 220 }),
        "malachite" => Some(Color { r: 11, g: 218, b: 81 }),
        "manatee" => Some(Color { r: 151, g: 154, b: 170 }),
        "mango tango" => Some(Color { r: 255, g: 130, b: 67 }),
        "mantis" => Some(Color { r: 116, g: 195, b: 101 }),
        "mardi gras" => Some(Color { r: 136, g: 0, b: 133 }),
        "maroon (crayola)" => Some(Color { r: 195, g: 33, b: 72 }),
        "maroon (html/css)" => Some(Color { r: 128, g: 0, b: 0 }),
        "maroon (x11)" => Some(Color { r: 176, g: 48, b: 96 }),
        "mauve" => Some(Color { r: 224, g: 176, b: 255 }),
        "mauve taupe" => Some(Color { r: 145, g: 95, b: 109 }),
        "mauvelous" => Some(Color { r: 239, g: 152, b: 170 }),
        "maya blue" => Some(Color { r: 115, g: 194, b: 251 }),
        "meat brown" => Some(Color { r: 229, g: 183, b: 59 }),
        "medium aquamarine" => Some(Color { r: 102, g: 221, b: 170 }),
        "medium blue" => Some(Color { r: 0, g: 0, b: 205 }),
        "medium candy apple red" => Some(Color { r: 226, g: 6, b: 44 }),
        "medium carmine" => Some(Color { r: 175, g: 64, b: 53 }),
        "medium champagne" => Some(Color { r: 243, g: 229, b: 171 }),
        "medium electric blue" => Some(Color { r: 3, g: 80, b: 150 }),
        "medium jungle green" => Some(Color { r: 28, g: 53, b: 45 }),
        "medium lavender magenta" => Some(Color { r: 221, g: 160, b: 221 }),
        "medium orchid" => Some(Color { r: 186, g: 85, b: 211 }),
        "medium persian blue" => Some(Color { r: 0, g: 103, b: 165 }),
        "medium purple" => Some(Color { r: 147, g: 112, b: 219 }),
        "medium red-violet" => Some(Color { r: 187, g: 51, b: 133 }),
        "medium ruby" => Some(Color { r: 170, g: 64, b: 105 }),
        "medium sea green" => Some(Color { r: 60, g: 179, b: 113 }),
        "medium slate blue" => Some(Color { r: 123, g: 104, b: 238 }),
        "medium spring bud" => Some(Color { r: 201, g: 220, b: 135 }),
        "medium spring green" => Some(Color { r: 0, g: 250, b: 154 }),
        "medium taupe" => Some(Color { r: 103, g: 76, b: 71 }),
        "medium turquoise" => Some(Color { r: 72, g: 209, b: 204 }),
        "medium tuscan red" => Some(Color { r: 121, g: 68, b: 59 }),
        "medium vermilion" => Some(Color { r: 217, g: 96, b: 59 }),
        "medium violet-red" => Some(Color { r: 199, g: 21, b: 133 }),
        "mellow apricot" => Some(Color { r: 248, g: 184, b: 120 }),
        "mellow yellow" => Some(Color { r: 248, g: 222, b: 126 }),
        "melon" => Some(Color { r: 253, g: 188, b: 180 }),
        "midnight blue" => Some(Color { r: 25, g: 25, b: 112 }),
        "midnight green (eagle green)" => Some(Color { r: 0, g: 73, b: 83 }),
        "mikado yellow" => Some(Color { r: 255, g: 196, b: 12 }),
        "mint" => Some(Color { r: 62, g: 180, b: 137 }),
        "mint cream" => Some(Color { r: 245, g: 255, b: 250 }),
        "mint green" => Some(Color { r: 152, g: 255, b: 152 }),
        "misty rose" => Some(Color { r: 255, g: 228, b: 225 }),
        "moccasin" => Some(Color { r: 250, g: 235, b: 215 }),
        "mode beige" => Some(Color { r: 150, g: 113, b: 23 }),
        "moonstone blue" => Some(Color { r: 115, g: 169, b: 194 }),
        "mordant red 19" => Some(Color { r: 174, g: 12, b: 0 }),
        "moss green" => Some(Color { r: 173, g: 223, b: 173 }),
        "mountain meadow" => Some(Color { r: 48, g: 186, b: 143 }),
        "mountbatten pink" => Some(Color { r: 153, g: 122, b: 141 }),
        "msu green" => Some(Color { r: 24, g: 69, b: 59 }),
        "mulberry" => Some(Color { r: 197, g: 75, b: 140 }),
        "mustard" => Some(Color { r: 255, g: 219, b: 88 }),
        "myrtle" => Some(Color { r: 33, g: 66, b: 30 }),
        "nadeshiko pink" => Some(Color { r: 246, g: 173, b: 198 }),
        "napier green" => Some(Color { r: 42, g: 128, b: 0 }),
        "naples yellow" => Some(Color { r: 250, g: 218, b: 94 }),
        "navajo white" => Some(Color { r: 255, g: 222, b: 173 }),
        "navy blue" => Some(Color { r: 0, g: 0, b: 128 }),
        "neon carrot" => Some(Color { r: 255, g: 163, b: 67 }),
        "neon fuchsia" => Some(Color { r: 254, g: 65, b: 100 }),
        "neon green" => Some(Color { r: 57, g: 255, b: 20 }),
        "new york pink" => Some(Color { r: 215, g: 131, b: 127 }),
        "non-photo blue" => Some(Color { r: 164, g: 221, b: 237 }),
        "north texas green" => Some(Color { r: 5, g: 144, b: 51 }),
        "ocean boat blue" => Some(Color { r: 0, g: 119, b: 190 }),
        "ochre" => Some(Color { r: 204, g: 119, b: 34 }),
        "office green" => Some(Color { r: 0, g: 128, b: 0 }),
        "old gold" => Some(Color { r: 207, g: 181, b: 59 }),
        "old lace" => Some(Color { r: 253, g: 245, b: 230 }),
        "old lavender" => Some(Color { r: 121, g: 104, b: 120 }),
        "old mauve" => Some(Color { r: 103, g: 49, b: 71 }),
        "old rose" => Some(Color { r: 192, g: 128, b: 129 }),
        "olive" => Some(Color { r: 128, g: 128, b: 0 }),
        "olive drab #7" => Some(Color { r: 60, g: 52, b: 31 }),
        "olive drab (web) (olive drab #3)" => Some(Color { r: 107, g: 142, b: 35 }),
        "olivine" => Some(Color { r: 154, g: 185, b: 115 }),
        "onyx" => Some(Color { r: 53, g: 56, b: 57 }),
        "opera mauve" => Some(Color { r: 183, g: 132, b: 167 }),
        "orange (color wheel)" => Some(Color { r: 255, g: 127, b: 0 }),
        "orange peel" => Some(Color { r: 255, g: 159, b: 0 }),
        "orange-red" => Some(Color { r: 255, g: 69, b: 0 }),
        "orange (ryb)" => Some(Color { r: 251, g: 153, b: 2 }),
        "orange (web color)" => Some(Color { r: 255, g: 165, b: 0 }),
        "orchid" => Some(Color { r: 218, g: 112, b: 214 }),
        "otter brown" => Some(Color { r: 101, g: 67, b: 33 }),
        "ou crimson red" => Some(Color { r: 153, g: 0, b: 0 }),
        "outer space" => Some(Color { r: 65, g: 74, b: 76 }),
        "outrageous orange" => Some(Color { r: 255, g: 110, b: 74 }),
        "oxford blue" => Some(Color { r: 0, g: 33, b: 71 }),
        "pakistan green" => Some(Color { r: 0, g: 102, b: 0 }),
        "palatinate blue" => Some(Color { r: 39, g: 59, b: 226 }),
        "palatinate purple" => Some(Color { r: 104, g: 40, b: 96 }),
        "pale aqua" => Some(Color { r: 188, g: 212, b: 230 }),
        "pale blue" => Some(Color { r: 175, g: 238, b: 238 }),
        "pale brown" => Some(Color { r: 152, g: 118, b: 84 }),
        "pale carmine" => Some(Color { r: 175, g: 64, b: 53 }),
        "pale cerulean" => Some(Color { r: 155, g: 196, b: 226 }),
        "pale chestnut" => Some(Color { r: 221, g: 173, b: 175 }),
        "pale copper" => Some(Color { r: 218, g: 138, b: 103 }),
        "pale cornflower blue" => Some(Color { r: 171, g: 205, b: 239 }),
        "pale gold" => Some(Color { r: 230, g: 190, b: 138 }),
        "pale goldenrod" => Some(Color { r: 238, g: 232, b: 170 }),
        "pale green" => Some(Color { r: 152, g: 251, b: 152 }),
        "pale lavender" => Some(Color { r: 220, g: 208, b: 255 }),
        "pale magenta" => Some(Color { r: 249, g: 132, b: 229 }),
        "pale pink" => Some(Color { r: 250, g: 218, b: 221 }),
        "pale plum" => Some(Color { r: 221, g: 160, b: 221 }),
        "pale red-violet" => Some(Color { r: 219, g: 112, b: 147 }),
        "pale robin egg blue" => Some(Color { r: 150, g: 222, b: 209 }),
        "pale silver" => Some(Color { r: 201, g: 192, b: 187 }),
        "pale spring bud" => Some(Color { r: 236, g: 235, b: 189 }),
        "pale taupe" => Some(Color { r: 188, g: 152, b: 126 }),
        "pale violet-red" => Some(Color { r: 219, g: 112, b: 147 }),
        "pansy purple" => Some(Color { r: 120, g: 24, b: 74 }),
        "papaya whip" => Some(Color { r: 255, g: 239, b: 213 }),
        "paris green" => Some(Color { r: 80, g: 200, b: 120 }),
        "pastel blue" => Some(Color { r: 174, g: 198, b: 207 }),
        "pastel brown" => Some(Color { r: 131, g: 105, b: 83 }),
        "pastel gray" => Some(Color { r: 207, g: 207, b: 196 }),
        "pastel green" => Some(Color { r: 119, g: 221, b: 119 }),
        "pastel magenta" => Some(Color { r: 244, g: 154, b: 194 }),
        "pastel orange" => Some(Color { r: 255, g: 179, b: 71 }),
        "pastel pink" => Some(Color { r: 222, g: 165, b: 164 }),
        "pastel purple" => Some(Color { r: 179, g: 158, b: 181 }),
        "pastel red" => Some(Color { r: 255, g: 105, b: 97 }),
        "pastel violet" => Some(Color { r: 203, g: 153, b: 201 }),
        "pastel yellow" => Some(Color { r: 253, g: 253, b: 150 }),
        "patriarch" => Some(Color { r: 128, g: 0, b: 128 }),
        "payne's grey" => Some(Color { r: 83, g: 104, b: 120 }),
        "peach" => Some(Color { r: 255, g: 229, b: 180 }),
        "peach (crayola)" => Some(Color { r: 255, g: 203, b: 164 }),
        "peach-orange" => Some(Color { r: 255, g: 204, b: 153 }),
        "peach puff" => Some(Color { r: 255, g: 218, b: 185 }),
        "peach-yellow" => Some(Color { r: 250, g: 223, b: 173 }),
        "pear" => Some(Color { r: 209, g: 226, b: 49 }),
        "pearl" => Some(Color { r: 234, g: 224, b: 200 }),
        "pearl aqua" => Some(Color { r: 136, g: 216, b: 192 }),
        "pearly purple" => Some(Color { r: 183, g: 104, b: 162 }),
        "peridot" => Some(Color { r: 230, g: 226, b: 0 }),
        "periwinkle" => Some(Color { r: 204, g: 204, b: 255 }),
        "persian blue" => Some(Color { r: 28, g: 57, b: 187 }),
        "persian green" => Some(Color { r: 0, g: 166, b: 147 }),
        "persian indigo" => Some(Color { r: 50, g: 18, b: 122 }),
        "persian orange" => Some(Color { r: 217, g: 144, b: 88 }),
        "persian pink" => Some(Color { r: 247, g: 127, b: 190 }),
        "persian plum" => Some(Color { r: 112, g: 28, b: 28 }),
        "persian red" => Some(Color { r: 204, g: 51, b: 51 }),
        "persian rose" => Some(Color { r: 254, g: 40, b: 162 }),
        "persimmon" => Some(Color { r: 236, g: 88, b: 0 }),
        "peru" => Some(Color { r: 205, g: 133, b: 63 }),
        "phlox" => Some(Color { r: 223, g: 0, b: 255 }),
        "phthalo blue" => Some(Color { r: 0, g: 15, b: 137 }),
        "phthalo green" => Some(Color { r: 18, g: 53, b: 36 }),
        "piggy pink" => Some(Color { r: 253, g: 221, b: 230 }),
        "pine green" => Some(Color { r: 1, g: 121, b: 111 }),
        "pink" => Some(Color { r: 255, g: 192, b: 203 }),
        "pink lace" => Some(Color { r: 255, g: 221, b: 244 }),
        "pink-orange" => Some(Color { r: 255, g: 153, b: 102 }),
        "pink pearl" => Some(Color { r: 231, g: 172, b: 207 }),
        "pink sherbet" => Some(Color { r: 247, g: 143, b: 167 }),
        "pistachio" => Some(Color { r: 147, g: 197, b: 114 }),
        "platinum" => Some(Color { r: 229, g: 228, b: 226 }),
        "plum (traditional)" => Some(Color { r: 142, g: 69, b: 133 }),
        "plum (web)" => Some(Color { r: 221, g: 160, b: 221 }),
        "portland orange" => Some(Color { r: 255, g: 90, b: 54 }),
        "powder blue (web)" => Some(Color { r: 176, g: 224, b: 230 }),
        "princeton orange" => Some(Color { r: 255, g: 143, b: 0 }),
        "prune" => Some(Color { r: 112, g: 28, b: 28 }),
        "prussian blue" => Some(Color { r: 0, g: 49, b: 83 }),
        "psychedelic purple" => Some(Color { r: 223, g: 0, b: 255 }),
        "puce" => Some(Color { r: 204, g: 136, b: 153 }),
        "pumpkin" => Some(Color { r: 255, g: 117, b: 24 }),
        "purple heart" => Some(Color { r: 105, g: 53, b: 156 }),
        "purple (html/css)" => Some(Color { r: 128, g: 0, b: 128 }),
        "purple mountain majesty" => Some(Color { r: 150, g: 120, b: 182 }),
        "purple (munsell)" => Some(Color { r: 159, g: 0, b: 197 }),
        "purple pizzazz" => Some(Color { r: 254, g: 78, b: 218 }),
        "purple taupe" => Some(Color { r: 80, g: 64, b: 77 }),
        "purple (x11)" => Some(Color { r: 160, g: 32, b: 240 }),
        "quartz" => Some(Color { r: 81, g: 72, b: 79 }),
        "rackley" => Some(Color { r: 93, g: 138, b: 168 }),
        "radical red" => Some(Color { r: 255, g: 53, b: 94 }),
        "rajah" => Some(Color { r: 251, g: 171, b: 96 }),
        "raspberry" => Some(Color { r: 227, g: 11, b: 93 }),
        "raspberry glace" => Some(Color { r: 145, g: 95, b: 109 }),
        "raspberry pink" => Some(Color { r: 226, g: 80, b: 152 }),
        "raspberry rose" => Some(Color { r: 179, g: 68, b: 108 }),
        "raw umber" => Some(Color { r: 130, g: 102, b: 68 }),
        "razzle dazzle rose" => Some(Color { r: 255, g: 51, b: 204 }),
        "razzmatazz" => Some(Color { r: 227, g: 37, b: 107 }),
        "red" => Some(Color { r: 255, g: 0, b: 0 }),
        "red-brown" => Some(Color { r: 165, g: 42, b: 42 }),
        "red devil" => Some(Color { r: 134, g: 1, b: 17 }),
        "red (munsell)" => Some(Color { r: 242, g: 0, b: 60 }),
        "red (ncs)" => Some(Color { r: 196, g: 2, b: 51 }),
        "red-orange" => Some(Color { r: 255, g: 83, b: 73 }),
        "red (pigment)" => Some(Color { r: 237, g: 28, b: 36 }),
        "red (ryb)" => Some(Color { r: 254, g: 39, b: 18 }),
        "red-violet" => Some(Color { r: 199, g: 21, b: 133 }),
        "redwood" => Some(Color { r: 171, g: 78, b: 82 }),
        "regalia" => Some(Color { r: 82, g: 45, b: 128 }),
        "resolution blue" => Some(Color { r: 0, g: 35, b: 135 }),
        "rich black" => Some(Color { r: 0, g: 64, b: 64 }),
        "rich brilliant lavender" => Some(Color { r: 241, g: 167, b: 254 }),
        "rich carmine" => Some(Color { r: 215, g: 0, b: 64 }),
        "rich electric blue" => Some(Color { r: 8, g: 146, b: 208 }),
        "rich lavender" => Some(Color { r: 167, g: 107, b: 207 }),
        "rich lilac" => Some(Color { r: 182, g: 102, b: 210 }),
        "rich maroon" => Some(Color { r: 176, g: 48, b: 96 }),
        "rifle green" => Some(Color { r: 65, g: 72, b: 51 }),
        "robin egg blue" => Some(Color { r: 0, g: 204, b: 204 }),
        "rose" => Some(Color { r: 255, g: 0, b: 127 }),
        "rose bonbon" => Some(Color { r: 249, g: 66, b: 158 }),
        "rose ebony" => Some(Color { r: 103, g: 72, b: 70 }),
        "rose gold" => Some(Color { r: 183, g: 110, b: 121 }),
        "rose madder" => Some(Color { r: 227, g: 38, b: 54 }),
        "rose pink" => Some(Color { r: 255, g: 102, b: 204 }),
        "rose quartz" => Some(Color { r: 170, g: 152, b: 169 }),
        "rose taupe" => Some(Color { r: 144, g: 93, b: 93 }),
        "rose vale" => Some(Color { r: 171, g: 78, b: 82 }),
        "rosewood" => Some(Color { r: 101, g: 0, b: 11 }),
        "rosso corsa" => Some(Color { r: 212, g: 0, b: 0 }),
        "rosy brown" => Some(Color { r: 188, g: 143, b: 143 }),
        "royal azure" => Some(Color { r: 0, g: 56, b: 168 }),
        "royal blue (traditional)" => Some(Color { r: 0, g: 35, b: 102 }),
        "royal blue (web)" => Some(Color { r: 65, g: 105, b: 225 }),
        "royal fuchsia" => Some(Color { r: 202, g: 44, b: 146 }),
        "royal purple" => Some(Color { r: 120, g: 81, b: 169 }),
        "royal yellow" => Some(Color { r: 250, g: 218, b: 94 }),
        "rubine red" => Some(Color { r: 209, g: 0, b: 86 }),
        "ruby" => Some(Color { r: 224, g: 17, b: 95 }),
        "ruby red" => Some(Color { r: 155, g: 17, b: 30 }),
        "ruddy" => Some(Color { r: 255, g: 0, b: 40 }),
        "ruddy brown" => Some(Color { r: 187, g: 101, b: 40 }),
        "ruddy pink" => Some(Color { r: 225, g: 142, b: 150 }),
        "rufous" => Some(Color { r: 168, g: 28, b: 7 }),
        "russet" => Some(Color { r: 128, g: 70, b: 27 }),
        "rust" => Some(Color { r: 183, g: 65, b: 14 }),
        "rusty red" => Some(Color { r: 218, g: 44, b: 67 }),
        "sacramento state green" => Some(Color { r: 0, g: 86, b: 63 }),
        "saddle brown" => Some(Color { r: 139, g: 69, b: 19 }),
        "safety orange (blaze orange)" => Some(Color { r: 255, g: 103, b: 0 }),
        "saffron" => Some(Color { r: 244, g: 196, b: 48 }),
        "salmon" => Some(Color { r: 255, g: 140, b: 105 }),
        "salmon pink" => Some(Color { r: 255, g: 145, b: 164 }),
        "sand" => Some(Color { r: 194, g: 178, b: 128 }),
        "sand dune" => Some(Color { r: 150, g: 113, b: 23 }),
        "sandstorm" => Some(Color { r: 236, g: 213, b: 64 }),
        "sandy brown" => Some(Color { r: 244, g: 164, b: 96 }),
        "sandy taupe" => Some(Color { r: 150, g: 113, b: 23 }),
        "sangria" => Some(Color { r: 146, g: 0, b: 10 }),
        "sap green" => Some(Color { r: 80, g: 125, b: 42 }),
        "sapphire" => Some(Color { r: 15, g: 82, b: 186 }),
        "sapphire blue" => Some(Color { r: 0, g: 103, b: 165 }),
        "satin sheen gold" => Some(Color { r: 203, g: 161, b: 53 }),
        "scarlet" => Some(Color { r: 255, g: 36, b: 0 }),
        "scarlet (crayola)" => Some(Color { r: 253, g: 14, b: 53 }),
        "school bus yellow" => Some(Color { r: 255, g: 216, b: 0 }),
        "screamin' green" => Some(Color { r: 118, g: 255, b: 122 }),
        "sea blue" => Some(Color { r: 0, g: 105, b: 148 }),
        "sea green" => Some(Color { r: 46, g: 139, b: 87 }),
        "seal brown" => Some(Color { r: 50, g: 20, b: 20 }),
        "seashell" => Some(Color { r: 255, g: 245, b: 238 }),
        "selective yellow" => Some(Color { r: 255, g: 186, b: 0 }),
        "sepia" => Some(Color { r: 112, g: 66, b: 20 }),
        "shadow" => Some(Color { r: 138, g: 121, b: 93 }),
        "shamrock green" => Some(Color { r: 0, g: 158, b: 96 }),
        "shocking pink" => Some(Color { r: 252, g: 15, b: 192 }),
        "shocking pink (crayola)" => Some(Color { r: 255, g: 111, b: 255 }),
        "sienna" => Some(Color { r: 136, g: 45, b: 23 }),
        "silver" => Some(Color { r: 192, g: 192, b: 192 }),
        "sinopia" => Some(Color { r: 203, g: 65, b: 11 }),
        "skobeloff" => Some(Color { r: 0, g: 116, b: 116 }),
        "sky blue" => Some(Color { r: 135, g: 206, b: 235 }),
        "sky magenta" => Some(Color { r: 207, g: 113, b: 175 }),
        "slate blue" => Some(Color { r: 106, g: 90, b: 205 }),
        "slate gray" => Some(Color { r: 112, g: 128, b: 144 }),
        "smalt (dark powder blue)" => Some(Color { r: 0, g: 51, b: 153 }),
        "smokey topaz" => Some(Color { r: 147, g: 61, b: 65 }),
        "smoky black" => Some(Color { r: 16, g: 12, b: 8 }),
        "snow" => Some(Color { r: 255, g: 250, b: 250 }),
        "spiro disco ball" => Some(Color { r: 15, g: 192, b: 252 }),
        "spring bud" => Some(Color { r: 167, g: 252, b: 0 }),
        "spring green" => Some(Color { r: 0, g: 255, b: 127 }),
        "st. patrick's blue" => Some(Color { r: 35, g: 41, b: 122 }),
        "steel blue" => Some(Color { r: 70, g: 130, b: 180 }),
        "stil de grain yellow" => Some(Color { r: 250, g: 218, b: 94 }),
        "stizza" => Some(Color { r: 153, g: 0, b: 0 }),
        "stormcloud" => Some(Color { r: 79, g: 102, b: 106 }),
        "straw" => Some(Color { r: 228, g: 217, b: 111 }),
        "sunglow" => Some(Color { r: 255, g: 204, b: 51 }),
        "sunset" => Some(Color { r: 250, g: 214, b: 165 }),
        "tan" => Some(Color { r: 210, g: 180, b: 140 }),
        "tangelo" => Some(Color { r: 249, g: 77, b: 0 }),
        "tangerine" => Some(Color { r: 242, g: 133, b: 0 }),
        "tangerine yellow" => Some(Color { r: 255, g: 204, b: 0 }),
        "tango pink" => Some(Color { r: 228, g: 113, b: 122 }),
        "taupe" => Some(Color { r: 72, g: 60, b: 50 }),
        "taupe gray" => Some(Color { r: 139, g: 133, b: 137 }),
        "tea green" => Some(Color { r: 208, g: 240, b: 192 }),
        "tea rose (orange)" => Some(Color { r: 248, g: 131, b: 121 }),
        "tea rose (rose)" => Some(Color { r: 244, g: 194, b: 194 }),
        "teal" => Some(Color { r: 0, g: 128, b: 128 }),
        "teal blue" => Some(Color { r: 54, g: 117, b: 136 }),
        "teal green" => Some(Color { r: 0, g: 130, b: 127 }),
        "telemagenta" => Some(Color { r: 207, g: 52, b: 118 }),
        "tenne (tawny)" => Some(Color { r: 205, g: 87, b: 0 }),
        "terra cotta" => Some(Color { r: 226, g: 114, b: 91 }),
        "thistle" => Some(Color { r: 216, g: 191, b: 216 }),
        "thulian pink" => Some(Color { r: 222, g: 111, b: 161 }),
        "tickle me pink" => Some(Color { r: 252, g: 137, b: 172 }),
        "tiffany blue" => Some(Color { r: 10, g: 186, b: 181 }),
        "tiger's eye" => Some(Color { r: 224, g: 141, b: 60 }),
        "timberwolf" => Some(Color { r: 219, g: 215, b: 210 }),
        "titanium yellow" => Some(Color { r: 238, g: 230, b: 0 }),
        "tomato" => Some(Color { r: 255, g: 99, b: 71 }),
        "toolbox" => Some(Color { r: 116, g: 108, b: 192 }),
        "topaz" => Some(Color { r: 255, g: 200, b: 124 }),
        "tractor red" => Some(Color { r: 253, g: 14, b: 53 }),
        "trolley grey" => Some(Color { r: 128, g: 128, b: 128 }),
        "tropical rain forest" => Some(Color { r: 0, g: 117, b: 94 }),
        "true blue" => Some(Color { r: 0, g: 115, b: 207 }),
        "tufts blue" => Some(Color { r: 65, g: 125, b: 193 }),
        "tumbleweed" => Some(Color { r: 222, g: 170, b: 136 }),
        "turkish rose" => Some(Color { r: 181, g: 114, b: 129 }),
        "turquoise" => Some(Color { r: 48, g: 213, b: 200 }),
        "turquoise blue" => Some(Color { r: 0, g: 255, b: 239 }),
        "turquoise green" => Some(Color { r: 160, g: 214, b: 180 }),
        "tuscan red" => Some(Color { r: 124, g: 72, b: 72 }),
        "twilight lavender" => Some(Color { r: 138, g: 73, b: 107 }),
        "tyrian purple" => Some(Color { r: 102, g: 2, b: 60 }),
        "ua blue" => Some(Color { r: 0, g: 51, b: 170 }),
        "ua red" => Some(Color { r: 217, g: 0, b: 76 }),
        "ube" => Some(Color { r: 136, g: 120, b: 195 }),
        "ucla blue" => Some(Color { r: 83, g: 104, b: 149 }),
        "ucla gold" => Some(Color { r: 255, g: 179, b: 0 }),
        "ufo green" => Some(Color { r: 60, g: 208, b: 112 }),
        "ultra pink" => Some(Color { r: 255, g: 111, b: 255 }),
        "ultramarine" => Some(Color { r: 18, g: 10, b: 143 }),
        "ultramarine blue" => Some(Color { r: 65, g: 102, b: 245 }),
        "umber" => Some(Color { r: 99, g: 81, b: 71 }),
        "unbleached silk" => Some(Color { r: 255, g: 221, b: 202 }),
        "united nations blue" => Some(Color { r: 91, g: 146, b: 229 }),
        "university of california gold" => Some(Color { r: 183, g: 135, b: 39 }),
        "unmellow yellow" => Some(Color { r: 255, g: 255, b: 102 }),
        "up forest green" => Some(Color { r: 1, g: 68, b: 33 }),
        "up maroon" => Some(Color { r: 123, g: 17, b: 19 }),
        "upsdell red" => Some(Color { r: 174, g: 32, b: 41 }),
        "urobilin" => Some(Color { r: 225, g: 173, b: 33 }),
        "usafa blue" => Some(Color { r: 0, g: 79, b: 152 }),
        "usc cardinal" => Some(Color { r: 153, g: 0, b: 0 }),
        "usc gold" => Some(Color { r: 255, g: 204, b: 0 }),
        "utah crimson" => Some(Color { r: 211, g: 0, b: 63 }),
        "vanilla" => Some(Color { r: 243, g: 229, b: 171 }),
        "vegas gold" => Some(Color { r: 197, g: 179, b: 88 }),
        "venetian red" => Some(Color { r: 200, g: 8, b: 21 }),
        "verdigris" => Some(Color { r: 67, g: 179, b: 174 }),
        "vermilion (cinnabar)" => Some(Color { r: 227, g: 66, b: 52 }),
        "vermilion (plochere)" => Some(Color { r: 217, g: 96, b: 59 }),
        "veronica" => Some(Color { r: 160, g: 32, b: 240 }),
        "violet" => Some(Color { r: 143, g: 0, b: 255 }),
        "violet-blue" => Some(Color { r: 50, g: 74, b: 178 }),
        "violet (color wheel)" => Some(Color { r: 127, g: 0, b: 255 }),
        "violet (ryb)" => Some(Color { r: 134, g: 1, b: 175 }),
        "violet (web)" => Some(Color { r: 238, g: 130, b: 238 }),
        "viridian" => Some(Color { r: 64, g: 130, b: 109 }),
        "vivid auburn" => Some(Color { r: 146, g: 39, b: 36 }),
        "vivid burgundy" => Some(Color { r: 159, g: 29, b: 53 }),
        "vivid cerise" => Some(Color { r: 218, g: 29, b: 129 }),
        "vivid tangerine" => Some(Color { r: 255, g: 160, b: 137 }),
        "vivid violet" => Some(Color { r: 159, g: 0, b: 255 }),
        "warm black" => Some(Color { r: 0, g: 66, b: 66 }),
        "waterspout" => Some(Color { r: 164, g: 244, b: 249 }),
        "wenge" => Some(Color { r: 100, g: 84, b: 82 }),
        "wheat" => Some(Color { r: 245, g: 222, b: 179 }),
        "white" => Some(Color { r: 255, g: 255, b: 255 }),
        "white smoke" => Some(Color { r: 245, g: 245, b: 245 }),
        "wild blue yonder" => Some(Color { r: 162, g: 173, b: 208 }),
        "wild strawberry" => Some(Color { r: 255, g: 67, b: 164 }),
        "wild watermelon" => Some(Color { r: 252, g: 108, b: 133 }),
        "wine" => Some(Color { r: 114, g: 47, b: 55 }),
        "wine dregs" => Some(Color { r: 103, g: 49, b: 71 }),
        "wisteria" => Some(Color { r: 201, g: 160, b: 220 }),
        "wood brown" => Some(Color { r: 193, g: 154, b: 107 }),
        "xanadu" => Some(Color { r: 115, g: 134, b: 120 }),
        "yale blue" => Some(Color { r: 15, g: 77, b: 146 }),
        "yellow" => Some(Color { r: 255, g: 255, b: 0 }),
        "yellow-green" => Some(Color { r: 154, g: 205, b: 50 }),
        "yellow (munsell)" => Some(Color { r: 239, g: 204, b: 0 }),
        "yellow (ncs)" => Some(Color { r: 255, g: 211, b: 0 }),
        "yellow orange" => Some(Color { r: 255, g: 174, b: 66 }),
        "yellow (process)" => Some(Color { r: 255, g: 239, b: 0 }),
        "yellow (ryb)" => Some(Color { r: 254, g: 254, b: 51 }),
        "zaffre" => Some(Color { r: 0, g: 20, b: 168 }),
        "zinnwaldite brown" => Some(Color { r: 44, g: 22, b: 8 }),
        _ => None,
    }
}

lazy_static! {
    static ref COLORS_MAP: HashMap<&'static str, Color> = {
        let mut map = HashMap::new();
        map.insert("air force blue (raf)", Color { r: 93, g: 138, b: 168 });
        map.insert("air force blue (usaf)", Color { r: 0, g: 48, b: 143 });
        map.insert("air superiority blue", Color { r: 114, g: 160, b: 193 });
        map.insert("alabama crimson", Color { r: 163, g: 38, b: 56 });
        map.insert("alice blue", Color { r: 240, g: 248, b: 255 });
        map.insert("alizarin crimson", Color { r: 227, g: 38, b: 54 });
        map.insert("alloy orange", Color { r: 196, g: 98, b: 16 });
        map.insert("almond", Color { r: 239, g: 222, b: 205 });
        map.insert("amaranth", Color { r: 229, g: 43, b: 80 });
        map.insert("amber", Color { r: 255, g: 191, b: 0 });
        map.insert("amber (sae/ece)", Color { r: 255, g: 126, b: 0 });
        map.insert("american rose", Color { r: 255, g: 3, b: 62 });
        map.insert("amethyst", Color { r: 153, g: 102, b: 204 });
        map.insert("android green", Color { r: 164, g: 198, b: 57 });
        map.insert("anti-flash white", Color { r: 242, g: 243, b: 244 });
        map.insert("antique brass", Color { r: 205, g: 149, b: 117 });
        map.insert("antique fuchsia", Color { r: 145, g: 92, b: 131 });
        map.insert("antique ruby", Color { r: 132, g: 27, b: 45 });
        map.insert("antique white", Color { r: 250, g: 235, b: 215 });
        map.insert("ao (english)", Color { r: 0, g: 128, b: 0 });
        map.insert("apple green", Color { r: 141, g: 182, b: 0 });
        map.insert("apricot", Color { r: 251, g: 206, b: 177 });
        map.insert("aqua", Color { r: 0, g: 255, b: 255 });
        map.insert("aquamarine", Color { r: 127, g: 255, b: 212 });
        map.insert("army green", Color { r: 75, g: 83, b: 32 });
        map.insert("arsenic", Color { r: 59, g: 68, b: 75 });
        map.insert("arylide yellow", Color { r: 233, g: 214, b: 107 });
        map.insert("ash grey", Color { r: 178, g: 190, b: 181 });
        map.insert("asparagus", Color { r: 135, g: 169, b: 107 });
        map.insert("atomic tangerine", Color { r: 255, g: 153, b: 102 });
        map.insert("auburn", Color { r: 165, g: 42, b: 42 });
        map.insert("aureolin", Color { r: 253, g: 238, b: 0 });
        map.insert("aurometalsaurus", Color { r: 110, g: 127, b: 128 });
        map.insert("avocado", Color { r: 86, g: 130, b: 3 });
        map.insert("azure", Color { r: 0, g: 127, b: 255 });
        map.insert("azure mist/web", Color { r: 240, g: 255, b: 255 });
        map.insert("baby blue", Color { r: 137, g: 207, b: 240 });
        map.insert("baby blue eyes", Color { r: 161, g: 202, b: 241 });
        map.insert("baby pink", Color { r: 244, g: 194, b: 194 });
        map.insert("ball blue", Color { r: 33, g: 171, b: 205 });
        map.insert("banana mania", Color { r: 250, g: 231, b: 181 });
        map.insert("banana yellow", Color { r: 255, g: 225, b: 53 });
        map.insert("barn red", Color { r: 124, g: 10, b: 2 });
        map.insert("battleship grey", Color { r: 132, g: 132, b: 130 });
        map.insert("bazaar", Color { r: 152, g: 119, b: 123 });
        map.insert("beau blue", Color { r: 188, g: 212, b: 230 });
        map.insert("beaver", Color { r: 159, g: 129, b: 112 });
        map.insert("beige", Color { r: 245, g: 245, b: 220 });
        map.insert("big dip o’ruby", Color { r: 156, g: 37, b: 66 });
        map.insert("bisque", Color { r: 255, g: 228, b: 196 });
        map.insert("bistre", Color { r: 61, g: 43, b: 31 });
        map.insert("bittersweet", Color { r: 254, g: 111, b: 94 });
        map.insert("bittersweet shimmer", Color { r: 191, g: 79, b: 81 });
        map.insert("black", Color { r: 0, g: 0, b: 0 });
        map.insert("black bean", Color { r: 61, g: 12, b: 2 });
        map.insert("black leather jacket", Color { r: 37, g: 53, b: 41 });
        map.insert("black olive", Color { r: 59, g: 60, b: 54 });
        map.insert("blanched almond", Color { r: 255, g: 235, b: 205 });
        map.insert("blast-off bronze", Color { r: 165, g: 113, b: 100 });
        map.insert("bleu de france", Color { r: 49, g: 140, b: 231 });
        map.insert("blizzard blue", Color { r: 172, g: 229, b: 238 });
        map.insert("blond", Color { r: 250, g: 240, b: 190 });
        map.insert("blue", Color { r: 0, g: 0, b: 255 });
        map.insert("blue bell", Color { r: 162, g: 162, b: 208 });
        map.insert("blue (crayola)", Color { r: 31, g: 117, b: 254 });
        map.insert("blue gray", Color { r: 102, g: 153, b: 204 });
        map.insert("blue-green", Color { r: 13, g: 152, b: 186 });
        map.insert("blue (munsell)", Color { r: 0, g: 147, b: 175 });
        map.insert("blue (ncs)", Color { r: 0, g: 135, b: 189 });
        map.insert("blue (pigment)", Color { r: 51, g: 51, b: 153 });
        map.insert("blue (ryb)", Color { r: 2, g: 71, b: 254 });
        map.insert("blue sapphire", Color { r: 18, g: 97, b: 128 });
        map.insert("blue-violet", Color { r: 138, g: 43, b: 226 });
        map.insert("blush", Color { r: 222, g: 93, b: 131 });
        map.insert("bole", Color { r: 121, g: 68, b: 59 });
        map.insert("bondi blue", Color { r: 0, g: 149, b: 182 });
        map.insert("bone", Color { r: 227, g: 218, b: 201 });
        map.insert("boston university red", Color { r: 204, g: 0, b: 0 });
        map.insert("bottle green", Color { r: 0, g: 106, b: 78 });
        map.insert("boysenberry", Color { r: 135, g: 50, b: 96 });
        map.insert("brandeis blue", Color { r: 0, g: 112, b: 255 });
        map.insert("brass", Color { r: 181, g: 166, b: 66 });
        map.insert("brick red", Color { r: 203, g: 65, b: 84 });
        map.insert("bright cerulean", Color { r: 29, g: 172, b: 214 });
        map.insert("bright green", Color { r: 102, g: 255, b: 0 });
        map.insert("bright lavender", Color { r: 191, g: 148, b: 228 });
        map.insert("bright maroon", Color { r: 195, g: 33, b: 72 });
        map.insert("bright pink", Color { r: 255, g: 0, b: 127 });
        map.insert("bright turquoise", Color { r: 8, g: 232, b: 222 });
        map.insert("bright ube", Color { r: 209, g: 159, b: 232 });
        map.insert("brilliant lavender", Color { r: 244, g: 187, b: 255 });
        map.insert("brilliant rose", Color { r: 255, g: 85, b: 163 });
        map.insert("brink pink", Color { r: 251, g: 96, b: 127 });
        map.insert("british racing green", Color { r: 0, g: 66, b: 37 });
        map.insert("bronze", Color { r: 205, g: 127, b: 50 });
        map.insert("brown (traditional)", Color { r: 150, g: 75, b: 0 });
        map.insert("brown (web)", Color { r: 165, g: 42, b: 42 });
        map.insert("bubble gum", Color { r: 255, g: 193, b: 204 });
        map.insert("bubbles", Color { r: 231, g: 254, b: 255 });
        map.insert("buff", Color { r: 240, g: 220, b: 130 });
        map.insert("bulgarian rose", Color { r: 72, g: 6, b: 7 });
        map.insert("burgundy", Color { r: 128, g: 0, b: 32 });
        map.insert("burlywood", Color { r: 222, g: 184, b: 135 });
        map.insert("burnt orange", Color { r: 204, g: 85, b: 0 });
        map.insert("burnt sienna", Color { r: 233, g: 116, b: 81 });
        map.insert("burnt umber", Color { r: 138, g: 51, b: 36 });
        map.insert("byzantine", Color { r: 189, g: 51, b: 164 });
        map.insert("byzantium", Color { r: 112, g: 41, b: 99 });
        map.insert("cadet", Color { r: 83, g: 104, b: 114 });
        map.insert("cadet blue", Color { r: 95, g: 158, b: 160 });
        map.insert("cadet grey", Color { r: 145, g: 163, b: 176 });
        map.insert("cadmium green", Color { r: 0, g: 107, b: 60 });
        map.insert("cadmium orange", Color { r: 237, g: 135, b: 45 });
        map.insert("cadmium red", Color { r: 227, g: 0, b: 34 });
        map.insert("cadmium yellow", Color { r: 255, g: 246, b: 0 });
        map.insert("cafe au lait", Color { r: 166, g: 123, b: 91 });
        map.insert("cafe noir", Color { r: 75, g: 54, b: 33 });
        map.insert("cal poly green", Color { r: 30, g: 77, b: 43 });
        map.insert("cambridge blue", Color { r: 163, g: 193, b: 173 });
        map.insert("camel", Color { r: 193, g: 154, b: 107 });
        map.insert("cameo pink", Color { r: 239, g: 187, b: 204 });
        map.insert("camouflage green", Color { r: 120, g: 134, b: 107 });
        map.insert("canary yellow", Color { r: 255, g: 239, b: 0 });
        map.insert("candy apple red", Color { r: 255, g: 8, b: 0 });
        map.insert("candy pink", Color { r: 228, g: 113, b: 122 });
        map.insert("capri", Color { r: 0, g: 191, b: 255 });
        map.insert("caput mortuum", Color { r: 89, g: 39, b: 32 });
        map.insert("cardinal", Color { r: 196, g: 30, b: 58 });
        map.insert("caribbean green", Color { r: 0, g: 204, b: 153 });
        map.insert("carmine", Color { r: 150, g: 0, b: 24 });
        map.insert("carmine (m&p)", Color { r: 215, g: 0, b: 64 });
        map.insert("carmine pink", Color { r: 235, g: 76, b: 66 });
        map.insert("carmine red", Color { r: 255, g: 0, b: 56 });
        map.insert("carnation pink", Color { r: 255, g: 166, b: 201 });
        map.insert("carnelian", Color { r: 179, g: 27, b: 27 });
        map.insert("carolina blue", Color { r: 153, g: 186, b: 221 });
        map.insert("carrot orange", Color { r: 237, g: 145, b: 33 });
        map.insert("catalina blue", Color { r: 6, g: 42, b: 120 });
        map.insert("ceil", Color { r: 146, g: 161, b: 207 });
        map.insert("celadon", Color { r: 172, g: 225, b: 175 });
        map.insert("celadon blue", Color { r: 0, g: 123, b: 167 });
        map.insert("celadon green", Color { r: 47, g: 132, b: 124 });
        map.insert("celeste (colour)", Color { r: 178, g: 255, b: 255 });
        map.insert("celestial blue", Color { r: 73, g: 151, b: 208 });
        map.insert("cerise", Color { r: 222, g: 49, b: 99 });
        map.insert("cerise pink", Color { r: 236, g: 59, b: 131 });
        map.insert("cerulean", Color { r: 0, g: 123, b: 167 });
        map.insert("cerulean blue", Color { r: 42, g: 82, b: 190 });
        map.insert("cerulean frost", Color { r: 109, g: 155, b: 195 });
        map.insert("cg blue", Color { r: 0, g: 122, b: 165 });
        map.insert("cg red", Color { r: 224, g: 60, b: 49 });
        map.insert("chamoisee", Color { r: 160, g: 120, b: 90 });
        map.insert("champagne", Color { r: 250, g: 214, b: 165 });
        map.insert("charcoal", Color { r: 54, g: 69, b: 79 });
        map.insert("charm pink", Color { r: 230, g: 143, b: 172 });
        map.insert("chartreuse (traditional)", Color { r: 223, g: 255, b: 0 });
        map.insert("chartreuse (web)", Color { r: 127, g: 255, b: 0 });
        map.insert("cherry", Color { r: 222, g: 49, b: 99 });
        map.insert("cherry blossom pink", Color { r: 255, g: 183, b: 197 });
        map.insert("chestnut", Color { r: 205, g: 92, b: 92 });
        map.insert("china pink", Color { r: 222, g: 111, b: 161 });
        map.insert("china rose", Color { r: 168, g: 81, b: 110 });
        map.insert("chinese red", Color { r: 170, g: 56, b: 30 });
        map.insert("chocolate (traditional)", Color { r: 123, g: 63, b: 0 });
        map.insert("chocolate (web)", Color { r: 210, g: 105, b: 30 });
        map.insert("chrome yellow", Color { r: 255, g: 167, b: 0 });
        map.insert("cinereous", Color { r: 152, g: 129, b: 123 });
        map.insert("cinnabar", Color { r: 227, g: 66, b: 52 });
        map.insert("cinnamon", Color { r: 210, g: 105, b: 30 });
        map.insert("citrine", Color { r: 228, g: 208, b: 10 });
        map.insert("classic rose", Color { r: 251, g: 204, b: 231 });
        map.insert("cobalt", Color { r: 0, g: 71, b: 171 });
        map.insert("cocoa brown", Color { r: 210, g: 105, b: 30 });
        map.insert("coffee", Color { r: 111, g: 78, b: 55 });
        map.insert("columbia blue", Color { r: 155, g: 221, b: 255 });
        map.insert("congo pink", Color { r: 248, g: 131, b: 121 });
        map.insert("cool black", Color { r: 0, g: 46, b: 99 });
        map.insert("cool grey", Color { r: 140, g: 146, b: 172 });
        map.insert("copper", Color { r: 184, g: 115, b: 51 });
        map.insert("copper (crayola)", Color { r: 218, g: 138, b: 103 });
        map.insert("copper penny", Color { r: 173, g: 111, b: 105 });
        map.insert("copper red", Color { r: 203, g: 109, b: 81 });
        map.insert("copper rose", Color { r: 153, g: 102, b: 102 });
        map.insert("coquelicot", Color { r: 255, g: 56, b: 0 });
        map.insert("coral", Color { r: 255, g: 127, b: 80 });
        map.insert("coral pink", Color { r: 248, g: 131, b: 121 });
        map.insert("coral red", Color { r: 255, g: 64, b: 64 });
        map.insert("cordovan", Color { r: 137, g: 63, b: 69 });
        map.insert("corn", Color { r: 251, g: 236, b: 93 });
        map.insert("cornell red", Color { r: 179, g: 27, b: 27 });
        map.insert("cornflower blue", Color { r: 100, g: 149, b: 237 });
        map.insert("cornsilk", Color { r: 255, g: 248, b: 220 });
        map.insert("cosmic latte", Color { r: 255, g: 248, b: 231 });
        map.insert("cotton candy", Color { r: 255, g: 188, b: 217 });
        map.insert("cream", Color { r: 255, g: 253, b: 208 });
        map.insert("crimson", Color { r: 220, g: 20, b: 60 });
        map.insert("crimson glory", Color { r: 190, g: 0, b: 50 });
        map.insert("cyan", Color { r: 0, g: 255, b: 255 });
        map.insert("cyan (process)", Color { r: 0, g: 183, b: 235 });
        map.insert("daffodil", Color { r: 255, g: 255, b: 49 });
        map.insert("dandelion", Color { r: 240, g: 225, b: 48 });
        map.insert("dark blue", Color { r: 0, g: 0, b: 139 });
        map.insert("dark brown", Color { r: 101, g: 67, b: 33 });
        map.insert("dark byzantium", Color { r: 93, g: 57, b: 84 });
        map.insert("dark candy apple red", Color { r: 164, g: 0, b: 0 });
        map.insert("dark cerulean", Color { r: 8, g: 69, b: 126 });
        map.insert("dark chestnut", Color { r: 152, g: 105, b: 96 });
        map.insert("dark coral", Color { r: 205, g: 91, b: 69 });
        map.insert("dark cyan", Color { r: 0, g: 139, b: 139 });
        map.insert("dark electric blue", Color { r: 83, g: 104, b: 120 });
        map.insert("dark goldenrod", Color { r: 184, g: 134, b: 11 });
        map.insert("dark gray", Color { r: 169, g: 169, b: 169 });
        map.insert("dark green", Color { r: 1, g: 50, b: 32 });
        map.insert("dark imperial blue", Color { r: 0, g: 65, b: 106 });
        map.insert("dark jungle green", Color { r: 26, g: 36, b: 33 });
        map.insert("dark khaki", Color { r: 189, g: 183, b: 107 });
        map.insert("dark lava", Color { r: 72, g: 60, b: 50 });
        map.insert("dark lavender", Color { r: 115, g: 79, b: 150 });
        map.insert("dark magenta", Color { r: 139, g: 0, b: 139 });
        map.insert("dark midnight blue", Color { r: 0, g: 51, b: 102 });
        map.insert("dark olive green", Color { r: 85, g: 107, b: 47 });
        map.insert("dark orange", Color { r: 255, g: 140, b: 0 });
        map.insert("dark orchid", Color { r: 153, g: 50, b: 204 });
        map.insert("dark pastel blue", Color { r: 119, g: 158, b: 203 });
        map.insert("dark pastel green", Color { r: 3, g: 192, b: 60 });
        map.insert("dark pastel purple", Color { r: 150, g: 111, b: 214 });
        map.insert("dark pastel red", Color { r: 194, g: 59, b: 34 });
        map.insert("dark pink", Color { r: 231, g: 84, b: 128 });
        map.insert("dark powder blue", Color { r: 0, g: 51, b: 153 });
        map.insert("dark raspberry", Color { r: 135, g: 38, b: 87 });
        map.insert("dark red", Color { r: 139, g: 0, b: 0 });
        map.insert("dark salmon", Color { r: 233, g: 150, b: 122 });
        map.insert("dark scarlet", Color { r: 86, g: 3, b: 25 });
        map.insert("dark sea green", Color { r: 143, g: 188, b: 143 });
        map.insert("dark sienna", Color { r: 60, g: 20, b: 20 });
        map.insert("dark slate blue", Color { r: 72, g: 61, b: 139 });
        map.insert("dark slate gray", Color { r: 47, g: 79, b: 79 });
        map.insert("dark spring green", Color { r: 23, g: 114, b: 69 });
        map.insert("dark tan", Color { r: 145, g: 129, b: 81 });
        map.insert("dark tangerine", Color { r: 255, g: 168, b: 18 });
        map.insert("dark taupe", Color { r: 72, g: 60, b: 50 });
        map.insert("dark terra cotta", Color { r: 204, g: 78, b: 92 });
        map.insert("dark turquoise", Color { r: 0, g: 206, b: 209 });
        map.insert("dark violet", Color { r: 148, g: 0, b: 211 });
        map.insert("dark yellow", Color { r: 155, g: 135, b: 12 });
        map.insert("dartmouth green", Color { r: 0, g: 112, b: 60 });
        map.insert("davy's grey", Color { r: 85, g: 85, b: 85 });
        map.insert("debian red", Color { r: 215, g: 10, b: 83 });
        map.insert("deep carmine", Color { r: 169, g: 32, b: 62 });
        map.insert("deep carmine pink", Color { r: 239, g: 48, b: 56 });
        map.insert("deep carrot orange", Color { r: 233, g: 105, b: 44 });
        map.insert("deep cerise", Color { r: 218, g: 50, b: 135 });
        map.insert("deep champagne", Color { r: 250, g: 214, b: 165 });
        map.insert("deep chestnut", Color { r: 185, g: 78, b: 72 });
        map.insert("deep coffee", Color { r: 112, g: 66, b: 65 });
        map.insert("deep fuchsia", Color { r: 193, g: 84, b: 193 });
        map.insert("deep jungle green", Color { r: 0, g: 75, b: 73 });
        map.insert("deep lilac", Color { r: 153, g: 85, b: 187 });
        map.insert("deep magenta", Color { r: 204, g: 0, b: 204 });
        map.insert("deep peach", Color { r: 255, g: 203, b: 164 });
        map.insert("deep pink", Color { r: 255, g: 20, b: 147 });
        map.insert("deep ruby", Color { r: 132, g: 63, b: 91 });
        map.insert("deep saffron", Color { r: 255, g: 153, b: 51 });
        map.insert("deep sky blue", Color { r: 0, g: 191, b: 255 });
        map.insert("deep tuscan red", Color { r: 102, g: 66, b: 77 });
        map.insert("denim", Color { r: 21, g: 96, b: 189 });
        map.insert("desert", Color { r: 193, g: 154, b: 107 });
        map.insert("desert sand", Color { r: 237, g: 201, b: 175 });
        map.insert("dim gray", Color { r: 105, g: 105, b: 105 });
        map.insert("dodger blue", Color { r: 30, g: 144, b: 255 });
        map.insert("dogwood rose", Color { r: 215, g: 24, b: 104 });
        map.insert("dollar bill", Color { r: 133, g: 187, b: 101 });
        map.insert("drab", Color { r: 150, g: 113, b: 23 });
        map.insert("duke blue", Color { r: 0, g: 0, b: 156 });
        map.insert("earth yellow", Color { r: 225, g: 169, b: 95 });
        map.insert("ebony", Color { r: 85, g: 93, b: 80 });
        map.insert("ecru", Color { r: 194, g: 178, b: 128 });
        map.insert("eggplant", Color { r: 97, g: 64, b: 81 });
        map.insert("eggshell", Color { r: 240, g: 234, b: 214 });
        map.insert("egyptian blue", Color { r: 16, g: 52, b: 166 });
        map.insert("electric blue", Color { r: 125, g: 249, b: 255 });
        map.insert("electric crimson", Color { r: 255, g: 0, b: 63 });
        map.insert("electric cyan", Color { r: 0, g: 255, b: 255 });
        map.insert("electric green", Color { r: 0, g: 255, b: 0 });
        map.insert("electric indigo", Color { r: 111, g: 0, b: 255 });
        map.insert("electric lavender", Color { r: 244, g: 187, b: 255 });
        map.insert("electric lime", Color { r: 204, g: 255, b: 0 });
        map.insert("electric purple", Color { r: 191, g: 0, b: 255 });
        map.insert("electric ultramarine", Color { r: 63, g: 0, b: 255 });
        map.insert("electric violet", Color { r: 143, g: 0, b: 255 });
        map.insert("electric yellow", Color { r: 255, g: 255, b: 0 });
        map.insert("emerald", Color { r: 80, g: 200, b: 120 });
        map.insert("english lavender", Color { r: 180, g: 131, b: 149 });
        map.insert("eton blue", Color { r: 150, g: 200, b: 162 });
        map.insert("fallow", Color { r: 193, g: 154, b: 107 });
        map.insert("falu red", Color { r: 128, g: 24, b: 24 });
        map.insert("fandango", Color { r: 181, g: 51, b: 137 });
        map.insert("fashion fuchsia", Color { r: 244, g: 0, b: 161 });
        map.insert("fawn", Color { r: 229, g: 170, b: 112 });
        map.insert("feldgrau", Color { r: 77, g: 93, b: 83 });
        map.insert("fern green", Color { r: 79, g: 121, b: 66 });
        map.insert("ferrari red", Color { r: 255, g: 40, b: 0 });
        map.insert("field drab", Color { r: 108, g: 84, b: 30 });
        map.insert("fire engine red", Color { r: 206, g: 32, b: 41 });
        map.insert("firebrick", Color { r: 178, g: 34, b: 34 });
        map.insert("flame", Color { r: 226, g: 88, b: 34 });
        map.insert("flamingo pink", Color { r: 252, g: 142, b: 172 });
        map.insert("flavescent", Color { r: 247, g: 233, b: 142 });
        map.insert("flax", Color { r: 238, g: 220, b: 130 });
        map.insert("floral white", Color { r: 255, g: 250, b: 240 });
        map.insert("fluorescent orange", Color { r: 255, g: 191, b: 0 });
        map.insert("fluorescent pink", Color { r: 255, g: 20, b: 147 });
        map.insert("fluorescent yellow", Color { r: 204, g: 255, b: 0 });
        map.insert("folly", Color { r: 255, g: 0, b: 79 });
        map.insert("forest green (traditional)", Color { r: 1, g: 68, b: 33 });
        map.insert("forest green (web)", Color { r: 34, g: 139, b: 34 });
        map.insert("french beige", Color { r: 166, g: 123, b: 91 });
        map.insert("french blue", Color { r: 0, g: 114, b: 187 });
        map.insert("french lilac", Color { r: 134, g: 96, b: 142 });
        map.insert("french lime", Color { r: 204, g: 255, b: 0 });
        map.insert("french raspberry", Color { r: 199, g: 44, b: 72 });
        map.insert("french rose", Color { r: 246, g: 74, b: 138 });
        map.insert("fuchsia", Color { r: 255, g: 0, b: 255 });
        map.insert("fuchsia (crayola)", Color { r: 193, g: 84, b: 193 });
        map.insert("fuchsia pink", Color { r: 255, g: 119, b: 255 });
        map.insert("fuchsia rose", Color { r: 199, g: 67, b: 117 });
        map.insert("fulvous", Color { r: 228, g: 132, b: 0 });
        map.insert("fuzzy wuzzy", Color { r: 204, g: 102, b: 102 });
        map.insert("gainsboro", Color { r: 220, g: 220, b: 220 });
        map.insert("gamboge", Color { r: 228, g: 155, b: 15 });
        map.insert("ghost white", Color { r: 248, g: 248, b: 255 });
        map.insert("ginger", Color { r: 176, g: 101, b: 0 });
        map.insert("glaucous", Color { r: 96, g: 130, b: 182 });
        map.insert("glitter", Color { r: 230, g: 232, b: 250 });
        map.insert("gold (metallic)", Color { r: 212, g: 175, b: 55 });
        map.insert("gold (web) (golden)", Color { r: 255, g: 215, b: 0 });
        map.insert("golden brown", Color { r: 153, g: 101, b: 21 });
        map.insert("golden poppy", Color { r: 252, g: 194, b: 0 });
        map.insert("golden yellow", Color { r: 255, g: 223, b: 0 });
        map.insert("goldenrod", Color { r: 218, g: 165, b: 32 });
        map.insert("granny smith apple", Color { r: 168, g: 228, b: 160 });
        map.insert("gray", Color { r: 128, g: 128, b: 128 });
        map.insert("gray-asparagus", Color { r: 70, g: 89, b: 69 });
        map.insert("gray (html/css gray)", Color { r: 128, g: 128, b: 128 });
        map.insert("gray (x11 gray)", Color { r: 190, g: 190, b: 190 });
        map.insert("green (color wheel) (x11 green)", Color { r: 0, g: 255, b: 0 });
        map.insert("green (crayola)", Color { r: 28, g: 172, b: 120 });
        map.insert("green (html/css green)", Color { r: 0, g: 128, b: 0 });
        map.insert("green (munsell)", Color { r: 0, g: 168, b: 119 });
        map.insert("green (ncs)", Color { r: 0, g: 159, b: 107 });
        map.insert("green (pigment)", Color { r: 0, g: 165, b: 80 });
        map.insert("green (ryb)", Color { r: 102, g: 176, b: 50 });
        map.insert("green-yellow", Color { r: 173, g: 255, b: 47 });
        map.insert("grullo", Color { r: 169, g: 154, b: 134 });
        map.insert("guppie green", Color { r: 0, g: 255, b: 127 });
        map.insert("halaya be", Color { r: 102, g: 56, b: 84 });
        map.insert("han blue", Color { r: 68, g: 108, b: 207 });
        map.insert("han purple", Color { r: 82, g: 24, b: 250 });
        map.insert("hansa yellow", Color { r: 233, g: 214, b: 107 });
        map.insert("harlequin", Color { r: 63, g: 255, b: 0 });
        map.insert("harvard crimson", Color { r: 201, g: 0, b: 22 });
        map.insert("harvest gold", Color { r: 218, g: 145, b: 0 });
        map.insert("heart gold", Color { r: 128, g: 128, b: 0 });
        map.insert("heliotrope", Color { r: 223, g: 115, b: 255 });
        map.insert("hollywood cerise", Color { r: 244, g: 0, b: 161 });
        map.insert("honeydew", Color { r: 240, g: 255, b: 240 });
        map.insert("honolulu blue", Color { r: 0, g: 127, b: 191 });
        map.insert("hooker's green", Color { r: 73, g: 121, b: 107 });
        map.insert("hot magenta", Color { r: 255, g: 29, b: 206 });
        map.insert("hot pink", Color { r: 255, g: 105, b: 180 });
        map.insert("hunter green", Color { r: 53, g: 94, b: 59 });
        map.insert("iceberg", Color { r: 113, g: 166, b: 210 });
        map.insert("icterine", Color { r: 252, g: 247, b: 94 });
        map.insert("imperial blue", Color { r: 0, g: 35, b: 149 });
        map.insert("inchworm", Color { r: 178, g: 236, b: 93 });
        map.insert("india green", Color { r: 19, g: 136, b: 8 });
        map.insert("indian red", Color { r: 205, g: 92, b: 92 });
        map.insert("indian yellow", Color { r: 227, g: 168, b: 87 });
        map.insert("indigo", Color { r: 111, g: 0, b: 255 });
        map.insert("indigo (dye)", Color { r: 0, g: 65, b: 106 });
        map.insert("indigo (web)", Color { r: 75, g: 0, b: 130 });
        map.insert("international klein blue", Color { r: 0, g: 47, b: 167 });
        map.insert("international orange (aerospace)", Color { r: 255, g: 79, b: 0 });
        map.insert("international orange (engineering)", Color { r: 186, g: 22, b: 12 });
        map.insert("international orange (golden gate bridge)", Color { r: 192, g: 54, b: 44 });
        map.insert("iris", Color { r: 90, g: 79, b: 207 });
        map.insert("isabelline", Color { r: 244, g: 240, b: 236 });
        map.insert("islamic green", Color { r: 0, g: 144, b: 0 });
        map.insert("ivory", Color { r: 255, g: 255, b: 240 });
        map.insert("jade", Color { r: 0, g: 168, b: 107 });
        map.insert("jasmine", Color { r: 248, g: 222, b: 126 });
        map.insert("jasper", Color { r: 215, g: 59, b: 62 });
        map.insert("jazzberry jam", Color { r: 165, g: 11, b: 94 });
        map.insert("jet", Color { r: 52, g: 52, b: 52 });
        map.insert("jonquil", Color { r: 250, g: 218, b: 94 });
        map.insert("june bud", Color { r: 189, g: 218, b: 87 });
        map.insert("jungle green", Color { r: 41, g: 171, b: 135 });
        map.insert("kelly green", Color { r: 76, g: 187, b: 23 });
        map.insert("kenyan copper", Color { r: 124, g: 28, b: 5 });
        map.insert("khaki (html/css) (khaki)", Color { r: 195, g: 176, b: 145 });
        map.insert("khaki (x11) (light khaki)", Color { r: 240, g: 230, b: 140 });
        map.insert("ku crimson", Color { r: 232, g: 0, b: 13 });
        map.insert("la salle green", Color { r: 8, g: 120, b: 48 });
        map.insert("languid lavender", Color { r: 214, g: 202, b: 221 });
        map.insert("lapis lazuli", Color { r: 38, g: 97, b: 156 });
        map.insert("laser lemon", Color { r: 254, g: 254, b: 34 });
        map.insert("laurel green", Color { r: 169, g: 186, b: 157 });
        map.insert("lava", Color { r: 207, g: 16, b: 32 });
        map.insert("lavender blue", Color { r: 204, g: 204, b: 255 });
        map.insert("lavender blush", Color { r: 255, g: 240, b: 245 });
        map.insert("lavender (floral)", Color { r: 181, g: 126, b: 220 });
        map.insert("lavender gray", Color { r: 196, g: 195, b: 208 });
        map.insert("lavender indigo", Color { r: 148, g: 87, b: 235 });
        map.insert("lavender magenta", Color { r: 238, g: 130, b: 238 });
        map.insert("lavender mist", Color { r: 230, g: 230, b: 250 });
        map.insert("lavender pink", Color { r: 251, g: 174, b: 210 });
        map.insert("lavender purple", Color { r: 150, g: 123, b: 182 });
        map.insert("lavender rose", Color { r: 251, g: 160, b: 227 });
        map.insert("lavender (web)", Color { r: 230, g: 230, b: 250 });
        map.insert("lawn green", Color { r: 124, g: 252, b: 0 });
        map.insert("lemon", Color { r: 255, g: 247, b: 0 });
        map.insert("lemon chiffon", Color { r: 255, g: 250, b: 205 });
        map.insert("lemon lime", Color { r: 227, g: 255, b: 0 });
        map.insert("licorice", Color { r: 26, g: 17, b: 16 });
        map.insert("light apricot", Color { r: 253, g: 213, b: 177 });
        map.insert("light blue", Color { r: 173, g: 216, b: 230 });
        map.insert("light brown", Color { r: 181, g: 101, b: 29 });
        map.insert("light carmine pink", Color { r: 230, g: 103, b: 113 });
        map.insert("light coral", Color { r: 240, g: 128, b: 128 });
        map.insert("light cornflower blue", Color { r: 147, g: 204, b: 234 });
        map.insert("light crimson", Color { r: 245, g: 105, b: 145 });
        map.insert("light cyan", Color { r: 224, g: 255, b: 255 });
        map.insert("light fuchsia pink", Color { r: 249, g: 132, b: 239 });
        map.insert("light goldenrod yellow", Color { r: 250, g: 250, b: 210 });
        map.insert("light gray", Color { r: 211, g: 211, b: 211 });
        map.insert("light green", Color { r: 144, g: 238, b: 144 });
        map.insert("light khaki", Color { r: 240, g: 230, b: 140 });
        map.insert("light pastel purple", Color { r: 177, g: 156, b: 217 });
        map.insert("light pink", Color { r: 255, g: 182, b: 193 });
        map.insert("light red ochre", Color { r: 233, g: 116, b: 81 });
        map.insert("light salmon", Color { r: 255, g: 160, b: 122 });
        map.insert("light salmon pink", Color { r: 255, g: 153, b: 153 });
        map.insert("light sea green", Color { r: 32, g: 178, b: 170 });
        map.insert("light sky blue", Color { r: 135, g: 206, b: 250 });
        map.insert("light slate gray", Color { r: 119, g: 136, b: 153 });
        map.insert("light taupe", Color { r: 179, g: 139, b: 109 });
        map.insert("light thulian pink", Color { r: 230, g: 143, b: 172 });
        map.insert("light yellow", Color { r: 255, g: 255, b: 224 });
        map.insert("lilac", Color { r: 200, g: 162, b: 200 });
        map.insert("lime (color wheel)", Color { r: 191, g: 255, b: 0 });
        map.insert("lime green", Color { r: 50, g: 205, b: 50 });
        map.insert("lime (web) (x11 green)", Color { r: 0, g: 255, b: 0 });
        map.insert("limerick", Color { r: 157, g: 194, b: 9 });
        map.insert("lincoln green", Color { r: 25, g: 89, b: 5 });
        map.insert("linen", Color { r: 250, g: 240, b: 230 });
        map.insert("lion", Color { r: 193, g: 154, b: 107 });
        map.insert("little boy blue", Color { r: 108, g: 160, b: 220 });
        map.insert("liver", Color { r: 83, g: 75, b: 79 });
        map.insert("lust", Color { r: 230, g: 32, b: 32 });
        map.insert("magenta", Color { r: 255, g: 0, b: 255 });
        map.insert("magenta (dye)", Color { r: 202, g: 31, b: 123 });
        map.insert("magenta (process)", Color { r: 255, g: 0, b: 144 });
        map.insert("magic mint", Color { r: 170, g: 240, b: 209 });
        map.insert("magnolia", Color { r: 248, g: 244, b: 255 });
        map.insert("mahogany", Color { r: 192, g: 64, b: 0 });
        map.insert("maize", Color { r: 251, g: 236, b: 93 });
        map.insert("majorelle blue", Color { r: 96, g: 80, b: 220 });
        map.insert("malachite", Color { r: 11, g: 218, b: 81 });
        map.insert("manatee", Color { r: 151, g: 154, b: 170 });
        map.insert("mango tango", Color { r: 255, g: 130, b: 67 });
        map.insert("mantis", Color { r: 116, g: 195, b: 101 });
        map.insert("mardi gras", Color { r: 136, g: 0, b: 133 });
        map.insert("maroon (crayola)", Color { r: 195, g: 33, b: 72 });
        map.insert("maroon (html/css)", Color { r: 128, g: 0, b: 0 });
        map.insert("maroon (x11)", Color { r: 176, g: 48, b: 96 });
        map.insert("mauve", Color { r: 224, g: 176, b: 255 });
        map.insert("mauve taupe", Color { r: 145, g: 95, b: 109 });
        map.insert("mauvelous", Color { r: 239, g: 152, b: 170 });
        map.insert("maya blue", Color { r: 115, g: 194, b: 251 });
        map.insert("meat brown", Color { r: 229, g: 183, b: 59 });
        map.insert("medium aquamarine", Color { r: 102, g: 221, b: 170 });
        map.insert("medium blue", Color { r: 0, g: 0, b: 205 });
        map.insert("medium candy apple red", Color { r: 226, g: 6, b: 44 });
        map.insert("medium carmine", Color { r: 175, g: 64, b: 53 });
        map.insert("medium champagne", Color { r: 243, g: 229, b: 171 });
        map.insert("medium electric blue", Color { r: 3, g: 80, b: 150 });
        map.insert("medium jungle green", Color { r: 28, g: 53, b: 45 });
        map.insert("medium lavender magenta", Color { r: 221, g: 160, b: 221 });
        map.insert("medium orchid", Color { r: 186, g: 85, b: 211 });
        map.insert("medium persian blue", Color { r: 0, g: 103, b: 165 });
        map.insert("medium purple", Color { r: 147, g: 112, b: 219 });
        map.insert("medium red-violet", Color { r: 187, g: 51, b: 133 });
        map.insert("medium ruby", Color { r: 170, g: 64, b: 105 });
        map.insert("medium sea green", Color { r: 60, g: 179, b: 113 });
        map.insert("medium slate blue", Color { r: 123, g: 104, b: 238 });
        map.insert("medium spring bud", Color { r: 201, g: 220, b: 135 });
        map.insert("medium spring green", Color { r: 0, g: 250, b: 154 });
        map.insert("medium taupe", Color { r: 103, g: 76, b: 71 });
        map.insert("medium turquoise", Color { r: 72, g: 209, b: 204 });
        map.insert("medium tuscan red", Color { r: 121, g: 68, b: 59 });
        map.insert("medium vermilion", Color { r: 217, g: 96, b: 59 });
        map.insert("medium violet-red", Color { r: 199, g: 21, b: 133 });
        map.insert("mellow apricot", Color { r: 248, g: 184, b: 120 });
        map.insert("mellow yellow", Color { r: 248, g: 222, b: 126 });
        map.insert("melon", Color { r: 253, g: 188, b: 180 });
        map.insert("midnight blue", Color { r: 25, g: 25, b: 112 });
        map.insert("midnight green (eagle green)", Color { r: 0, g: 73, b: 83 });
        map.insert("mikado yellow", Color { r: 255, g: 196, b: 12 });
        map.insert("mint", Color { r: 62, g: 180, b: 137 });
        map.insert("mint cream", Color { r: 245, g: 255, b: 250 });
        map.insert("mint green", Color { r: 152, g: 255, b: 152 });
        map.insert("misty rose", Color { r: 255, g: 228, b: 225 });
        map.insert("moccasin", Color { r: 250, g: 235, b: 215 });
        map.insert("mode beige", Color { r: 150, g: 113, b: 23 });
        map.insert("moonstone blue", Color { r: 115, g: 169, b: 194 });
        map.insert("mordant red 19", Color { r: 174, g: 12, b: 0 });
        map.insert("moss green", Color { r: 173, g: 223, b: 173 });
        map.insert("mountain meadow", Color { r: 48, g: 186, b: 143 });
        map.insert("mountbatten pink", Color { r: 153, g: 122, b: 141 });
        map.insert("msu green", Color { r: 24, g: 69, b: 59 });
        map.insert("mulberry", Color { r: 197, g: 75, b: 140 });
        map.insert("mustard", Color { r: 255, g: 219, b: 88 });
        map.insert("myrtle", Color { r: 33, g: 66, b: 30 });
        map.insert("nadeshiko pink", Color { r: 246, g: 173, b: 198 });
        map.insert("napier green", Color { r: 42, g: 128, b: 0 });
        map.insert("naples yellow", Color { r: 250, g: 218, b: 94 });
        map.insert("navajo white", Color { r: 255, g: 222, b: 173 });
        map.insert("navy blue", Color { r: 0, g: 0, b: 128 });
        map.insert("neon carrot", Color { r: 255, g: 163, b: 67 });
        map.insert("neon fuchsia", Color { r: 254, g: 65, b: 100 });
        map.insert("neon green", Color { r: 57, g: 255, b: 20 });
        map.insert("new york pink", Color { r: 215, g: 131, b: 127 });
        map.insert("non-photo blue", Color { r: 164, g: 221, b: 237 });
        map.insert("north texas green", Color { r: 5, g: 144, b: 51 });
        map.insert("ocean boat blue", Color { r: 0, g: 119, b: 190 });
        map.insert("ochre", Color { r: 204, g: 119, b: 34 });
        map.insert("office green", Color { r: 0, g: 128, b: 0 });
        map.insert("old gold", Color { r: 207, g: 181, b: 59 });
        map.insert("old lace", Color { r: 253, g: 245, b: 230 });
        map.insert("old lavender", Color { r: 121, g: 104, b: 120 });
        map.insert("old mauve", Color { r: 103, g: 49, b: 71 });
        map.insert("old rose", Color { r: 192, g: 128, b: 129 });
        map.insert("olive", Color { r: 128, g: 128, b: 0 });
        map.insert("olive drab #7", Color { r: 60, g: 52, b: 31 });
        map.insert("olive drab (web) (olive drab #3)", Color { r: 107, g: 142, b: 35 });
        map.insert("olivine", Color { r: 154, g: 185, b: 115 });
        map.insert("onyx", Color { r: 53, g: 56, b: 57 });
        map.insert("opera mauve", Color { r: 183, g: 132, b: 167 });
        map.insert("orange (color wheel)", Color { r: 255, g: 127, b: 0 });
        map.insert("orange peel", Color { r: 255, g: 159, b: 0 });
        map.insert("orange-red", Color { r: 255, g: 69, b: 0 });
        map.insert("orange (ryb)", Color { r: 251, g: 153, b: 2 });
        map.insert("orange (web color)", Color { r: 255, g: 165, b: 0 });
        map.insert("orchid", Color { r: 218, g: 112, b: 214 });
        map.insert("otter brown", Color { r: 101, g: 67, b: 33 });
        map.insert("ou crimson red", Color { r: 153, g: 0, b: 0 });
        map.insert("outer space", Color { r: 65, g: 74, b: 76 });
        map.insert("outrageous orange", Color { r: 255, g: 110, b: 74 });
        map.insert("oxford blue", Color { r: 0, g: 33, b: 71 });
        map.insert("pakistan green", Color { r: 0, g: 102, b: 0 });
        map.insert("palatinate blue", Color { r: 39, g: 59, b: 226 });
        map.insert("palatinate purple", Color { r: 104, g: 40, b: 96 });
        map.insert("pale aqua", Color { r: 188, g: 212, b: 230 });
        map.insert("pale blue", Color { r: 175, g: 238, b: 238 });
        map.insert("pale brown", Color { r: 152, g: 118, b: 84 });
        map.insert("pale carmine", Color { r: 175, g: 64, b: 53 });
        map.insert("pale cerulean", Color { r: 155, g: 196, b: 226 });
        map.insert("pale chestnut", Color { r: 221, g: 173, b: 175 });
        map.insert("pale copper", Color { r: 218, g: 138, b: 103 });
        map.insert("pale cornflower blue", Color { r: 171, g: 205, b: 239 });
        map.insert("pale gold", Color { r: 230, g: 190, b: 138 });
        map.insert("pale goldenrod", Color { r: 238, g: 232, b: 170 });
        map.insert("pale green", Color { r: 152, g: 251, b: 152 });
        map.insert("pale lavender", Color { r: 220, g: 208, b: 255 });
        map.insert("pale magenta", Color { r: 249, g: 132, b: 229 });
        map.insert("pale pink", Color { r: 250, g: 218, b: 221 });
        map.insert("pale plum", Color { r: 221, g: 160, b: 221 });
        map.insert("pale red-violet", Color { r: 219, g: 112, b: 147 });
        map.insert("pale robin egg blue", Color { r: 150, g: 222, b: 209 });
        map.insert("pale silver", Color { r: 201, g: 192, b: 187 });
        map.insert("pale spring bud", Color { r: 236, g: 235, b: 189 });
        map.insert("pale taupe", Color { r: 188, g: 152, b: 126 });
        map.insert("pale violet-red", Color { r: 219, g: 112, b: 147 });
        map.insert("pansy purple", Color { r: 120, g: 24, b: 74 });
        map.insert("papaya whip", Color { r: 255, g: 239, b: 213 });
        map.insert("paris green", Color { r: 80, g: 200, b: 120 });
        map.insert("pastel blue", Color { r: 174, g: 198, b: 207 });
        map.insert("pastel brown", Color { r: 131, g: 105, b: 83 });
        map.insert("pastel gray", Color { r: 207, g: 207, b: 196 });
        map.insert("pastel green", Color { r: 119, g: 221, b: 119 });
        map.insert("pastel magenta", Color { r: 244, g: 154, b: 194 });
        map.insert("pastel orange", Color { r: 255, g: 179, b: 71 });
        map.insert("pastel pink", Color { r: 222, g: 165, b: 164 });
        map.insert("pastel purple", Color { r: 179, g: 158, b: 181 });
        map.insert("pastel red", Color { r: 255, g: 105, b: 97 });
        map.insert("pastel violet", Color { r: 203, g: 153, b: 201 });
        map.insert("pastel yellow", Color { r: 253, g: 253, b: 150 });
        map.insert("patriarch", Color { r: 128, g: 0, b: 128 });
        map.insert("payne's grey", Color { r: 83, g: 104, b: 120 });
        map.insert("peach", Color { r: 255, g: 229, b: 180 });
        map.insert("peach (crayola)", Color { r: 255, g: 203, b: 164 });
        map.insert("peach-orange", Color { r: 255, g: 204, b: 153 });
        map.insert("peach puff", Color { r: 255, g: 218, b: 185 });
        map.insert("peach-yellow", Color { r: 250, g: 223, b: 173 });
        map.insert("pear", Color { r: 209, g: 226, b: 49 });
        map.insert("pearl", Color { r: 234, g: 224, b: 200 });
        map.insert("pearl aqua", Color { r: 136, g: 216, b: 192 });
        map.insert("pearly purple", Color { r: 183, g: 104, b: 162 });
        map.insert("peridot", Color { r: 230, g: 226, b: 0 });
        map.insert("periwinkle", Color { r: 204, g: 204, b: 255 });
        map.insert("persian blue", Color { r: 28, g: 57, b: 187 });
        map.insert("persian green", Color { r: 0, g: 166, b: 147 });
        map.insert("persian indigo", Color { r: 50, g: 18, b: 122 });
        map.insert("persian orange", Color { r: 217, g: 144, b: 88 });
        map.insert("persian pink", Color { r: 247, g: 127, b: 190 });
        map.insert("persian plum", Color { r: 112, g: 28, b: 28 });
        map.insert("persian red", Color { r: 204, g: 51, b: 51 });
        map.insert("persian rose", Color { r: 254, g: 40, b: 162 });
        map.insert("persimmon", Color { r: 236, g: 88, b: 0 });
        map.insert("peru", Color { r: 205, g: 133, b: 63 });
        map.insert("phlox", Color { r: 223, g: 0, b: 255 });
        map.insert("phthalo blue", Color { r: 0, g: 15, b: 137 });
        map.insert("phthalo green", Color { r: 18, g: 53, b: 36 });
        map.insert("piggy pink", Color { r: 253, g: 221, b: 230 });
        map.insert("pine green", Color { r: 1, g: 121, b: 111 });
        map.insert("pink", Color { r: 255, g: 192, b: 203 });
        map.insert("pink lace", Color { r: 255, g: 221, b: 244 });
        map.insert("pink-orange", Color { r: 255, g: 153, b: 102 });
        map.insert("pink pearl", Color { r: 231, g: 172, b: 207 });
        map.insert("pink sherbet", Color { r: 247, g: 143, b: 167 });
        map.insert("pistachio", Color { r: 147, g: 197, b: 114 });
        map.insert("platinum", Color { r: 229, g: 228, b: 226 });
        map.insert("plum (traditional)", Color { r: 142, g: 69, b: 133 });
        map.insert("plum (web)", Color { r: 221, g: 160, b: 221 });
        map.insert("portland orange", Color { r: 255, g: 90, b: 54 });
        map.insert("powder blue (web)", Color { r: 176, g: 224, b: 230 });
        map.insert("princeton orange", Color { r: 255, g: 143, b: 0 });
        map.insert("prune", Color { r: 112, g: 28, b: 28 });
        map.insert("prussian blue", Color { r: 0, g: 49, b: 83 });
        map.insert("psychedelic purple", Color { r: 223, g: 0, b: 255 });
        map.insert("puce", Color { r: 204, g: 136, b: 153 });
        map.insert("pumpkin", Color { r: 255, g: 117, b: 24 });
        map.insert("purple heart", Color { r: 105, g: 53, b: 156 });
        map.insert("purple (html/css)", Color { r: 128, g: 0, b: 128 });
        map.insert("purple mountain majesty", Color { r: 150, g: 120, b: 182 });
        map.insert("purple (munsell)", Color { r: 159, g: 0, b: 197 });
        map.insert("purple pizzazz", Color { r: 254, g: 78, b: 218 });
        map.insert("purple taupe", Color { r: 80, g: 64, b: 77 });
        map.insert("purple (x11)", Color { r: 160, g: 32, b: 240 });
        map.insert("quartz", Color { r: 81, g: 72, b: 79 });
        map.insert("rackley", Color { r: 93, g: 138, b: 168 });
        map.insert("radical red", Color { r: 255, g: 53, b: 94 });
        map.insert("rajah", Color { r: 251, g: 171, b: 96 });
        map.insert("raspberry", Color { r: 227, g: 11, b: 93 });
        map.insert("raspberry glace", Color { r: 145, g: 95, b: 109 });
        map.insert("raspberry pink", Color { r: 226, g: 80, b: 152 });
        map.insert("raspberry rose", Color { r: 179, g: 68, b: 108 });
        map.insert("raw umber", Color { r: 130, g: 102, b: 68 });
        map.insert("razzle dazzle rose", Color { r: 255, g: 51, b: 204 });
        map.insert("razzmatazz", Color { r: 227, g: 37, b: 107 });
        map.insert("red", Color { r: 255, g: 0, b: 0 });
        map.insert("red-brown", Color { r: 165, g: 42, b: 42 });
        map.insert("red devil", Color { r: 134, g: 1, b: 17 });
        map.insert("red (munsell)", Color { r: 242, g: 0, b: 60 });
        map.insert("red (ncs)", Color { r: 196, g: 2, b: 51 });
        map.insert("red-orange", Color { r: 255, g: 83, b: 73 });
        map.insert("red (pigment)", Color { r: 237, g: 28, b: 36 });
        map.insert("red (ryb)", Color { r: 254, g: 39, b: 18 });
        map.insert("red-violet", Color { r: 199, g: 21, b: 133 });
        map.insert("redwood", Color { r: 171, g: 78, b: 82 });
        map.insert("regalia", Color { r: 82, g: 45, b: 128 });
        map.insert("resolution blue", Color { r: 0, g: 35, b: 135 });
        map.insert("rich black", Color { r: 0, g: 64, b: 64 });
        map.insert("rich brilliant lavender", Color { r: 241, g: 167, b: 254 });
        map.insert("rich carmine", Color { r: 215, g: 0, b: 64 });
        map.insert("rich electric blue", Color { r: 8, g: 146, b: 208 });
        map.insert("rich lavender", Color { r: 167, g: 107, b: 207 });
        map.insert("rich lilac", Color { r: 182, g: 102, b: 210 });
        map.insert("rich maroon", Color { r: 176, g: 48, b: 96 });
        map.insert("rifle green", Color { r: 65, g: 72, b: 51 });
        map.insert("robin egg blue", Color { r: 0, g: 204, b: 204 });
        map.insert("rose", Color { r: 255, g: 0, b: 127 });
        map.insert("rose bonbon", Color { r: 249, g: 66, b: 158 });
        map.insert("rose ebony", Color { r: 103, g: 72, b: 70 });
        map.insert("rose gold", Color { r: 183, g: 110, b: 121 });
        map.insert("rose madder", Color { r: 227, g: 38, b: 54 });
        map.insert("rose pink", Color { r: 255, g: 102, b: 204 });
        map.insert("rose quartz", Color { r: 170, g: 152, b: 169 });
        map.insert("rose taupe", Color { r: 144, g: 93, b: 93 });
        map.insert("rose vale", Color { r: 171, g: 78, b: 82 });
        map.insert("rosewood", Color { r: 101, g: 0, b: 11 });
        map.insert("rosso corsa", Color { r: 212, g: 0, b: 0 });
        map.insert("rosy brown", Color { r: 188, g: 143, b: 143 });
        map.insert("royal azure", Color { r: 0, g: 56, b: 168 });
        map.insert("royal blue (traditional)", Color { r: 0, g: 35, b: 102 });
        map.insert("royal blue (web)", Color { r: 65, g: 105, b: 225 });
        map.insert("royal fuchsia", Color { r: 202, g: 44, b: 146 });
        map.insert("royal purple", Color { r: 120, g: 81, b: 169 });
        map.insert("royal yellow", Color { r: 250, g: 218, b: 94 });
        map.insert("rubine red", Color { r: 209, g: 0, b: 86 });
        map.insert("ruby", Color { r: 224, g: 17, b: 95 });
        map.insert("ruby red", Color { r: 155, g: 17, b: 30 });
        map.insert("ruddy", Color { r: 255, g: 0, b: 40 });
        map.insert("ruddy brown", Color { r: 187, g: 101, b: 40 });
        map.insert("ruddy pink", Color { r: 225, g: 142, b: 150 });
        map.insert("rufous", Color { r: 168, g: 28, b: 7 });
        map.insert("russet", Color { r: 128, g: 70, b: 27 });
        map.insert("rust", Color { r: 183, g: 65, b: 14 });
        map.insert("rusty red", Color { r: 218, g: 44, b: 67 });
        map.insert("sacramento state green", Color { r: 0, g: 86, b: 63 });
        map.insert("saddle brown", Color { r: 139, g: 69, b: 19 });
        map.insert("safety orange (blaze orange)", Color { r: 255, g: 103, b: 0 });
        map.insert("saffron", Color { r: 244, g: 196, b: 48 });
        map.insert("salmon", Color { r: 255, g: 140, b: 105 });
        map.insert("salmon pink", Color { r: 255, g: 145, b: 164 });
        map.insert("sand", Color { r: 194, g: 178, b: 128 });
        map.insert("sand dune", Color { r: 150, g: 113, b: 23 });
        map.insert("sandstorm", Color { r: 236, g: 213, b: 64 });
        map.insert("sandy brown", Color { r: 244, g: 164, b: 96 });
        map.insert("sandy taupe", Color { r: 150, g: 113, b: 23 });
        map.insert("sangria", Color { r: 146, g: 0, b: 10 });
        map.insert("sap green", Color { r: 80, g: 125, b: 42 });
        map.insert("sapphire", Color { r: 15, g: 82, b: 186 });
        map.insert("sapphire blue", Color { r: 0, g: 103, b: 165 });
        map.insert("satin sheen gold", Color { r: 203, g: 161, b: 53 });
        map.insert("scarlet", Color { r: 255, g: 36, b: 0 });
        map.insert("scarlet (crayola)", Color { r: 253, g: 14, b: 53 });
        map.insert("school bus yellow", Color { r: 255, g: 216, b: 0 });
        map.insert("screamin' green", Color { r: 118, g: 255, b: 122 });
        map.insert("sea blue", Color { r: 0, g: 105, b: 148 });
        map.insert("sea green", Color { r: 46, g: 139, b: 87 });
        map.insert("seal brown", Color { r: 50, g: 20, b: 20 });
        map.insert("seashell", Color { r: 255, g: 245, b: 238 });
        map.insert("selective yellow", Color { r: 255, g: 186, b: 0 });
        map.insert("sepia", Color { r: 112, g: 66, b: 20 });
        map.insert("shadow", Color { r: 138, g: 121, b: 93 });
        map.insert("shamrock green", Color { r: 0, g: 158, b: 96 });
        map.insert("shocking pink", Color { r: 252, g: 15, b: 192 });
        map.insert("shocking pink (crayola)", Color { r: 255, g: 111, b: 255 });
        map.insert("sienna", Color { r: 136, g: 45, b: 23 });
        map.insert("silver", Color { r: 192, g: 192, b: 192 });
        map.insert("sinopia", Color { r: 203, g: 65, b: 11 });
        map.insert("skobeloff", Color { r: 0, g: 116, b: 116 });
        map.insert("sky blue", Color { r: 135, g: 206, b: 235 });
        map.insert("sky magenta", Color { r: 207, g: 113, b: 175 });
        map.insert("slate blue", Color { r: 106, g: 90, b: 205 });
        map.insert("slate gray", Color { r: 112, g: 128, b: 144 });
        map.insert("smalt (dark powder blue)", Color { r: 0, g: 51, b: 153 });
        map.insert("smokey topaz", Color { r: 147, g: 61, b: 65 });
        map.insert("smoky black", Color { r: 16, g: 12, b: 8 });
        map.insert("snow", Color { r: 255, g: 250, b: 250 });
        map.insert("spiro disco ball", Color { r: 15, g: 192, b: 252 });
        map.insert("spring bud", Color { r: 167, g: 252, b: 0 });
        map.insert("spring green", Color { r: 0, g: 255, b: 127 });
        map.insert("st. patrick's blue", Color { r: 35, g: 41, b: 122 });
        map.insert("steel blue", Color { r: 70, g: 130, b: 180 });
        map.insert("stil de grain yellow", Color { r: 250, g: 218, b: 94 });
        map.insert("stizza", Color { r: 153, g: 0, b: 0 });
        map.insert("stormcloud", Color { r: 79, g: 102, b: 106 });
        map.insert("straw", Color { r: 228, g: 217, b: 111 });
        map.insert("sunglow", Color { r: 255, g: 204, b: 51 });
        map.insert("sunset", Color { r: 250, g: 214, b: 165 });
        map.insert("tan", Color { r: 210, g: 180, b: 140 });
        map.insert("tangelo", Color { r: 249, g: 77, b: 0 });
        map.insert("tangerine", Color { r: 242, g: 133, b: 0 });
        map.insert("tangerine yellow", Color { r: 255, g: 204, b: 0 });
        map.insert("tango pink", Color { r: 228, g: 113, b: 122 });
        map.insert("taupe", Color { r: 72, g: 60, b: 50 });
        map.insert("taupe gray", Color { r: 139, g: 133, b: 137 });
        map.insert("tea green", Color { r: 208, g: 240, b: 192 });
        map.insert("tea rose (orange)", Color { r: 248, g: 131, b: 121 });
        map.insert("tea rose (rose)", Color { r: 244, g: 194, b: 194 });
        map.insert("teal", Color { r: 0, g: 128, b: 128 });
        map.insert("teal blue", Color { r: 54, g: 117, b: 136 });
        map.insert("teal green", Color { r: 0, g: 130, b: 127 });
        map.insert("telemagenta", Color { r: 207, g: 52, b: 118 });
        map.insert("tenne (tawny)", Color { r: 205, g: 87, b: 0 });
        map.insert("terra cotta", Color { r: 226, g: 114, b: 91 });
        map.insert("thistle", Color { r: 216, g: 191, b: 216 });
        map.insert("thulian pink", Color { r: 222, g: 111, b: 161 });
        map.insert("tickle me pink", Color { r: 252, g: 137, b: 172 });
        map.insert("tiffany blue", Color { r: 10, g: 186, b: 181 });
        map.insert("tiger's eye", Color { r: 224, g: 141, b: 60 });
        map.insert("timberwolf", Color { r: 219, g: 215, b: 210 });
        map.insert("titanium yellow", Color { r: 238, g: 230, b: 0 });
        map.insert("tomato", Color { r: 255, g: 99, b: 71 });
        map.insert("toolbox", Color { r: 116, g: 108, b: 192 });
        map.insert("topaz", Color { r: 255, g: 200, b: 124 });
        map.insert("tractor red", Color { r: 253, g: 14, b: 53 });
        map.insert("trolley grey", Color { r: 128, g: 128, b: 128 });
        map.insert("tropical rain forest", Color { r: 0, g: 117, b: 94 });
        map.insert("true blue", Color { r: 0, g: 115, b: 207 });
        map.insert("tufts blue", Color { r: 65, g: 125, b: 193 });
        map.insert("tumbleweed", Color { r: 222, g: 170, b: 136 });
        map.insert("turkish rose", Color { r: 181, g: 114, b: 129 });
        map.insert("turquoise", Color { r: 48, g: 213, b: 200 });
        map.insert("turquoise blue", Color { r: 0, g: 255, b: 239 });
        map.insert("turquoise green", Color { r: 160, g: 214, b: 180 });
        map.insert("tuscan red", Color { r: 124, g: 72, b: 72 });
        map.insert("twilight lavender", Color { r: 138, g: 73, b: 107 });
        map.insert("tyrian purple", Color { r: 102, g: 2, b: 60 });
        map.insert("ua blue", Color { r: 0, g: 51, b: 170 });
        map.insert("ua red", Color { r: 217, g: 0, b: 76 });
        map.insert("ube", Color { r: 136, g: 120, b: 195 });
        map.insert("ucla blue", Color { r: 83, g: 104, b: 149 });
        map.insert("ucla gold", Color { r: 255, g: 179, b: 0 });
        map.insert("ufo green", Color { r: 60, g: 208, b: 112 });
        map.insert("ultra pink", Color { r: 255, g: 111, b: 255 });
        map.insert("ultramarine", Color { r: 18, g: 10, b: 143 });
        map.insert("ultramarine blue", Color { r: 65, g: 102, b: 245 });
        map.insert("umber", Color { r: 99, g: 81, b: 71 });
        map.insert("unbleached silk", Color { r: 255, g: 221, b: 202 });
        map.insert("united nations blue", Color { r: 91, g: 146, b: 229 });
        map.insert("university of california gold", Color { r: 183, g: 135, b: 39 });
        map.insert("unmellow yellow", Color { r: 255, g: 255, b: 102 });
        map.insert("up forest green", Color { r: 1, g: 68, b: 33 });
        map.insert("up maroon", Color { r: 123, g: 17, b: 19 });
        map.insert("upsdell red", Color { r: 174, g: 32, b: 41 });
        map.insert("urobilin", Color { r: 225, g: 173, b: 33 });
        map.insert("usafa blue", Color { r: 0, g: 79, b: 152 });
        map.insert("usc cardinal", Color { r: 153, g: 0, b: 0 });
        map.insert("usc gold", Color { r: 255, g: 204, b: 0 });
        map.insert("utah crimson", Color { r: 211, g: 0, b: 63 });
        map.insert("vanilla", Color { r: 243, g: 229, b: 171 });
        map.insert("vegas gold", Color { r: 197, g: 179, b: 88 });
        map.insert("venetian red", Color { r: 200, g: 8, b: 21 });
        map.insert("verdigris", Color { r: 67, g: 179, b: 174 });
        map.insert("vermilion (cinnabar)", Color { r: 227, g: 66, b: 52 });
        map.insert("vermilion (plochere)", Color { r: 217, g: 96, b: 59 });
        map.insert("veronica", Color { r: 160, g: 32, b: 240 });
        map.insert("violet", Color { r: 143, g: 0, b: 255 });
        map.insert("violet-blue", Color { r: 50, g: 74, b: 178 });
        map.insert("violet (color wheel)", Color { r: 127, g: 0, b: 255 });
        map.insert("violet (ryb)", Color { r: 134, g: 1, b: 175 });
        map.insert("violet (web)", Color { r: 238, g: 130, b: 238 });
        map.insert("viridian", Color { r: 64, g: 130, b: 109 });
        map.insert("vivid auburn", Color { r: 146, g: 39, b: 36 });
        map.insert("vivid burgundy", Color { r: 159, g: 29, b: 53 });
        map.insert("vivid cerise", Color { r: 218, g: 29, b: 129 });
        map.insert("vivid tangerine", Color { r: 255, g: 160, b: 137 });
        map.insert("vivid violet", Color { r: 159, g: 0, b: 255 });
        map.insert("warm black", Color { r: 0, g: 66, b: 66 });
        map.insert("waterspout", Color { r: 164, g: 244, b: 249 });
        map.insert("wenge", Color { r: 100, g: 84, b: 82 });
        map.insert("wheat", Color { r: 245, g: 222, b: 179 });
        map.insert("white", Color { r: 255, g: 255, b: 255 });
        map.insert("white smoke", Color { r: 245, g: 245, b: 245 });
        map.insert("wild blue yonder", Color { r: 162, g: 173, b: 208 });
        map.insert("wild strawberry", Color { r: 255, g: 67, b: 164 });
        map.insert("wild watermelon", Color { r: 252, g: 108, b: 133 });
        map.insert("wine", Color { r: 114, g: 47, b: 55 });
        map.insert("wine dregs", Color { r: 103, g: 49, b: 71 });
        map.insert("wisteria", Color { r: 201, g: 160, b: 220 });
        map.insert("wood brown", Color { r: 193, g: 154, b: 107 });
        map.insert("xanadu", Color { r: 115, g: 134, b: 120 });
        map.insert("yale blue", Color { r: 15, g: 77, b: 146 });
        map.insert("yellow", Color { r: 255, g: 255, b: 0 });
        map.insert("yellow-green", Color { r: 154, g: 205, b: 50 });
        map.insert("yellow (munsell)", Color { r: 239, g: 204, b: 0 });
        map.insert("yellow (ncs)", Color { r: 255, g: 211, b: 0 });
        map.insert("yellow orange", Color { r: 255, g: 174, b: 66 });
        map.insert("yellow (process)", Color { r: 255, g: 239, b: 0 });
        map.insert("yellow (ryb)", Color { r: 254, g: 254, b: 51 });
        map.insert("zaffre", Color { r: 0, g: 20, b: 168 });
        map.insert("zinnwaldite brown", Color { r: 44, g: 22, b: 8 });
        map
    };
}

pub fn find_color_lazy_static(name: &str) -> Option<Color> {
    COLORS_MAP.get(name.to_lowercase().as_str()).cloned()
}

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "air force blue (raf)" => Color { r: 93, g: 138, b: 168 },
    "air force blue (usaf)" => Color { r: 0, g: 48, b: 143 },
    "air superiority blue" => Color { r: 114, g: 160, b: 193 },
    "alabama crimson" => Color { r: 163, g: 38, b: 56 },
    "alice blue" => Color { r: 240, g: 248, b: 255 },
    "alizarin crimson" => Color { r: 227, g: 38, b: 54 },
    "alloy orange" => Color { r: 196, g: 98, b: 16 },
    "almond" => Color { r: 239, g: 222, b: 205 },
    "amaranth" => Color { r: 229, g: 43, b: 80 },
    "amber" => Color { r: 255, g: 191, b: 0 },
    "amber (sae/ece)" => Color { r: 255, g: 126, b: 0 },
    "american rose" => Color { r: 255, g: 3, b: 62 },
    "amethyst" => Color { r: 153, g: 102, b: 204 },
    "android green" => Color { r: 164, g: 198, b: 57 },
    "anti-flash white" => Color { r: 242, g: 243, b: 244 },
    "antique brass" => Color { r: 205, g: 149, b: 117 },
    "antique fuchsia" => Color { r: 145, g: 92, b: 131 },
    "antique ruby" => Color { r: 132, g: 27, b: 45 },
    "antique white" => Color { r: 250, g: 235, b: 215 },
    "ao (english)" => Color { r: 0, g: 128, b: 0 },
    "apple green" => Color { r: 141, g: 182, b: 0 },
    "apricot" => Color { r: 251, g: 206, b: 177 },
    "aqua" => Color { r: 0, g: 255, b: 255 },
    "aquamarine" => Color { r: 127, g: 255, b: 212 },
    "army green" => Color { r: 75, g: 83, b: 32 },
    "arsenic" => Color { r: 59, g: 68, b: 75 },
    "arylide yellow" => Color { r: 233, g: 214, b: 107 },
    "ash grey" => Color { r: 178, g: 190, b: 181 },
    "asparagus" => Color { r: 135, g: 169, b: 107 },
    "atomic tangerine" => Color { r: 255, g: 153, b: 102 },
    "auburn" => Color { r: 165, g: 42, b: 42 },
    "aureolin" => Color { r: 253, g: 238, b: 0 },
    "aurometalsaurus" => Color { r: 110, g: 127, b: 128 },
    "avocado" => Color { r: 86, g: 130, b: 3 },
    "azure" => Color { r: 0, g: 127, b: 255 },
    "azure mist/web" => Color { r: 240, g: 255, b: 255 },
    "baby blue" => Color { r: 137, g: 207, b: 240 },
    "baby blue eyes" => Color { r: 161, g: 202, b: 241 },
    "baby pink" => Color { r: 244, g: 194, b: 194 },
    "ball blue" => Color { r: 33, g: 171, b: 205 },
    "banana mania" => Color { r: 250, g: 231, b: 181 },
    "banana yellow" => Color { r: 255, g: 225, b: 53 },
    "barn red" => Color { r: 124, g: 10, b: 2 },
    "battleship grey" => Color { r: 132, g: 132, b: 130 },
    "bazaar" => Color { r: 152, g: 119, b: 123 },
    "beau blue" => Color { r: 188, g: 212, b: 230 },
    "beaver" => Color { r: 159, g: 129, b: 112 },
    "beige" => Color { r: 245, g: 245, b: 220 },
    "big dip o’ruby" => Color { r: 156, g: 37, b: 66 },
    "bisque" => Color { r: 255, g: 228, b: 196 },
    "bistre" => Color { r: 61, g: 43, b: 31 },
    "bittersweet" => Color { r: 254, g: 111, b: 94 },
    "bittersweet shimmer" => Color { r: 191, g: 79, b: 81 },
    "black" => Color { r: 0, g: 0, b: 0 },
    "black bean" => Color { r: 61, g: 12, b: 2 },
    "black leather jacket" => Color { r: 37, g: 53, b: 41 },
    "black olive" => Color { r: 59, g: 60, b: 54 },
    "blanched almond" => Color { r: 255, g: 235, b: 205 },
    "blast-off bronze" => Color { r: 165, g: 113, b: 100 },
    "bleu de france" => Color { r: 49, g: 140, b: 231 },
    "blizzard blue" => Color { r: 172, g: 229, b: 238 },
    "blond" => Color { r: 250, g: 240, b: 190 },
    "blue" => Color { r: 0, g: 0, b: 255 },
    "blue bell" => Color { r: 162, g: 162, b: 208 },
    "blue (crayola)" => Color { r: 31, g: 117, b: 254 },
    "blue gray" => Color { r: 102, g: 153, b: 204 },
    "blue-green" => Color { r: 13, g: 152, b: 186 },
    "blue (munsell)" => Color { r: 0, g: 147, b: 175 },
    "blue (ncs)" => Color { r: 0, g: 135, b: 189 },
    "blue (pigment)" => Color { r: 51, g: 51, b: 153 },
    "blue (ryb)" => Color { r: 2, g: 71, b: 254 },
    "blue sapphire" => Color { r: 18, g: 97, b: 128 },
    "blue-violet" => Color { r: 138, g: 43, b: 226 },
    "blush" => Color { r: 222, g: 93, b: 131 },
    "bole" => Color { r: 121, g: 68, b: 59 },
    "bondi blue" => Color { r: 0, g: 149, b: 182 },
    "bone" => Color { r: 227, g: 218, b: 201 },
    "boston university red" => Color { r: 204, g: 0, b: 0 },
    "bottle green" => Color { r: 0, g: 106, b: 78 },
    "boysenberry" => Color { r: 135, g: 50, b: 96 },
    "brandeis blue" => Color { r: 0, g: 112, b: 255 },
    "brass" => Color { r: 181, g: 166, b: 66 },
    "brick red" => Color { r: 203, g: 65, b: 84 },
    "bright cerulean" => Color { r: 29, g: 172, b: 214 },
    "bright green" => Color { r: 102, g: 255, b: 0 },
    "bright lavender" => Color { r: 191, g: 148, b: 228 },
    "bright maroon" => Color { r: 195, g: 33, b: 72 },
    "bright pink" => Color { r: 255, g: 0, b: 127 },
    "bright turquoise" => Color { r: 8, g: 232, b: 222 },
    "bright ube" => Color { r: 209, g: 159, b: 232 },
    "brilliant lavender" => Color { r: 244, g: 187, b: 255 },
    "brilliant rose" => Color { r: 255, g: 85, b: 163 },
    "brink pink" => Color { r: 251, g: 96, b: 127 },
    "british racing green" => Color { r: 0, g: 66, b: 37 },
    "bronze" => Color { r: 205, g: 127, b: 50 },
    "brown (traditional)" => Color { r: 150, g: 75, b: 0 },
    "brown (web)" => Color { r: 165, g: 42, b: 42 },
    "bubble gum" => Color { r: 255, g: 193, b: 204 },
    "bubbles" => Color { r: 231, g: 254, b: 255 },
    "buff" => Color { r: 240, g: 220, b: 130 },
    "bulgarian rose" => Color { r: 72, g: 6, b: 7 },
    "burgundy" => Color { r: 128, g: 0, b: 32 },
    "burlywood" => Color { r: 222, g: 184, b: 135 },
    "burnt orange" => Color { r: 204, g: 85, b: 0 },
    "burnt sienna" => Color { r: 233, g: 116, b: 81 },
    "burnt umber" => Color { r: 138, g: 51, b: 36 },
    "byzantine" => Color { r: 189, g: 51, b: 164 },
    "byzantium" => Color { r: 112, g: 41, b: 99 },
    "cadet" => Color { r: 83, g: 104, b: 114 },
    "cadet blue" => Color { r: 95, g: 158, b: 160 },
    "cadet grey" => Color { r: 145, g: 163, b: 176 },
    "cadmium green" => Color { r: 0, g: 107, b: 60 },
    "cadmium orange" => Color { r: 237, g: 135, b: 45 },
    "cadmium red" => Color { r: 227, g: 0, b: 34 },
    "cadmium yellow" => Color { r: 255, g: 246, b: 0 },
    "cafe au lait" => Color { r: 166, g: 123, b: 91 },
    "cafe noir" => Color { r: 75, g: 54, b: 33 },
    "cal poly green" => Color { r: 30, g: 77, b: 43 },
    "cambridge blue" => Color { r: 163, g: 193, b: 173 },
    "camel" => Color { r: 193, g: 154, b: 107 },
    "cameo pink" => Color { r: 239, g: 187, b: 204 },
    "camouflage green" => Color { r: 120, g: 134, b: 107 },
    "canary yellow" => Color { r: 255, g: 239, b: 0 },
    "candy apple red" => Color { r: 255, g: 8, b: 0 },
    "candy pink" => Color { r: 228, g: 113, b: 122 },
    "capri" => Color { r: 0, g: 191, b: 255 },
    "caput mortuum" => Color { r: 89, g: 39, b: 32 },
    "cardinal" => Color { r: 196, g: 30, b: 58 },
    "caribbean green" => Color { r: 0, g: 204, b: 153 },
    "carmine" => Color { r: 150, g: 0, b: 24 },
    "carmine (m&p)" => Color { r: 215, g: 0, b: 64 },
    "carmine pink" => Color { r: 235, g: 76, b: 66 },
    "carmine red" => Color { r: 255, g: 0, b: 56 },
    "carnation pink" => Color { r: 255, g: 166, b: 201 },
    "carnelian" => Color { r: 179, g: 27, b: 27 },
    "carolina blue" => Color { r: 153, g: 186, b: 221 },
    "carrot orange" => Color { r: 237, g: 145, b: 33 },
    "catalina blue" => Color { r: 6, g: 42, b: 120 },
    "ceil" => Color { r: 146, g: 161, b: 207 },
    "celadon" => Color { r: 172, g: 225, b: 175 },
    "celadon blue" => Color { r: 0, g: 123, b: 167 },
    "celadon green" => Color { r: 47, g: 132, b: 124 },
    "celeste (colour)" => Color { r: 178, g: 255, b: 255 },
    "celestial blue" => Color { r: 73, g: 151, b: 208 },
    "cerise" => Color { r: 222, g: 49, b: 99 },
    "cerise pink" => Color { r: 236, g: 59, b: 131 },
    "cerulean" => Color { r: 0, g: 123, b: 167 },
    "cerulean blue" => Color { r: 42, g: 82, b: 190 },
    "cerulean frost" => Color { r: 109, g: 155, b: 195 },
    "cg blue" => Color { r: 0, g: 122, b: 165 },
    "cg red" => Color { r: 224, g: 60, b: 49 },
    "chamoisee" => Color { r: 160, g: 120, b: 90 },
    "champagne" => Color { r: 250, g: 214, b: 165 },
    "charcoal" => Color { r: 54, g: 69, b: 79 },
    "charm pink" => Color { r: 230, g: 143, b: 172 },
    "chartreuse (traditional)" => Color { r: 223, g: 255, b: 0 },
    "chartreuse (web)" => Color { r: 127, g: 255, b: 0 },
    "cherry" => Color { r: 222, g: 49, b: 99 },
    "cherry blossom pink" => Color { r: 255, g: 183, b: 197 },
    "chestnut" => Color { r: 205, g: 92, b: 92 },
    "china pink" => Color { r: 222, g: 111, b: 161 },
    "china rose" => Color { r: 168, g: 81, b: 110 },
    "chinese red" => Color { r: 170, g: 56, b: 30 },
    "chocolate (traditional)" => Color { r: 123, g: 63, b: 0 },
    "chocolate (web)" => Color { r: 210, g: 105, b: 30 },
    "chrome yellow" => Color { r: 255, g: 167, b: 0 },
    "cinereous" => Color { r: 152, g: 129, b: 123 },
    "cinnabar" => Color { r: 227, g: 66, b: 52 },
    "cinnamon" => Color { r: 210, g: 105, b: 30 },
    "citrine" => Color { r: 228, g: 208, b: 10 },
    "classic rose" => Color { r: 251, g: 204, b: 231 },
    "cobalt" => Color { r: 0, g: 71, b: 171 },
    "cocoa brown" => Color { r: 210, g: 105, b: 30 },
    "coffee" => Color { r: 111, g: 78, b: 55 },
    "columbia blue" => Color { r: 155, g: 221, b: 255 },
    "congo pink" => Color { r: 248, g: 131, b: 121 },
    "cool black" => Color { r: 0, g: 46, b: 99 },
    "cool grey" => Color { r: 140, g: 146, b: 172 },
    "copper" => Color { r: 184, g: 115, b: 51 },
    "copper (crayola)" => Color { r: 218, g: 138, b: 103 },
    "copper penny" => Color { r: 173, g: 111, b: 105 },
    "copper red" => Color { r: 203, g: 109, b: 81 },
    "copper rose" => Color { r: 153, g: 102, b: 102 },
    "coquelicot" => Color { r: 255, g: 56, b: 0 },
    "coral" => Color { r: 255, g: 127, b: 80 },
    "coral pink" => Color { r: 248, g: 131, b: 121 },
    "coral red" => Color { r: 255, g: 64, b: 64 },
    "cordovan" => Color { r: 137, g: 63, b: 69 },
    "corn" => Color { r: 251, g: 236, b: 93 },
    "cornell red" => Color { r: 179, g: 27, b: 27 },
    "cornflower blue" => Color { r: 100, g: 149, b: 237 },
    "cornsilk" => Color { r: 255, g: 248, b: 220 },
    "cosmic latte" => Color { r: 255, g: 248, b: 231 },
    "cotton candy" => Color { r: 255, g: 188, b: 217 },
    "cream" => Color { r: 255, g: 253, b: 208 },
    "crimson" => Color { r: 220, g: 20, b: 60 },
    "crimson glory" => Color { r: 190, g: 0, b: 50 },
    "cyan" => Color { r: 0, g: 255, b: 255 },
    "cyan (process)" => Color { r: 0, g: 183, b: 235 },
    "daffodil" => Color { r: 255, g: 255, b: 49 },
    "dandelion" => Color { r: 240, g: 225, b: 48 },
    "dark blue" => Color { r: 0, g: 0, b: 139 },
    "dark brown" => Color { r: 101, g: 67, b: 33 },
    "dark byzantium" => Color { r: 93, g: 57, b: 84 },
    "dark candy apple red" => Color { r: 164, g: 0, b: 0 },
    "dark cerulean" => Color { r: 8, g: 69, b: 126 },
    "dark chestnut" => Color { r: 152, g: 105, b: 96 },
    "dark coral" => Color { r: 205, g: 91, b: 69 },
    "dark cyan" => Color { r: 0, g: 139, b: 139 },
    "dark electric blue" => Color { r: 83, g: 104, b: 120 },
    "dark goldenrod" => Color { r: 184, g: 134, b: 11 },
    "dark gray" => Color { r: 169, g: 169, b: 169 },
    "dark green" => Color { r: 1, g: 50, b: 32 },
    "dark imperial blue" => Color { r: 0, g: 65, b: 106 },
    "dark jungle green" => Color { r: 26, g: 36, b: 33 },
    "dark khaki" => Color { r: 189, g: 183, b: 107 },
    "dark lava" => Color { r: 72, g: 60, b: 50 },
    "dark lavender" => Color { r: 115, g: 79, b: 150 },
    "dark magenta" => Color { r: 139, g: 0, b: 139 },
    "dark midnight blue" => Color { r: 0, g: 51, b: 102 },
    "dark olive green" => Color { r: 85, g: 107, b: 47 },
    "dark orange" => Color { r: 255, g: 140, b: 0 },
    "dark orchid" => Color { r: 153, g: 50, b: 204 },
    "dark pastel blue" => Color { r: 119, g: 158, b: 203 },
    "dark pastel green" => Color { r: 3, g: 192, b: 60 },
    "dark pastel purple" => Color { r: 150, g: 111, b: 214 },
    "dark pastel red" => Color { r: 194, g: 59, b: 34 },
    "dark pink" => Color { r: 231, g: 84, b: 128 },
    "dark powder blue" => Color { r: 0, g: 51, b: 153 },
    "dark raspberry" => Color { r: 135, g: 38, b: 87 },
    "dark red" => Color { r: 139, g: 0, b: 0 },
    "dark salmon" => Color { r: 233, g: 150, b: 122 },
    "dark scarlet" => Color { r: 86, g: 3, b: 25 },
    "dark sea green" => Color { r: 143, g: 188, b: 143 },
    "dark sienna" => Color { r: 60, g: 20, b: 20 },
    "dark slate blue" => Color { r: 72, g: 61, b: 139 },
    "dark slate gray" => Color { r: 47, g: 79, b: 79 },
    "dark spring green" => Color { r: 23, g: 114, b: 69 },
    "dark tan" => Color { r: 145, g: 129, b: 81 },
    "dark tangerine" => Color { r: 255, g: 168, b: 18 },
    "dark taupe" => Color { r: 72, g: 60, b: 50 },
    "dark terra cotta" => Color { r: 204, g: 78, b: 92 },
    "dark turquoise" => Color { r: 0, g: 206, b: 209 },
    "dark violet" => Color { r: 148, g: 0, b: 211 },
    "dark yellow" => Color { r: 155, g: 135, b: 12 },
    "dartmouth green" => Color { r: 0, g: 112, b: 60 },
    "davy's grey" => Color { r: 85, g: 85, b: 85 },
    "debian red" => Color { r: 215, g: 10, b: 83 },
    "deep carmine" => Color { r: 169, g: 32, b: 62 },
    "deep carmine pink" => Color { r: 239, g: 48, b: 56 },
    "deep carrot orange" => Color { r: 233, g: 105, b: 44 },
    "deep cerise" => Color { r: 218, g: 50, b: 135 },
    "deep champagne" => Color { r: 250, g: 214, b: 165 },
    "deep chestnut" => Color { r: 185, g: 78, b: 72 },
    "deep coffee" => Color { r: 112, g: 66, b: 65 },
    "deep fuchsia" => Color { r: 193, g: 84, b: 193 },
    "deep jungle green" => Color { r: 0, g: 75, b: 73 },
    "deep lilac" => Color { r: 153, g: 85, b: 187 },
    "deep magenta" => Color { r: 204, g: 0, b: 204 },
    "deep peach" => Color { r: 255, g: 203, b: 164 },
    "deep pink" => Color { r: 255, g: 20, b: 147 },
    "deep ruby" => Color { r: 132, g: 63, b: 91 },
    "deep saffron" => Color { r: 255, g: 153, b: 51 },
    "deep sky blue" => Color { r: 0, g: 191, b: 255 },
    "deep tuscan red" => Color { r: 102, g: 66, b: 77 },
    "denim" => Color { r: 21, g: 96, b: 189 },
    "desert" => Color { r: 193, g: 154, b: 107 },
    "desert sand" => Color { r: 237, g: 201, b: 175 },
    "dim gray" => Color { r: 105, g: 105, b: 105 },
    "dodger blue" => Color { r: 30, g: 144, b: 255 },
    "dogwood rose" => Color { r: 215, g: 24, b: 104 },
    "dollar bill" => Color { r: 133, g: 187, b: 101 },
    "drab" => Color { r: 150, g: 113, b: 23 },
    "duke blue" => Color { r: 0, g: 0, b: 156 },
    "earth yellow" => Color { r: 225, g: 169, b: 95 },
    "ebony" => Color { r: 85, g: 93, b: 80 },
    "ecru" => Color { r: 194, g: 178, b: 128 },
    "eggplant" => Color { r: 97, g: 64, b: 81 },
    "eggshell" => Color { r: 240, g: 234, b: 214 },
    "egyptian blue" => Color { r: 16, g: 52, b: 166 },
    "electric blue" => Color { r: 125, g: 249, b: 255 },
    "electric crimson" => Color { r: 255, g: 0, b: 63 },
    "electric cyan" => Color { r: 0, g: 255, b: 255 },
    "electric green" => Color { r: 0, g: 255, b: 0 },
    "electric indigo" => Color { r: 111, g: 0, b: 255 },
    "electric lavender" => Color { r: 244, g: 187, b: 255 },
    "electric lime" => Color { r: 204, g: 255, b: 0 },
    "electric purple" => Color { r: 191, g: 0, b: 255 },
    "electric ultramarine" => Color { r: 63, g: 0, b: 255 },
    "electric violet" => Color { r: 143, g: 0, b: 255 },
    "electric yellow" => Color { r: 255, g: 255, b: 0 },
    "emerald" => Color { r: 80, g: 200, b: 120 },
    "english lavender" => Color { r: 180, g: 131, b: 149 },
    "eton blue" => Color { r: 150, g: 200, b: 162 },
    "fallow" => Color { r: 193, g: 154, b: 107 },
    "falu red" => Color { r: 128, g: 24, b: 24 },
    "fandango" => Color { r: 181, g: 51, b: 137 },
    "fashion fuchsia" => Color { r: 244, g: 0, b: 161 },
    "fawn" => Color { r: 229, g: 170, b: 112 },
    "feldgrau" => Color { r: 77, g: 93, b: 83 },
    "fern green" => Color { r: 79, g: 121, b: 66 },
    "ferrari red" => Color { r: 255, g: 40, b: 0 },
    "field drab" => Color { r: 108, g: 84, b: 30 },
    "fire engine red" => Color { r: 206, g: 32, b: 41 },
    "firebrick" => Color { r: 178, g: 34, b: 34 },
    "flame" => Color { r: 226, g: 88, b: 34 },
    "flamingo pink" => Color { r: 252, g: 142, b: 172 },
    "flavescent" => Color { r: 247, g: 233, b: 142 },
    "flax" => Color { r: 238, g: 220, b: 130 },
    "floral white" => Color { r: 255, g: 250, b: 240 },
    "fluorescent orange" => Color { r: 255, g: 191, b: 0 },
    "fluorescent pink" => Color { r: 255, g: 20, b: 147 },
    "fluorescent yellow" => Color { r: 204, g: 255, b: 0 },
    "folly" => Color { r: 255, g: 0, b: 79 },
    "forest green (traditional)" => Color { r: 1, g: 68, b: 33 },
    "forest green (web)" => Color { r: 34, g: 139, b: 34 },
    "french beige" => Color { r: 166, g: 123, b: 91 },
    "french blue" => Color { r: 0, g: 114, b: 187 },
    "french lilac" => Color { r: 134, g: 96, b: 142 },
    "french lime" => Color { r: 204, g: 255, b: 0 },
    "french raspberry" => Color { r: 199, g: 44, b: 72 },
    "french rose" => Color { r: 246, g: 74, b: 138 },
    "fuchsia" => Color { r: 255, g: 0, b: 255 },
    "fuchsia (crayola)" => Color { r: 193, g: 84, b: 193 },
    "fuchsia pink" => Color { r: 255, g: 119, b: 255 },
    "fuchsia rose" => Color { r: 199, g: 67, b: 117 },
    "fulvous" => Color { r: 228, g: 132, b: 0 },
    "fuzzy wuzzy" => Color { r: 204, g: 102, b: 102 },
    "gainsboro" => Color { r: 220, g: 220, b: 220 },
    "gamboge" => Color { r: 228, g: 155, b: 15 },
    "ghost white" => Color { r: 248, g: 248, b: 255 },
    "ginger" => Color { r: 176, g: 101, b: 0 },
    "glaucous" => Color { r: 96, g: 130, b: 182 },
    "glitter" => Color { r: 230, g: 232, b: 250 },
    "gold (metallic)" => Color { r: 212, g: 175, b: 55 },
    "gold (web) (golden)" => Color { r: 255, g: 215, b: 0 },
    "golden brown" => Color { r: 153, g: 101, b: 21 },
    "golden poppy" => Color { r: 252, g: 194, b: 0 },
    "golden yellow" => Color { r: 255, g: 223, b: 0 },
    "goldenrod" => Color { r: 218, g: 165, b: 32 },
    "granny smith apple" => Color { r: 168, g: 228, b: 160 },
    "gray" => Color { r: 128, g: 128, b: 128 },
    "gray-asparagus" => Color { r: 70, g: 89, b: 69 },
    "gray (html/css gray)" => Color { r: 128, g: 128, b: 128 },
    "gray (x11 gray)" => Color { r: 190, g: 190, b: 190 },
    "green (color wheel) (x11 green)" => Color { r: 0, g: 255, b: 0 },
    "green (crayola)" => Color { r: 28, g: 172, b: 120 },
    "green (html/css green)" => Color { r: 0, g: 128, b: 0 },
    "green (munsell)" => Color { r: 0, g: 168, b: 119 },
    "green (ncs)" => Color { r: 0, g: 159, b: 107 },
    "green (pigment)" => Color { r: 0, g: 165, b: 80 },
    "green (ryb)" => Color { r: 102, g: 176, b: 50 },
    "green-yellow" => Color { r: 173, g: 255, b: 47 },
    "grullo" => Color { r: 169, g: 154, b: 134 },
    "guppie green" => Color { r: 0, g: 255, b: 127 },
    "halaya be" => Color { r: 102, g: 56, b: 84 },
    "han blue" => Color { r: 68, g: 108, b: 207 },
    "han purple" => Color { r: 82, g: 24, b: 250 },
    "hansa yellow" => Color { r: 233, g: 214, b: 107 },
    "harlequin" => Color { r: 63, g: 255, b: 0 },
    "harvard crimson" => Color { r: 201, g: 0, b: 22 },
    "harvest gold" => Color { r: 218, g: 145, b: 0 },
    "heart gold" => Color { r: 128, g: 128, b: 0 },
    "heliotrope" => Color { r: 223, g: 115, b: 255 },
    "hollywood cerise" => Color { r: 244, g: 0, b: 161 },
    "honeydew" => Color { r: 240, g: 255, b: 240 },
    "honolulu blue" => Color { r: 0, g: 127, b: 191 },
    "hooker's green" => Color { r: 73, g: 121, b: 107 },
    "hot magenta" => Color { r: 255, g: 29, b: 206 },
    "hot pink" => Color { r: 255, g: 105, b: 180 },
    "hunter green" => Color { r: 53, g: 94, b: 59 },
    "iceberg" => Color { r: 113, g: 166, b: 210 },
    "icterine" => Color { r: 252, g: 247, b: 94 },
    "imperial blue" => Color { r: 0, g: 35, b: 149 },
    "inchworm" => Color { r: 178, g: 236, b: 93 },
    "india green" => Color { r: 19, g: 136, b: 8 },
    "indian red" => Color { r: 205, g: 92, b: 92 },
    "indian yellow" => Color { r: 227, g: 168, b: 87 },
    "indigo" => Color { r: 111, g: 0, b: 255 },
    "indigo (dye)" => Color { r: 0, g: 65, b: 106 },
    "indigo (web)" => Color { r: 75, g: 0, b: 130 },
    "international klein blue" => Color { r: 0, g: 47, b: 167 },
    "international orange (aerospace)" => Color { r: 255, g: 79, b: 0 },
    "international orange (engineering)" => Color { r: 186, g: 22, b: 12 },
    "international orange (golden gate bridge)" => Color { r: 192, g: 54, b: 44 },
    "iris" => Color { r: 90, g: 79, b: 207 },
    "isabelline" => Color { r: 244, g: 240, b: 236 },
    "islamic green" => Color { r: 0, g: 144, b: 0 },
    "ivory" => Color { r: 255, g: 255, b: 240 },
    "jade" => Color { r: 0, g: 168, b: 107 },
    "jasmine" => Color { r: 248, g: 222, b: 126 },
    "jasper" => Color { r: 215, g: 59, b: 62 },
    "jazzberry jam" => Color { r: 165, g: 11, b: 94 },
    "jet" => Color { r: 52, g: 52, b: 52 },
    "jonquil" => Color { r: 250, g: 218, b: 94 },
    "june bud" => Color { r: 189, g: 218, b: 87 },
    "jungle green" => Color { r: 41, g: 171, b: 135 },
    "kelly green" => Color { r: 76, g: 187, b: 23 },
    "kenyan copper" => Color { r: 124, g: 28, b: 5 },
    "khaki (html/css) (khaki)" => Color { r: 195, g: 176, b: 145 },
    "khaki (x11) (light khaki)" => Color { r: 240, g: 230, b: 140 },
    "ku crimson" => Color { r: 232, g: 0, b: 13 },
    "la salle green" => Color { r: 8, g: 120, b: 48 },
    "languid lavender" => Color { r: 214, g: 202, b: 221 },
    "lapis lazuli" => Color { r: 38, g: 97, b: 156 },
    "laser lemon" => Color { r: 254, g: 254, b: 34 },
    "laurel green" => Color { r: 169, g: 186, b: 157 },
    "lava" => Color { r: 207, g: 16, b: 32 },
    "lavender blue" => Color { r: 204, g: 204, b: 255 },
    "lavender blush" => Color { r: 255, g: 240, b: 245 },
    "lavender (floral)" => Color { r: 181, g: 126, b: 220 },
    "lavender gray" => Color { r: 196, g: 195, b: 208 },
    "lavender indigo" => Color { r: 148, g: 87, b: 235 },
    "lavender magenta" => Color { r: 238, g: 130, b: 238 },
    "lavender mist" => Color { r: 230, g: 230, b: 250 },
    "lavender pink" => Color { r: 251, g: 174, b: 210 },
    "lavender purple" => Color { r: 150, g: 123, b: 182 },
    "lavender rose" => Color { r: 251, g: 160, b: 227 },
    "lavender (web)" => Color { r: 230, g: 230, b: 250 },
    "lawn green" => Color { r: 124, g: 252, b: 0 },
    "lemon" => Color { r: 255, g: 247, b: 0 },
    "lemon chiffon" => Color { r: 255, g: 250, b: 205 },
    "lemon lime" => Color { r: 227, g: 255, b: 0 },
    "licorice" => Color { r: 26, g: 17, b: 16 },
    "light apricot" => Color { r: 253, g: 213, b: 177 },
    "light blue" => Color { r: 173, g: 216, b: 230 },
    "light brown" => Color { r: 181, g: 101, b: 29 },
    "light carmine pink" => Color { r: 230, g: 103, b: 113 },
    "light coral" => Color { r: 240, g: 128, b: 128 },
    "light cornflower blue" => Color { r: 147, g: 204, b: 234 },
    "light crimson" => Color { r: 245, g: 105, b: 145 },
    "light cyan" => Color { r: 224, g: 255, b: 255 },
    "light fuchsia pink" => Color { r: 249, g: 132, b: 239 },
    "light goldenrod yellow" => Color { r: 250, g: 250, b: 210 },
    "light gray" => Color { r: 211, g: 211, b: 211 },
    "light green" => Color { r: 144, g: 238, b: 144 },
    "light khaki" => Color { r: 240, g: 230, b: 140 },
    "light pastel purple" => Color { r: 177, g: 156, b: 217 },
    "light pink" => Color { r: 255, g: 182, b: 193 },
    "light red ochre" => Color { r: 233, g: 116, b: 81 },
    "light salmon" => Color { r: 255, g: 160, b: 122 },
    "light salmon pink" => Color { r: 255, g: 153, b: 153 },
    "light sea green" => Color { r: 32, g: 178, b: 170 },
    "light sky blue" => Color { r: 135, g: 206, b: 250 },
    "light slate gray" => Color { r: 119, g: 136, b: 153 },
    "light taupe" => Color { r: 179, g: 139, b: 109 },
    "light thulian pink" => Color { r: 230, g: 143, b: 172 },
    "light yellow" => Color { r: 255, g: 255, b: 224 },
    "lilac" => Color { r: 200, g: 162, b: 200 },
    "lime (color wheel)" => Color { r: 191, g: 255, b: 0 },
    "lime green" => Color { r: 50, g: 205, b: 50 },
    "lime (web) (x11 green)" => Color { r: 0, g: 255, b: 0 },
    "limerick" => Color { r: 157, g: 194, b: 9 },
    "lincoln green" => Color { r: 25, g: 89, b: 5 },
    "linen" => Color { r: 250, g: 240, b: 230 },
    "lion" => Color { r: 193, g: 154, b: 107 },
    "little boy blue" => Color { r: 108, g: 160, b: 220 },
    "liver" => Color { r: 83, g: 75, b: 79 },
    "lust" => Color { r: 230, g: 32, b: 32 },
    "magenta" => Color { r: 255, g: 0, b: 255 },
    "magenta (dye)" => Color { r: 202, g: 31, b: 123 },
    "magenta (process)" => Color { r: 255, g: 0, b: 144 },
    "magic mint" => Color { r: 170, g: 240, b: 209 },
    "magnolia" => Color { r: 248, g: 244, b: 255 },
    "mahogany" => Color { r: 192, g: 64, b: 0 },
    "maize" => Color { r: 251, g: 236, b: 93 },
    "majorelle blue" => Color { r: 96, g: 80, b: 220 },
    "malachite" => Color { r: 11, g: 218, b: 81 },
    "manatee" => Color { r: 151, g: 154, b: 170 },
    "mango tango" => Color { r: 255, g: 130, b: 67 },
    "mantis" => Color { r: 116, g: 195, b: 101 },
    "mardi gras" => Color { r: 136, g: 0, b: 133 },
    "maroon (crayola)" => Color { r: 195, g: 33, b: 72 },
    "maroon (html/css)" => Color { r: 128, g: 0, b: 0 },
    "maroon (x11)" => Color { r: 176, g: 48, b: 96 },
    "mauve" => Color { r: 224, g: 176, b: 255 },
    "mauve taupe" => Color { r: 145, g: 95, b: 109 },
    "mauvelous" => Color { r: 239, g: 152, b: 170 },
    "maya blue" => Color { r: 115, g: 194, b: 251 },
    "meat brown" => Color { r: 229, g: 183, b: 59 },
    "medium aquamarine" => Color { r: 102, g: 221, b: 170 },
    "medium blue" => Color { r: 0, g: 0, b: 205 },
    "medium candy apple red" => Color { r: 226, g: 6, b: 44 },
    "medium carmine" => Color { r: 175, g: 64, b: 53 },
    "medium champagne" => Color { r: 243, g: 229, b: 171 },
    "medium electric blue" => Color { r: 3, g: 80, b: 150 },
    "medium jungle green" => Color { r: 28, g: 53, b: 45 },
    "medium lavender magenta" => Color { r: 221, g: 160, b: 221 },
    "medium orchid" => Color { r: 186, g: 85, b: 211 },
    "medium persian blue" => Color { r: 0, g: 103, b: 165 },
    "medium purple" => Color { r: 147, g: 112, b: 219 },
    "medium red-violet" => Color { r: 187, g: 51, b: 133 },
    "medium ruby" => Color { r: 170, g: 64, b: 105 },
    "medium sea green" => Color { r: 60, g: 179, b: 113 },
    "medium slate blue" => Color { r: 123, g: 104, b: 238 },
    "medium spring bud" => Color { r: 201, g: 220, b: 135 },
    "medium spring green" => Color { r: 0, g: 250, b: 154 },
    "medium taupe" => Color { r: 103, g: 76, b: 71 },
    "medium turquoise" => Color { r: 72, g: 209, b: 204 },
    "medium tuscan red" => Color { r: 121, g: 68, b: 59 },
    "medium vermilion" => Color { r: 217, g: 96, b: 59 },
    "medium violet-red" => Color { r: 199, g: 21, b: 133 },
    "mellow apricot" => Color { r: 248, g: 184, b: 120 },
    "mellow yellow" => Color { r: 248, g: 222, b: 126 },
    "melon" => Color { r: 253, g: 188, b: 180 },
    "midnight blue" => Color { r: 25, g: 25, b: 112 },
    "midnight green (eagle green)" => Color { r: 0, g: 73, b: 83 },
    "mikado yellow" => Color { r: 255, g: 196, b: 12 },
    "mint" => Color { r: 62, g: 180, b: 137 },
    "mint cream" => Color { r: 245, g: 255, b: 250 },
    "mint green" => Color { r: 152, g: 255, b: 152 },
    "misty rose" => Color { r: 255, g: 228, b: 225 },
    "moccasin" => Color { r: 250, g: 235, b: 215 },
    "mode beige" => Color { r: 150, g: 113, b: 23 },
    "moonstone blue" => Color { r: 115, g: 169, b: 194 },
    "mordant red 19" => Color { r: 174, g: 12, b: 0 },
    "moss green" => Color { r: 173, g: 223, b: 173 },
    "mountain meadow" => Color { r: 48, g: 186, b: 143 },
    "mountbatten pink" => Color { r: 153, g: 122, b: 141 },
    "msu green" => Color { r: 24, g: 69, b: 59 },
    "mulberry" => Color { r: 197, g: 75, b: 140 },
    "mustard" => Color { r: 255, g: 219, b: 88 },
    "myrtle" => Color { r: 33, g: 66, b: 30 },
    "nadeshiko pink" => Color { r: 246, g: 173, b: 198 },
    "napier green" => Color { r: 42, g: 128, b: 0 },
    "naples yellow" => Color { r: 250, g: 218, b: 94 },
    "navajo white" => Color { r: 255, g: 222, b: 173 },
    "navy blue" => Color { r: 0, g: 0, b: 128 },
    "neon carrot" => Color { r: 255, g: 163, b: 67 },
    "neon fuchsia" => Color { r: 254, g: 65, b: 100 },
    "neon green" => Color { r: 57, g: 255, b: 20 },
    "new york pink" => Color { r: 215, g: 131, b: 127 },
    "non-photo blue" => Color { r: 164, g: 221, b: 237 },
    "north texas green" => Color { r: 5, g: 144, b: 51 },
    "ocean boat blue" => Color { r: 0, g: 119, b: 190 },
    "ochre" => Color { r: 204, g: 119, b: 34 },
    "office green" => Color { r: 0, g: 128, b: 0 },
    "old gold" => Color { r: 207, g: 181, b: 59 },
    "old lace" => Color { r: 253, g: 245, b: 230 },
    "old lavender" => Color { r: 121, g: 104, b: 120 },
    "old mauve" => Color { r: 103, g: 49, b: 71 },
    "old rose" => Color { r: 192, g: 128, b: 129 },
    "olive" => Color { r: 128, g: 128, b: 0 },
    "olive drab #7" => Color { r: 60, g: 52, b: 31 },
    "olive drab (web) (olive drab #3)" => Color { r: 107, g: 142, b: 35 },
    "olivine" => Color { r: 154, g: 185, b: 115 },
    "onyx" => Color { r: 53, g: 56, b: 57 },
    "opera mauve" => Color { r: 183, g: 132, b: 167 },
    "orange (color wheel)" => Color { r: 255, g: 127, b: 0 },
    "orange peel" => Color { r: 255, g: 159, b: 0 },
    "orange-red" => Color { r: 255, g: 69, b: 0 },
    "orange (ryb)" => Color { r: 251, g: 153, b: 2 },
    "orange (web color)" => Color { r: 255, g: 165, b: 0 },
    "orchid" => Color { r: 218, g: 112, b: 214 },
    "otter brown" => Color { r: 101, g: 67, b: 33 },
    "ou crimson red" => Color { r: 153, g: 0, b: 0 },
    "outer space" => Color { r: 65, g: 74, b: 76 },
    "outrageous orange" => Color { r: 255, g: 110, b: 74 },
    "oxford blue" => Color { r: 0, g: 33, b: 71 },
    "pakistan green" => Color { r: 0, g: 102, b: 0 },
    "palatinate blue" => Color { r: 39, g: 59, b: 226 },
    "palatinate purple" => Color { r: 104, g: 40, b: 96 },
    "pale aqua" => Color { r: 188, g: 212, b: 230 },
    "pale blue" => Color { r: 175, g: 238, b: 238 },
    "pale brown" => Color { r: 152, g: 118, b: 84 },
    "pale carmine" => Color { r: 175, g: 64, b: 53 },
    "pale cerulean" => Color { r: 155, g: 196, b: 226 },
    "pale chestnut" => Color { r: 221, g: 173, b: 175 },
    "pale copper" => Color { r: 218, g: 138, b: 103 },
    "pale cornflower blue" => Color { r: 171, g: 205, b: 239 },
    "pale gold" => Color { r: 230, g: 190, b: 138 },
    "pale goldenrod" => Color { r: 238, g: 232, b: 170 },
    "pale green" => Color { r: 152, g: 251, b: 152 },
    "pale lavender" => Color { r: 220, g: 208, b: 255 },
    "pale magenta" => Color { r: 249, g: 132, b: 229 },
    "pale pink" => Color { r: 250, g: 218, b: 221 },
    "pale plum" => Color { r: 221, g: 160, b: 221 },
    "pale red-violet" => Color { r: 219, g: 112, b: 147 },
    "pale robin egg blue" => Color { r: 150, g: 222, b: 209 },
    "pale silver" => Color { r: 201, g: 192, b: 187 },
    "pale spring bud" => Color { r: 236, g: 235, b: 189 },
    "pale taupe" => Color { r: 188, g: 152, b: 126 },
    "pale violet-red" => Color { r: 219, g: 112, b: 147 },
    "pansy purple" => Color { r: 120, g: 24, b: 74 },
    "papaya whip" => Color { r: 255, g: 239, b: 213 },
    "paris green" => Color { r: 80, g: 200, b: 120 },
    "pastel blue" => Color { r: 174, g: 198, b: 207 },
    "pastel brown" => Color { r: 131, g: 105, b: 83 },
    "pastel gray" => Color { r: 207, g: 207, b: 196 },
    "pastel green" => Color { r: 119, g: 221, b: 119 },
    "pastel magenta" => Color { r: 244, g: 154, b: 194 },
    "pastel orange" => Color { r: 255, g: 179, b: 71 },
    "pastel pink" => Color { r: 222, g: 165, b: 164 },
    "pastel purple" => Color { r: 179, g: 158, b: 181 },
    "pastel red" => Color { r: 255, g: 105, b: 97 },
    "pastel violet" => Color { r: 203, g: 153, b: 201 },
    "pastel yellow" => Color { r: 253, g: 253, b: 150 },
    "patriarch" => Color { r: 128, g: 0, b: 128 },
    "payne's grey" => Color { r: 83, g: 104, b: 120 },
    "peach" => Color { r: 255, g: 229, b: 180 },
    "peach (crayola)" => Color { r: 255, g: 203, b: 164 },
    "peach-orange" => Color { r: 255, g: 204, b: 153 },
    "peach puff" => Color { r: 255, g: 218, b: 185 },
    "peach-yellow" => Color { r: 250, g: 223, b: 173 },
    "pear" => Color { r: 209, g: 226, b: 49 },
    "pearl" => Color { r: 234, g: 224, b: 200 },
    "pearl aqua" => Color { r: 136, g: 216, b: 192 },
    "pearly purple" => Color { r: 183, g: 104, b: 162 },
    "peridot" => Color { r: 230, g: 226, b: 0 },
    "periwinkle" => Color { r: 204, g: 204, b: 255 },
    "persian blue" => Color { r: 28, g: 57, b: 187 },
    "persian green" => Color { r: 0, g: 166, b: 147 },
    "persian indigo" => Color { r: 50, g: 18, b: 122 },
    "persian orange" => Color { r: 217, g: 144, b: 88 },
    "persian pink" => Color { r: 247, g: 127, b: 190 },
    "persian plum" => Color { r: 112, g: 28, b: 28 },
    "persian red" => Color { r: 204, g: 51, b: 51 },
    "persian rose" => Color { r: 254, g: 40, b: 162 },
    "persimmon" => Color { r: 236, g: 88, b: 0 },
    "peru" => Color { r: 205, g: 133, b: 63 },
    "phlox" => Color { r: 223, g: 0, b: 255 },
    "phthalo blue" => Color { r: 0, g: 15, b: 137 },
    "phthalo green" => Color { r: 18, g: 53, b: 36 },
    "piggy pink" => Color { r: 253, g: 221, b: 230 },
    "pine green" => Color { r: 1, g: 121, b: 111 },
    "pink" => Color { r: 255, g: 192, b: 203 },
    "pink lace" => Color { r: 255, g: 221, b: 244 },
    "pink-orange" => Color { r: 255, g: 153, b: 102 },
    "pink pearl" => Color { r: 231, g: 172, b: 207 },
    "pink sherbet" => Color { r: 247, g: 143, b: 167 },
    "pistachio" => Color { r: 147, g: 197, b: 114 },
    "platinum" => Color { r: 229, g: 228, b: 226 },
    "plum (traditional)" => Color { r: 142, g: 69, b: 133 },
    "plum (web)" => Color { r: 221, g: 160, b: 221 },
    "portland orange" => Color { r: 255, g: 90, b: 54 },
    "powder blue (web)" => Color { r: 176, g: 224, b: 230 },
    "princeton orange" => Color { r: 255, g: 143, b: 0 },
    "prune" => Color { r: 112, g: 28, b: 28 },
    "prussian blue" => Color { r: 0, g: 49, b: 83 },
    "psychedelic purple" => Color { r: 223, g: 0, b: 255 },
    "puce" => Color { r: 204, g: 136, b: 153 },
    "pumpkin" => Color { r: 255, g: 117, b: 24 },
    "purple heart" => Color { r: 105, g: 53, b: 156 },
    "purple (html/css)" => Color { r: 128, g: 0, b: 128 },
    "purple mountain majesty" => Color { r: 150, g: 120, b: 182 },
    "purple (munsell)" => Color { r: 159, g: 0, b: 197 },
    "purple pizzazz" => Color { r: 254, g: 78, b: 218 },
    "purple taupe" => Color { r: 80, g: 64, b: 77 },
    "purple (x11)" => Color { r: 160, g: 32, b: 240 },
    "quartz" => Color { r: 81, g: 72, b: 79 },
    "rackley" => Color { r: 93, g: 138, b: 168 },
    "radical red" => Color { r: 255, g: 53, b: 94 },
    "rajah" => Color { r: 251, g: 171, b: 96 },
    "raspberry" => Color { r: 227, g: 11, b: 93 },
    "raspberry glace" => Color { r: 145, g: 95, b: 109 },
    "raspberry pink" => Color { r: 226, g: 80, b: 152 },
    "raspberry rose" => Color { r: 179, g: 68, b: 108 },
    "raw umber" => Color { r: 130, g: 102, b: 68 },
    "razzle dazzle rose" => Color { r: 255, g: 51, b: 204 },
    "razzmatazz" => Color { r: 227, g: 37, b: 107 },
    "red" => Color { r: 255, g: 0, b: 0 },
    "red-brown" => Color { r: 165, g: 42, b: 42 },
    "red devil" => Color { r: 134, g: 1, b: 17 },
    "red (munsell)" => Color { r: 242, g: 0, b: 60 },
    "red (ncs)" => Color { r: 196, g: 2, b: 51 },
    "red-orange" => Color { r: 255, g: 83, b: 73 },
    "red (pigment)" => Color { r: 237, g: 28, b: 36 },
    "red (ryb)" => Color { r: 254, g: 39, b: 18 },
    "red-violet" => Color { r: 199, g: 21, b: 133 },
    "redwood" => Color { r: 171, g: 78, b: 82 },
    "regalia" => Color { r: 82, g: 45, b: 128 },
    "resolution blue" => Color { r: 0, g: 35, b: 135 },
    "rich black" => Color { r: 0, g: 64, b: 64 },
    "rich brilliant lavender" => Color { r: 241, g: 167, b: 254 },
    "rich carmine" => Color { r: 215, g: 0, b: 64 },
    "rich electric blue" => Color { r: 8, g: 146, b: 208 },
    "rich lavender" => Color { r: 167, g: 107, b: 207 },
    "rich lilac" => Color { r: 182, g: 102, b: 210 },
    "rich maroon" => Color { r: 176, g: 48, b: 96 },
    "rifle green" => Color { r: 65, g: 72, b: 51 },
    "robin egg blue" => Color { r: 0, g: 204, b: 204 },
    "rose" => Color { r: 255, g: 0, b: 127 },
    "rose bonbon" => Color { r: 249, g: 66, b: 158 },
    "rose ebony" => Color { r: 103, g: 72, b: 70 },
    "rose gold" => Color { r: 183, g: 110, b: 121 },
    "rose madder" => Color { r: 227, g: 38, b: 54 },
    "rose pink" => Color { r: 255, g: 102, b: 204 },
    "rose quartz" => Color { r: 170, g: 152, b: 169 },
    "rose taupe" => Color { r: 144, g: 93, b: 93 },
    "rose vale" => Color { r: 171, g: 78, b: 82 },
    "rosewood" => Color { r: 101, g: 0, b: 11 },
    "rosso corsa" => Color { r: 212, g: 0, b: 0 },
    "rosy brown" => Color { r: 188, g: 143, b: 143 },
    "royal azure" => Color { r: 0, g: 56, b: 168 },
    "royal blue (traditional)" => Color { r: 0, g: 35, b: 102 },
    "royal blue (web)" => Color { r: 65, g: 105, b: 225 },
    "royal fuchsia" => Color { r: 202, g: 44, b: 146 },
    "royal purple" => Color { r: 120, g: 81, b: 169 },
    "royal yellow" => Color { r: 250, g: 218, b: 94 },
    "rubine red" => Color { r: 209, g: 0, b: 86 },
    "ruby" => Color { r: 224, g: 17, b: 95 },
    "ruby red" => Color { r: 155, g: 17, b: 30 },
    "ruddy" => Color { r: 255, g: 0, b: 40 },
    "ruddy brown" => Color { r: 187, g: 101, b: 40 },
    "ruddy pink" => Color { r: 225, g: 142, b: 150 },
    "rufous" => Color { r: 168, g: 28, b: 7 },
    "russet" => Color { r: 128, g: 70, b: 27 },
    "rust" => Color { r: 183, g: 65, b: 14 },
    "rusty red" => Color { r: 218, g: 44, b: 67 },
    "sacramento state green" => Color { r: 0, g: 86, b: 63 },
    "saddle brown" => Color { r: 139, g: 69, b: 19 },
    "safety orange (blaze orange)" => Color { r: 255, g: 103, b: 0 },
    "saffron" => Color { r: 244, g: 196, b: 48 },
    "salmon" => Color { r: 255, g: 140, b: 105 },
    "salmon pink" => Color { r: 255, g: 145, b: 164 },
    "sand" => Color { r: 194, g: 178, b: 128 },
    "sand dune" => Color { r: 150, g: 113, b: 23 },
    "sandstorm" => Color { r: 236, g: 213, b: 64 },
    "sandy brown" => Color { r: 244, g: 164, b: 96 },
    "sandy taupe" => Color { r: 150, g: 113, b: 23 },
    "sangria" => Color { r: 146, g: 0, b: 10 },
    "sap green" => Color { r: 80, g: 125, b: 42 },
    "sapphire" => Color { r: 15, g: 82, b: 186 },
    "sapphire blue" => Color { r: 0, g: 103, b: 165 },
    "satin sheen gold" => Color { r: 203, g: 161, b: 53 },
    "scarlet" => Color { r: 255, g: 36, b: 0 },
    "scarlet (crayola)" => Color { r: 253, g: 14, b: 53 },
    "school bus yellow" => Color { r: 255, g: 216, b: 0 },
    "screamin' green" => Color { r: 118, g: 255, b: 122 },
    "sea blue" => Color { r: 0, g: 105, b: 148 },
    "sea green" => Color { r: 46, g: 139, b: 87 },
    "seal brown" => Color { r: 50, g: 20, b: 20 },
    "seashell" => Color { r: 255, g: 245, b: 238 },
    "selective yellow" => Color { r: 255, g: 186, b: 0 },
    "sepia" => Color { r: 112, g: 66, b: 20 },
    "shadow" => Color { r: 138, g: 121, b: 93 },
    "shamrock green" => Color { r: 0, g: 158, b: 96 },
    "shocking pink" => Color { r: 252, g: 15, b: 192 },
    "shocking pink (crayola)" => Color { r: 255, g: 111, b: 255 },
    "sienna" => Color { r: 136, g: 45, b: 23 },
    "silver" => Color { r: 192, g: 192, b: 192 },
    "sinopia" => Color { r: 203, g: 65, b: 11 },
    "skobeloff" => Color { r: 0, g: 116, b: 116 },
    "sky blue" => Color { r: 135, g: 206, b: 235 },
    "sky magenta" => Color { r: 207, g: 113, b: 175 },
    "slate blue" => Color { r: 106, g: 90, b: 205 },
    "slate gray" => Color { r: 112, g: 128, b: 144 },
    "smalt (dark powder blue)" => Color { r: 0, g: 51, b: 153 },
    "smokey topaz" => Color { r: 147, g: 61, b: 65 },
    "smoky black" => Color { r: 16, g: 12, b: 8 },
    "snow" => Color { r: 255, g: 250, b: 250 },
    "spiro disco ball" => Color { r: 15, g: 192, b: 252 },
    "spring bud" => Color { r: 167, g: 252, b: 0 },
    "spring green" => Color { r: 0, g: 255, b: 127 },
    "st. patrick's blue" => Color { r: 35, g: 41, b: 122 },
    "steel blue" => Color { r: 70, g: 130, b: 180 },
    "stil de grain yellow" => Color { r: 250, g: 218, b: 94 },
    "stizza" => Color { r: 153, g: 0, b: 0 },
    "stormcloud" => Color { r: 79, g: 102, b: 106 },
    "straw" => Color { r: 228, g: 217, b: 111 },
    "sunglow" => Color { r: 255, g: 204, b: 51 },
    "sunset" => Color { r: 250, g: 214, b: 165 },
    "tan" => Color { r: 210, g: 180, b: 140 },
    "tangelo" => Color { r: 249, g: 77, b: 0 },
    "tangerine" => Color { r: 242, g: 133, b: 0 },
    "tangerine yellow" => Color { r: 255, g: 204, b: 0 },
    "tango pink" => Color { r: 228, g: 113, b: 122 },
    "taupe" => Color { r: 72, g: 60, b: 50 },
    "taupe gray" => Color { r: 139, g: 133, b: 137 },
    "tea green" => Color { r: 208, g: 240, b: 192 },
    "tea rose (orange)" => Color { r: 248, g: 131, b: 121 },
    "tea rose (rose)" => Color { r: 244, g: 194, b: 194 },
    "teal" => Color { r: 0, g: 128, b: 128 },
    "teal blue" => Color { r: 54, g: 117, b: 136 },
    "teal green" => Color { r: 0, g: 130, b: 127 },
    "telemagenta" => Color { r: 207, g: 52, b: 118 },
    "tenne (tawny)" => Color { r: 205, g: 87, b: 0 },
    "terra cotta" => Color { r: 226, g: 114, b: 91 },
    "thistle" => Color { r: 216, g: 191, b: 216 },
    "thulian pink" => Color { r: 222, g: 111, b: 161 },
    "tickle me pink" => Color { r: 252, g: 137, b: 172 },
    "tiffany blue" => Color { r: 10, g: 186, b: 181 },
    "tiger's eye" => Color { r: 224, g: 141, b: 60 },
    "timberwolf" => Color { r: 219, g: 215, b: 210 },
    "titanium yellow" => Color { r: 238, g: 230, b: 0 },
    "tomato" => Color { r: 255, g: 99, b: 71 },
    "toolbox" => Color { r: 116, g: 108, b: 192 },
    "topaz" => Color { r: 255, g: 200, b: 124 },
    "tractor red" => Color { r: 253, g: 14, b: 53 },
    "trolley grey" => Color { r: 128, g: 128, b: 128 },
    "tropical rain forest" => Color { r: 0, g: 117, b: 94 },
    "true blue" => Color { r: 0, g: 115, b: 207 },
    "tufts blue" => Color { r: 65, g: 125, b: 193 },
    "tumbleweed" => Color { r: 222, g: 170, b: 136 },
    "turkish rose" => Color { r: 181, g: 114, b: 129 },
    "turquoise" => Color { r: 48, g: 213, b: 200 },
    "turquoise blue" => Color { r: 0, g: 255, b: 239 },
    "turquoise green" => Color { r: 160, g: 214, b: 180 },
    "tuscan red" => Color { r: 124, g: 72, b: 72 },
    "twilight lavender" => Color { r: 138, g: 73, b: 107 },
    "tyrian purple" => Color { r: 102, g: 2, b: 60 },
    "ua blue" => Color { r: 0, g: 51, b: 170 },
    "ua red" => Color { r: 217, g: 0, b: 76 },
    "ube" => Color { r: 136, g: 120, b: 195 },
    "ucla blue" => Color { r: 83, g: 104, b: 149 },
    "ucla gold" => Color { r: 255, g: 179, b: 0 },
    "ufo green" => Color { r: 60, g: 208, b: 112 },
    "ultra pink" => Color { r: 255, g: 111, b: 255 },
    "ultramarine" => Color { r: 18, g: 10, b: 143 },
    "ultramarine blue" => Color { r: 65, g: 102, b: 245 },
    "umber" => Color { r: 99, g: 81, b: 71 },
    "unbleached silk" => Color { r: 255, g: 221, b: 202 },
    "united nations blue" => Color { r: 91, g: 146, b: 229 },
    "university of california gold" => Color { r: 183, g: 135, b: 39 },
    "unmellow yellow" => Color { r: 255, g: 255, b: 102 },
    "up forest green" => Color { r: 1, g: 68, b: 33 },
    "up maroon" => Color { r: 123, g: 17, b: 19 },
    "upsdell red" => Color { r: 174, g: 32, b: 41 },
    "urobilin" => Color { r: 225, g: 173, b: 33 },
    "usafa blue" => Color { r: 0, g: 79, b: 152 },
    "usc cardinal" => Color { r: 153, g: 0, b: 0 },
    "usc gold" => Color { r: 255, g: 204, b: 0 },
    "utah crimson" => Color { r: 211, g: 0, b: 63 },
    "vanilla" => Color { r: 243, g: 229, b: 171 },
    "vegas gold" => Color { r: 197, g: 179, b: 88 },
    "venetian red" => Color { r: 200, g: 8, b: 21 },
    "verdigris" => Color { r: 67, g: 179, b: 174 },
    "vermilion (cinnabar)" => Color { r: 227, g: 66, b: 52 },
    "vermilion (plochere)" => Color { r: 217, g: 96, b: 59 },
    "veronica" => Color { r: 160, g: 32, b: 240 },
    "violet" => Color { r: 143, g: 0, b: 255 },
    "violet-blue" => Color { r: 50, g: 74, b: 178 },
    "violet (color wheel)" => Color { r: 127, g: 0, b: 255 },
    "violet (ryb)" => Color { r: 134, g: 1, b: 175 },
    "violet (web)" => Color { r: 238, g: 130, b: 238 },
    "viridian" => Color { r: 64, g: 130, b: 109 },
    "vivid auburn" => Color { r: 146, g: 39, b: 36 },
    "vivid burgundy" => Color { r: 159, g: 29, b: 53 },
    "vivid cerise" => Color { r: 218, g: 29, b: 129 },
    "vivid tangerine" => Color { r: 255, g: 160, b: 137 },
    "vivid violet" => Color { r: 159, g: 0, b: 255 },
    "warm black" => Color { r: 0, g: 66, b: 66 },
    "waterspout" => Color { r: 164, g: 244, b: 249 },
    "wenge" => Color { r: 100, g: 84, b: 82 },
    "wheat" => Color { r: 245, g: 222, b: 179 },
    "white" => Color { r: 255, g: 255, b: 255 },
    "white smoke" => Color { r: 245, g: 245, b: 245 },
    "wild blue yonder" => Color { r: 162, g: 173, b: 208 },
    "wild strawberry" => Color { r: 255, g: 67, b: 164 },
    "wild watermelon" => Color { r: 252, g: 108, b: 133 },
    "wine" => Color { r: 114, g: 47, b: 55 },
    "wine dregs" => Color { r: 103, g: 49, b: 71 },
    "wisteria" => Color { r: 201, g: 160, b: 220 },
    "wood brown" => Color { r: 193, g: 154, b: 107 },
    "xanadu" => Color { r: 115, g: 134, b: 120 },
    "yale blue" => Color { r: 15, g: 77, b: 146 },
    "yellow" => Color { r: 255, g: 255, b: 0 },
    "yellow-green" => Color { r: 154, g: 205, b: 50 },
    "yellow (munsell)" => Color { r: 239, g: 204, b: 0 },
    "yellow (ncs)" => Color { r: 255, g: 211, b: 0 },
    "yellow orange" => Color { r: 255, g: 174, b: 66 },
    "yellow (process)" => Color { r: 255, g: 239, b: 0 },
    "yellow (ryb)" => Color { r: 254, g: 254, b: 51 },
    "zaffre" => Color { r: 0, g: 20, b: 168 },
    "zinnwaldite brown" => Color { r: 44, g: 22, b: 8 },
};

pub fn find_color_phf(name: &str) -> Option<Color> {
    COLORS.get(name.to_lowercase().as_str()).cloned()
}

