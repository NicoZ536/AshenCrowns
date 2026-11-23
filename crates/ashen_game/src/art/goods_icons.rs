//! Icons der Waren und Betriebszustände der Echtzeit-Wirtschaft (16×16-Pixel-Art).
//!
//! Die Motive stehen als kleine Zeichenraster im Quelltext: jedes Zeichen verweist auf eine Palettenfarbe,
//! `.` ist durchsichtig. Das Raster wird mittig in 16×16 gesetzt und wie alle Icons mit Kontur versehen.

use ashen_core::color::Rgba;
use ashen_engine::assets::{AtlasBuilder, Canvas};
use glam::Vec2;

/// Setzt ein Zeichenraster mittig auf eine 16×16-Fläche.
fn art(rows: &[&str], pal: &[(char, u32)]) -> Canvas {
    let mut c = Canvas::new(16, 16);
    let h = rows.len() as i32;
    let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as i32;
    let (ox, oy) = ((16 - w) / 2, (16 - h) / 2);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if let Some((_, hex)) = pal.iter().find(|(k, _)| *k == ch) {
                c.px(ox + x as i32, oy + y as i32, Rgba::hex(*hex));
            }
        }
    }
    c
}

fn finish(mut c: Canvas) -> Canvas {
    c.outline(0.3);
    c
}

/// Runde Sprechblase als Grundlage der Statussymbole.
fn bubble(fill: u32, rim: u32) -> Canvas {
    let mut c = Canvas::new(16, 16);
    for y in 0..16 {
        for x in 0..16 {
            let (dx, dy) = (x as f32 - 7.5, y as f32 - 7.5);
            let d = (dx * dx + dy * dy).sqrt();
            if d <= 7.2 {
                let col = if d > 6.0 { rim } else if dy < -2.0 && dx < 1.0 { fill.saturating_add(0x101010) } else { fill };
                c.px(x, y, Rgba::hex(col));
            }
        }
    }
    c
}

fn overlay(c: &mut Canvas, rows: &[&str], pal: &[(char, u32)]) {
    let h = rows.len() as i32;
    let w = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as i32;
    let (ox, oy) = ((16 - w) / 2, (16 - h) / 2);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if let Some((_, hex)) = pal.iter().find(|(k, _)| *k == ch) {
                c.px(ox + x as i32, oy + y as i32, Rgba::hex(*hex));
            }
        }
    }
}

pub fn build(atlas: &mut AtlasBuilder) {
    let mut add = |name: &str, c: Canvas| atlas.add(format!("icon_{name}"), finish(c), Vec2::ZERO);

    // ---- Rohstoffe
    add(
        "clay",
        art(
            &["...hhhhh....", "..hhbbbbbd..", ".hbbbbbbbbd.", "hbbbbbbbbbbd", "bbbbbbbbbbdd", ".bbbbbbbbdd.", "..dddddddd.."],
            &[('h', 0xc08a5a), ('b', 0x94603c), ('d', 0x5e3c26)],
        ),
    );
    add(
        "iron_ore",
        art(
            &["...rrrr....", "..rbbbbrr..", ".rbbmbbbbr.", "rbbbbbbmbbr", "rbmbbbbbbbd", ".rbbbbbbbd.", "..dddddddd."],
            &[('r', 0x7a5a48), ('b', 0x5a4438), ('m', 0xc8a070), ('d', 0x2e2420)],
        ),
    );
    add(
        "coal",
        art(
            &["...kkkk....", "..kllkkkk..", ".kkkkkklkk.", "kkklkkkkkkk", "kkkkkkkklkd", ".kkkkkkkkd.", "..dddddddd."],
            &[('k', 0x2a282c), ('l', 0x5a585e), ('d', 0x121114)],
        ),
    );
    add(
        "grain",
        art(
            &["..y..y..y..", ".yty.yty.yt.", ".yyy.yyy.yy.", ".tyt.tyt.ty.", "..y..y..y...", "..s..s..s...", "..s..s..s...", ".sss.sss.ss."],
            &[('y', 0xe0b84a), ('t', 0xa8802a), ('s', 0x8a7a3a)],
        ),
    );
    add(
        "wool",
        art(
            &["...wwww....", "..wWWWWww..", ".wWWWWWWWw.", "wWWWWWWWWWw", "wWWWWWWWWwd", ".wwwwwwwwd.", "..dddddddd."],
            &[('W', 0xf4f0e4), ('w', 0xd6d0c0), ('d', 0x8a8478)],
        ),
    );
    add(
        "herbs",
        art(
            &["....g..g....", "..g.gg.g.p.", ".ggg.gggppp", "..gggggg.p.", "...gggg....", "....gg.....", "....s......", "....s......"],
            &[('g', 0x5a9a3c), ('p', 0x9a5aa8), ('s', 0x4a6a2c)],
        ),
    );

    // ---- Zwischengüter
    add(
        "planks",
        art(
            &[".wwwwwwwwwww.", "wLLLLLLLLLLwd", "wDDDDDDDDDDDd", ".wwwwwwwwwww.", "wLLLLLLLLLLwd", "wDDDDDDDDDDDd", ".wwwwwwwwwww."],
            &[('w', 0xb48a5a), ('L', 0xd0a470), ('D', 0x7a5632), ('d', 0x4a3220)],
        ),
    );
    add(
        "bricks",
        art(
            &["rrrrr.rrrrr", "rRRRr.rRRRr", "rrrrr.rrrrr", "..rrrrr.rrr", "..rRRRr.rRR", "..rrrrr.rrr", "rrrrr.rrrrr", "rRRRr.rRRRr"],
            &[('r', 0x9a4a32), ('R', 0xc2694a)],
        ),
    );
    add(
        "iron",
        art(
            &["..sssssssss.", ".sLLLLLLLLsd", "sLLLLLLLLLLd", "sddddddddddd", "..sssssssss.", ".sLLLLLLLLsd", "sddddddddddd"],
            &[('s', 0x8a96a0), ('L', 0xc4ced6), ('d', 0x4a5560)],
        ),
    );
    add(
        "flour",
        art(
            &["..tttttttt..", ".ttwwwwwwtt.", "tbwwwwwwwwbt", "twwwwwwwwwwt", "twwFFFFFFwwt", "twwwwwwwwwwt", ".tttttttttt."],
            &[('t', 0xb8a078), ('b', 0x8a7448), ('w', 0xe8dfc8), ('F', 0xc8b890)],
        ),
    );
    add(
        "cloth",
        art(
            &["bbbbbbbbbbb.", "bBBBBBBBBBbb", "bBBBBBBBBBBb", "bddddddddddb", "bBBBBBBBBBBb", "bBBBBBBBBBbb", ".bbbbbbbbbb."],
            &[('b', 0x3a5a8a), ('B', 0x5a82b8), ('d', 0x2a4268)],
        ),
    );

    // ---- Fertiggüter
    add(
        "bread",
        art(
            &["...bbbbbb...", "..bBBBBBBb..", ".bBBlBBlBBb.", "bBBBBBBBBBBb", "bBBBBBBBBBBb", ".bbbbbbbbbb.", "..dddddddd.."],
            &[('b', 0xa8702a), ('B', 0xd09a48), ('l', 0xf0c878), ('d', 0x6a421a)],
        ),
    );
    add(
        "beer",
        art(
            &["..wwwwwww...", ".wFFFFFFFw..", ".wyyyyyyyww.", ".wyyyyyyyw.w", ".wyyyyyyyw.w", ".wyyyyyyyww.", ".wyyyyyyyw..", "..wwwwwww..."],
            &[('w', 0xc8c0a8), ('F', 0xf8f4e8), ('y', 0xe0a02a)],
        ),
    );
    add(
        "clothing",
        art(
            &["bb..bbbb..bb", "bBbbBBBBbbBb", ".bBBBBBBBBb.", "..bBBBBBBb..", "..bBBBBBBb..", "..bBBddBBb..", "..bBBBBBBb..", "..bbbbbbbb.."],
            &[('b', 0x7a3a2a), ('B', 0xa85a42), ('d', 0x5a2a1e)],
        ),
    );
    add(
        "tools",
        art(
            &["..sssss.....", ".sLLLLss.m..", ".ssssLLs.mm.", ".....sss.m..", "...ww.....m.", "..ww........", ".ww.....ss..", "ww.....sLLs.", "w.......ss.."],
            &[('s', 0x8a96a0), ('L', 0xc4ced6), ('w', 0x7a5632), ('m', 0x6a727c)],
        ),
    );
    add(
        "weapons",
        art(
            &["........ss.", ".......sLs..", "......sLs...", ".....sLs....", "g...sLs.....", ".g.sLs......", "..gLs.......", "..gg........", ".wg.g.......", "ww..........", "w..........."],
            &[('s', 0x8a96a0), ('L', 0xdce4ea), ('g', 0xcfa23f), ('w', 0x5a3a22)],
        ),
    );
    add(
        "armor",
        art(
            &["ss......ss", "sLs.ssss.sLs", "sLLsLLLLsLLs", ".sLLLLLLLLs.", "..sLLddLLs..", "..sLLLLLLs..", "..sLLLLLLs..", "...sLLLLs...", "....ssss...."],
            &[('s', 0x6a727c), ('L', 0xaeb8c0), ('d', 0x4a5560)],
        ),
    );

    // ---- Betriebszustände (Sprechblasen über dem Gebäude)
    let mut c = bubble(0xc83a2a, 0x601a14);
    overlay(&mut c, &["wwwwww", "w....w", "w....w", "wwwwww"], &[('w', 0xffe8d0)]);
    overlay(&mut c, &["..", "..", "ww", "ww"], &[('w', 0xffe8d0)]);
    add("status_input", c);
    let mut c = bubble(0xd8a02a, 0x6a4a10);
    overlay(&mut c, &["..kk..", ".kkkk.", ".kkkk.", "..kk..", ".kkkk.", "kkkkkk", "kkkkkk"], &[('k', 0x3a2808)]);
    add("status_workers", c);
    let mut c = bubble(0x3a6aa8, 0x1a2c4a);
    overlay(&mut c, &["..ww..", ".wwww.", "wwwwww", "..ww..", "..ww..", "wwwwww"], &[('w', 0xe0ecff)]);
    add("status_full", c);
    let mut c = bubble(0xc8702a, 0x5a2c10);
    overlay(&mut c, &["..ww..", ".wwww.", "wwwwww", "w.ww.w", "w.ww.w", "wwwwww"], &[('w', 0xffe8d0)]);
    overlay(&mut c, &["r....r", ".r..r.", "..rr..", "..rr..", ".r..r.", "r....r"], &[('r', 0xc83a2a)]);
    add("status_depot", c);
    let mut c = bubble(0x3a8a3a, 0x16401a);
    overlay(&mut c, &["..ww..", ".wwww.", "wwwwww", ".wwww.", "..ww..", "..ww.."], &[('w', 0xe0ffe0)]);
    overlay(&mut c, &["r....r", ".r..r.", "..rr..", "..rr..", ".r..r.", "r....r"], &[('r', 0xc83a2a)]);
    add("status_site", c);
    let mut c = bubble(0x6a6a72, 0x2a2a30);
    overlay(&mut c, &["ww..ww", "ww..ww", "ww..ww", "ww..ww", "ww..ww", "ww..ww"], &[('w', 0xe8e8f0)]);
    add("status_off", c);

    // ---- Stadt
    add(
        "workers",
        art(
            &["....ss....", "...ssss...", "...ssss...", "....ss....", "..cccccc..", ".ccccccccm", ".ccccccccm", "..cc..cc.m", "..cc..cc..", "..bb..bb.."],
            &[('s', 0xc99c79), ('c', 0x5d4630), ('b', 0x2c1c13), ('m', 0x8a96a0)],
        ),
    );
    add(
        "happy",
        art(
            &["...yyyyyy...", "..yyyyyyyy..", ".yykyyyykyy.", ".yyyyyyyyyy.", ".ykyyyyyykY.", ".yykkkkkkyY.", "..yyyyyyyy..", "...yyyyyy..."],
            &[('y', 0xe8c040), ('Y', 0xb8902a), ('k', 0x3a2808)],
        ),
    );
    add(
        "sad",
        art(
            &["...rrrrrr...", "..rrrrrrrr..", ".rrkrrrrkrr.", ".rrrrrrrrrr.", ".rrrkkkkrrR.", ".rrkrrrrkrR.", "..rrrrrrrr..", "...rrrrrr..."],
            &[('r', 0xc86a48), ('R', 0x8a3a24), ('k', 0x3a1a0c)],
        ),
    );
    add(
        "castle",
        art(
            &["s.s.ss.s.s", "ssssssssss", ".sLLLLLLs.", ".sLLddLLs.", ".sLLddLLs.", "ssLLLLLLss", "sLLLddLLLs", "ssssssssss"],
            &[('s', 0x6a6a72), ('L', 0xa0a0a8), ('d', 0x2a2a30)],
        ),
    );
    add(
        "gate",
        art(
            &["ssssssssss", "sLLLLLLLLs", "sLwwwwwwLs", "sLwddddwLs", "sLwwwwwwLs", "sLwddddwLs", "sLwwwwwwLs", "sLLLLLLLLs"],
            &[('s', 0x5a5a60), ('L', 0x8a8a92), ('w', 0x7a5632), ('d', 0x3a2818)],
        ),
    );
}
