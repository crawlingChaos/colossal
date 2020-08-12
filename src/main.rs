mod data;
mod fortran;

use fortran::{
    achar, goto, iachar, pause, stop, Array, Array2D, Char, File, Keyboard, Rand, Word, BLANK,
};
use Label::*;

#[derive(Clone, Copy, Debug)]
pub enum Label {
    L1002,
    L1004,
    L1007,
    L1008,
    L1009,
    L1010,
    L1011,
    L1012,
    L1023,
    L1013,
    L1014,
    L1016,
    L1017,
    L1019,
    L1020,
    L1100,
    L1,
    L2,
    L74,
    L60,
    L63,
    L75,
    L77,
    L79,
    L81,
    L82,
    L83,
    L69,
    L71,
    L4,
    L7,
    L8,
    L9,
    L12,
    L10,
    L11,
    L19,
    L21,
    L22,
    L23,
    L24,
    L25,
    L26,
    L27,
    L28,
    L29,
    L30,
    L31,
    L32,
    L33,
    L34,
    L35,
    L38,
    L36,
    L37,
    L39,
    L40,
    L2000,
    L2001,
    L2003,
    L2004,
    L2005,
    L2008,
    L2012,
    L2009,
    L2010,
    L5200,
    L2011,
    L2020,
    L2021,
    L2023,
    L2025,
    L2026,
    L2027,
    L2028,
    L3000,
    L2032,
    L2034,
    L2033,
    L2035,
    L2036,
    L2037,
    L5062,
    L5333,
    L5014,
    L5017,
    L5000,
    L502,
    L5316,
    L5098,
    L5097,
    L5004,
    L5064,
    L5314,
    L9000,
    L9001,
    L9002,
    L9003,
    L9004,
    L9005,
    L9006,
    L9007,
    L9008,
    L9403,
    L5032,
    L5105,
    L5066,
    L5012,
    L5160,
    L9401,
    L5031,
    L5102,
    L5104,
    L5107,
    L5034,
    L5033,
    L5109,
    L9404,
    L9406,
    L5081,
    L5300,
    L5302,
    L5307,
    L5309,
    L5311,
    L5502,
    L5501,
    L5504,
    L5505,
    L5506,
}

fn main() {
    let mut rand = Rand::new();
    let mut i = 0;
    let mut ikind = 0;
    let mut jkind = 0;
    let mut lkind = 0;
    let mut kk = 0;
    let mut idwarf = 0;
    let mut ifirst = 0;
    let mut iwest = 0;
    let mut ilong = 0;
    let mut idetal = 0;
    let mut yea = 0;
    let mut l = 0;
    let mut loc = 0;
    let mut j = 0;
    let mut attack = 0;
    let mut dtot = 0;
    let mut stick = 0;
    let mut k = 0;
    let mut lold = 0;
    let mut ll = 0;
    let mut jspk = 0;
    let mut jverb = 0;
    let mut ltrubl = 0;
    let mut idark = 0;
    let mut twowds = 0;
    let mut jobj = 0;
    let mut itemp = 0;
    let mut iid = 0;
    let mut ran = 0.0;
    let mut a = Word::new();
    let mut b = Word::new();
    let mut wd2 = Word::new();
    let mut atab = Array::<Word>::new(1000);
    let mut lline_text = Array2D::<Char>::new(1000, 100);
    let mut iobj = Array::<i32>::new(300);
    let mut ichain = Array::<i32>::new(100);
    let mut iplace = Array::<i32>::new(100);
    let mut lline_cont = Array::<i32>::new(1000);
    let mut ifixed = Array::<i32>::new(100);
    let mut cond = Array::<i32>::new(300);
    let mut prop = Array::<i32>::new(100);
    let mut abb = Array::<i32>::new(300);
    let mut lline_len = Array::<i32>::new(1000);
    let mut ltext = Array::<i32>::new(300);
    let mut stext = Array::<i32>::new(300);
    let mut key = Array::<i32>::new(300);
    let mut default = Array::<i32>::new(300);
    let mut travel = Array::<i32>::new(1000);
    let mut tk = Array::<i32>::new(25);
    let mut ktab = Array::<i32>::new(1000);
    let mut btext = Array::<i32>::new(200);
    let mut dseen = Array::<i32>::new(10);
    let mut dloc = Array::<i32>::new(10);
    let mut odloc = Array::<i32>::new(10);
    let mut dtrav = Array::<i32>::new(20);
    let mut rtext = Array::<i32>::new(100);
    let mut jspkt = Array::<i32>::new(100);
    let mut iplt = Array::<i32>::new(100);
    let mut ifixt = Array::<i32>::new(100);
    let setup = 1;
    let keys = 1;
    let lamp = 2;
    let grate = 3;
    let rod = 5;
    let bird = 7;
    let nugget = 10;
    let snake = 11;
    let food = 19;
    let water = 20;
    let axe = 21;
    jspkt.copy_from_slice(&[24, 29, 0, 31, 0, 31, 38, 38, 42, 42, 43, 46, 77, 71, 73, 75]);
    iplt.copy_from_slice(&[
        3, 3, 8, 10, 11, 14, 13, 9, 15, 18, 19, 17, 27, 28, 29, 30, 0, 0, 3, 3,
    ]);
    ifixt.copy_from_slice(&[0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    dtrav.copy_from_slice(&[36, 28, 19, 30, 62, 60, 41, 27, 17, 15, 19, 28, 36, 300, 300]);
    for x in 1..=300 {
        i = x;
        stext.set(i, 0);
        if i <= 200 {
            btext.set(i, 0);
        }
        if i <= 100 {
            rtext.set(i, 0);
        }
        ltext.set(i, 0);
    }
    i = 1;
    let mut file = File::open();
    // The label variable and loops are needed to emulate goto statements
    let mut label = L1002;
    loop {
        label = 'goto: loop {
            match label {
                L1002 => {
                    file.read().read_i7(&mut ikind);
                    if let Some(label) = goto(
                        &[L1100, L1004, L1004, L1013, L1020, L1004, L1004],
                        ikind + 1,
                    ) {
                        break label;
                    }
                    break L1004;
                }
                L1004 => {
                    let mut line = file.read();
                    line.read_i7(&mut jkind);
                    for x in 1..=100 {
                        j = x;
                        line.read_char(lline_text.get_mut(i, j));
                    }
                    if jkind == -1 {
                        break L1002;
                    }
                    for x in 1..=100 {
                        k = x;
                        kk = k;
                        if *lline_text.get(i, 101 - k) != BLANK {
                            break 'goto L1007;
                        }
                    }
                    stop();
                }
                L1007 => {
                    lline_len.set(i, 100 - kk + 1);
                    lline_cont.set(i, 0);
                    if ikind == 6 {
                        break L1023;
                    }
                    if ikind == 5 {
                        break L1011;
                    }
                    if ikind == 1 {
                        break L1008;
                    }
                    if *stext.get(jkind) != 0 {
                        break L1009;
                    }
                    stext.set(jkind, i);
                    break L1010;
                }
                L1008 => {
                    if *ltext.get(jkind) != 0 {
                        break L1009;
                    }
                    ltext.set(jkind, i);
                    break L1010;
                }
                L1009 => {
                    lline_cont.set(i - 1, i);
                    break L1010;
                }
                L1010 => {
                    i += 1;
                    if i != 1000 {
                        break L1004;
                    }
                    pause("TOO MANY LINES");
                    break L1011;
                }
                L1011 => {
                    if jkind < 200 {
                        break L1012;
                    }
                    if *btext.get(jkind - 100) != 0 {
                        break L1009;
                    }
                    btext.set(jkind - 100, i);
                    btext.set(jkind - 200, i);
                    break L1010;
                }
                L1012 => {
                    if *btext.get(jkind) != 0 {
                        break L1009;
                    }
                    btext.set(jkind, i);
                    break L1010;
                }
                L1023 => {
                    if *rtext.get(jkind) != 0 {
                        break L1009;
                    }
                    rtext.set(jkind, i);
                    break L1010;
                }
                L1013 => {
                    i = 1;
                    break L1014;
                }
                L1014 => {
                    let mut line = file.read();
                    line.read_i7(&mut jkind);
                    line.read_i7(&mut lkind);
                    for x in 1..=10 {
                        l = x;
                        line.read_i7(tk.get_mut(l));
                    }
                    if jkind == -1 {
                        break L1002;
                    }
                    if *key.get(jkind) != 0 {
                        break L1016;
                    }
                    key.set(jkind, i);
                    break L1017;
                }
                L1016 => {
                    travel.set(i - 1, -travel.get(i - 1));
                    break L1017;
                }
                L1017 => {
                    for x in 1..=10 {
                        l = x;
                        if *tk.get(l) == 0 {
                            break 'goto L1019;
                        }
                        travel.set(i, lkind * 1024 + tk.get(l));
                        i += 1;
                        if i == 1000 {
                            stop();
                        }
                    }
                    break L1019;
                }
                L1019 => {
                    travel.set(i - 1, -travel.get(i - 1));
                    break L1014;
                }
                L1020 => {
                    for iu in 1..=1000 {
                        let mut line = file.read();
                        line.read_i7(ktab.get_mut(iu));
                        line.read_word(atab.get_mut(iu));
                        if *ktab.get(iu) == -1 {
                            break 'goto L1002;
                        }
                    }
                    pause("TOO MANY WORDS");
                    break L1100;
                }
                L1100 => {
                    for x in 1..=100 {
                        i = x;
                        iplace.set(i, *iplt.get(i));
                        ifixed.set(i, *ifixt.get(i));
                        ichain.set(i, 0);
                    }
                    for x in 1..=300 {
                        i = x;
                        cond.set(i, 0);
                        abb.set(i, 0);
                        iobj.set(i, 0);
                    }
                    for x in 1..=10 {
                        i = x;
                        cond.set(i, 1);
                    }
                    cond.set(16, 2);
                    cond.set(20, 2);
                    cond.set(21, 2);
                    cond.set(22, 2);
                    cond.set(23, 2);
                    cond.set(24, 2);
                    cond.set(25, 2);
                    cond.set(26, 2);
                    cond.set(31, 2);
                    cond.set(32, 2);
                    cond.set(79, 2);
                    for x in 1..=100 {
                        i = x;
                        let mut ktem = *iplace.get(i);
                        if ktem != 0 {
                            if *iobj.get(ktem) != 0 {
                                ktem = *iobj.get(ktem);
                                while *ichain.get(ktem) != 0 {
                                    ktem = *ichain.get(ktem);
                                }
                                ichain.set(ktem, i);
                            } else {
                                iobj.set(ktem, i);
                            }
                        }
                    }
                    idwarf = 0;
                    ifirst = 1;
                    iwest = 0;
                    ilong = 1;
                    idetal = 0;
                    pause("INIT DONE");
                    break L1;
                }
                L1 => {
                    yes(
                        65,
                        1,
                        0,
                        &mut yea,
                        &rtext,
                        &lline_text,
                        &lline_len,
                        &lline_cont,
                    );
                    l = 1;
                    loc = 1;
                    break L2;
                }
                L2 => {
                    for x in 1..=3 {
                        i = x;
                        if *odloc.get(i) != l || *dseen.get(i) == 0 {
                            continue;
                        }
                        l = loc;
                        speak(2, &rtext, &lline_text, &lline_len, &lline_cont);
                        break 'goto L74;
                    }
                    break L74;
                }
                L74 => {
                    loc = l;
                    if idwarf != 0 {
                        break L60;
                    }
                    if loc == 15 {
                        idwarf = 1;
                    }
                    break L71;
                }
                L60 => {
                    if idwarf != 1 {
                        break L63;
                    }
                    if rand.gen() > 0.05 {
                        break L71;
                    }
                    idwarf = 2;
                    for x in 1..=3 {
                        i = x;
                        dloc.set(i, 0);
                        odloc.set(i, 0);
                        dseen.set(i, 0);
                    }
                    speak(3, &rtext, &lline_text, &lline_len, &lline_cont);
                    ichain.set(axe, *iobj.get(loc));
                    iobj.set(loc, axe);
                    iplace.set(axe, loc);
                    break L71;
                }
                L63 => {
                    idwarf += 1;
                    attack = 0;
                    dtot = 0;
                    stick = 0;
                    for x in 1..=3 {
                        i = x;
                        if 2 * i + idwarf < 8 {
                            continue;
                        }
                        if 2 * i + idwarf > 23 && *dseen.get(i) == 0 {
                            continue;
                        }
                        odloc.set(i, *dloc.get(i));
                        if !(*dseen.get(i) != 0 && loc > 14) {
                            dloc.set(i, *dtrav.get(i * 2 + idwarf - 8));
                            dseen.set(i, 0);
                            if *dloc.get(i) != loc && *odloc.get(i) != loc {
                                continue;
                            }
                        }
                        dseen.set(i, 1);
                        dloc.set(i, loc);
                        dtot += 1;
                        if *odloc.get(i) != *dloc.get(i) {
                            continue;
                        }
                        attack += 1;
                        if rand.gen() < 0.1 {
                            stick += 1;
                        }
                    }
                    if dtot == 0 {
                        break L71;
                    }
                    if dtot == 1 {
                        break L75;
                    }
                    println!(
                        " THERE ARE {:>2} THREATENING LITTLE DWARVES IN THE ROOM WITH YOU.",
                        dtot
                    );
                    break L77;
                }
                L75 => {
                    speak(4, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L77;
                }
                L77 => {
                    if attack == 0 {
                        break L71;
                    }
                    if attack == 1 {
                        break L79;
                    }
                    println!(" {:>2} OF THEM THROW KNIVES AT YOU!", attack);
                    break L81;
                }
                L79 => {
                    speak(5, &rtext, &lline_text, &lline_len, &lline_cont);
                    speak(52 + stick, &rtext, &lline_text, &lline_len, &lline_cont);
                    if let Some(label) = goto(&[L71, L83], stick + 1) {
                        break label;
                    }
                    break L81;
                }
                L81 => {
                    if stick == 0 {
                        break L69;
                    }
                    if stick == 1 {
                        break L82;
                    }
                    println!(" {:>2} OF THEM GET YOU.", stick);
                    break L83;
                }
                L82 => {
                    speak(6, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L83;
                }
                L83 => {
                    pause("GAMES OVER");
                    break L71;
                }
                L69 => {
                    speak(7, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L71;
                }
                L71 => {
                    kk = *stext.get(l);
                    if *abb.get(l) == 0 || kk == 0 {
                        kk = *ltext.get(l);
                    }
                    if kk == 0 {
                        break L7;
                    }
                    break L4;
                }
                L4 => {
                    for jj in 1..=*lline_len.get(kk) {
                        print!("{}", lline_text.get(kk, jj));
                    }
                    kk += 1;
                    if *lline_cont.get(kk - 1) != 0 {
                        break L4;
                    }
                    println!();
                    break L7;
                }
                L7 => {
                    if *cond.get(l) == 2 {
                        break L8;
                    }
                    if loc == 33 && rand.gen() < 0.25 {
                        speak(8, &rtext, &lline_text, &lline_len, &lline_cont);
                    }
                    j = l;
                    break L2000;
                }
                L8 => {
                    kk = *key.get(loc);
                    if kk == 0 {
                        break L19;
                    }
                    if k == 57 {
                        break L32;
                    }
                    if k == 67 {
                        break L40;
                    }
                    if k == 8 {
                        break L12;
                    }
                    lold = l;
                    break L9;
                }
                L9 => {
                    ll = *travel.get(kk);
                    if ll < 0 {
                        ll = -ll;
                    }
                    if 1 == ll % 1024 {
                        break L10;
                    }
                    if k == ll % 1024 {
                        break L10;
                    }
                    if *travel.get(kk) < 0 {
                        break L11;
                    }
                    kk += 1;
                    break L9;
                }
                L12 => {
                    let temp = lold;
                    lold = l;
                    l = temp;
                    break L21;
                }
                L10 => {
                    l = ll / 1024;
                    break L21;
                }
                L11 => {
                    jspk = 12;
                    if k >= 43 && k <= 46 {
                        jspk = 9;
                    }
                    if k == 29 || k == 30 {
                        jspk = 9;
                    }
                    if k == 7 || k == 8 || k == 36 || k == 37 || k == 68 {
                        jspk = 10;
                    }
                    if k == 11 || k == 19 {
                        jspk = 11;
                    }
                    if jverb == 1 {
                        jspk = 59;
                    }
                    if k == 48 {
                        jspk = 42;
                    }
                    if k == 17 {
                        jspk = 80;
                    }
                    speak(jspk, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2;
                }
                L19 => {
                    speak(13, &rtext, &lline_text, &lline_len, &lline_cont);
                    l = loc;
                    if ifirst == 0 {
                        speak(14, &rtext, &lline_text, &lline_len, &lline_cont);
                    }
                    break L21;
                }
                L21 => {
                    if l < 300 {
                        break L2;
                    }
                    let il = l - 300 + 1;
                    if let Some(label) = goto(
                        &[
                            L22, L23, L24, L25, L26, L31, L27, L28, L29, L30, L33, L34, L36, L37,
                        ],
                        il,
                    ) {
                        break label;
                    }
                    break L2;
                }
                L22 => {
                    l = 6;
                    if rand.gen() > 0.5 {
                        l = 5;
                    }
                    break L2;
                }
                L23 => {
                    l = 23;
                    if *prop.get(grate) != 0 {
                        l = 9;
                    }
                    break L2;
                }
                L24 => {
                    l = 9;
                    if *prop.get(grate) != 0 {
                        l = 8;
                    }
                    break L2;
                }
                L25 => {
                    l = 20;
                    if *iplace.get(nugget) != -1 {
                        l = 15;
                    }
                    break L2;
                }
                L26 => {
                    l = 22;
                    if *iplace.get(nugget) != -1 {
                        l = 14;
                    }
                    break L2;
                }
                L27 => {
                    l = 27;
                    if *prop.get(12) == 0 {
                        l = 31;
                    }
                    break L2;
                }
                L28 => {
                    l = 28;
                    if *prop.get(snake) == 0 {
                        l = 32;
                    }
                    break L2;
                }
                L29 => {
                    l = 29;
                    if *prop.get(snake) == 0 {
                        l = 32;
                    }
                    break L2;
                }
                L30 => {
                    l = 30;
                    if *prop.get(snake) == 0 {
                        l = 32;
                    }
                    break L2;
                }
                L31 => {
                    pause("GAME IS OVER");
                    break L1100;
                }
                L32 => {
                    if idetal < 3 {
                        speak(15, &rtext, &lline_text, &lline_len, &lline_cont);
                    }
                    idetal += 1;
                    l = loc;
                    abb.set(l, 0);
                    break L2;
                }
                L33 => {
                    l = 8;
                    if *prop.get(grate) == 0 {
                        l = 9;
                    }
                    break L2;
                }
                L34 => {
                    if rand.gen() > 0.2 {
                        break L35;
                    }
                    l = 68;
                    break L2;
                }
                L35 => {
                    l = 65;
                    break L38;
                }
                L38 => {
                    speak(56, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2;
                }
                L36 => {
                    if rand.gen() > 0.2 {
                        break L35;
                    }
                    l = 39;
                    if rand.gen() > 0.5 {
                        l = 70;
                    }
                    break L2;
                }
                L37 => {
                    l = 66;
                    if rand.gen() > 0.4 {
                        break L38;
                    }
                    l = 71;
                    if rand.gen() > 0.25 {
                        l = 72;
                    }
                    break L2;
                }
                L39 => {
                    l = 66;
                    if rand.gen() > 0.2 {
                        break L38;
                    }
                    l = 77;
                    break L2;
                }
                L40 => {
                    if loc < 8 {
                        speak(57, &rtext, &lline_text, &lline_len, &lline_cont);
                    }
                    if loc >= 8 {
                        speak(58, &rtext, &lline_text, &lline_len, &lline_cont);
                    }
                    l = loc;
                    break L2;
                }
                L2000 => {
                    ltrubl = 0;
                    loc = j;
                    abb.set(j, (*abb.get(j) + 1) % 5);
                    idark = 0;
                    if *cond.get(j) % 2 == 1 {
                        break L2003;
                    }
                    if *iplace.get(2) != j && *iplace.get(2) != -1 {
                        break L2001;
                    }
                    if *prop.get(2) == 1 {
                        break L2003;
                    }
                    break L2001;
                }
                L2001 => {
                    speak(16, &rtext, &lline_text, &lline_len, &lline_cont);
                    idark = 1;
                    break L2003;
                }
                L2003 => {
                    i = *iobj.get(j);
                    break L2004;
                }
                L2004 => {
                    if i == 0 {
                        break L2011;
                    }
                    if (i == 6 || i == 9) && *iplace.get(10) == -1 {
                        break L2008;
                    }
                    let mut ilk = i;
                    if *prop.get(i) != 0 {
                        ilk = i + 100;
                    }
                    kk = *btext.get(ilk);
                    if kk == 0 {
                        break L2008;
                    }
                    break L2005;
                }
                L2005 => {
                    for jj in 1..=*lline_len.get(kk) {
                        print!("{}", lline_text.get(kk, jj));
                    }
                    kk += 1;
                    if *lline_cont.get(kk - 1) != 0 {
                        break L2005;
                    }
                    println!();
                    break L2008;
                }
                L2008 => {
                    i = *ichain.get(i);
                    break L2004;
                }
                L2012 => {
                    a = wd2;
                    b = Word::new();
                    twowds = 0;
                    break L2021;
                }
                L2009 => {
                    k = 54;
                    break L2010;
                }
                L2010 => {
                    jspk = k;
                    break L5200;
                }
                L5200 => {
                    speak(jspk, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L2011 => {
                    jverb = 0;
                    jobj = 0;
                    twowds = 0;
                    break L2020;
                }
                L2020 => {
                    getin(&mut twowds, &mut a, &mut wd2, &mut b);
                    k = 70;
                    if a.eq("ENTER") && (wd2.eq("STREA") || wd2.eq("WATER")) {
                        break L2010;
                    }
                    if a.eq("ENTER") && twowds != 0 {
                        break L2012;
                    }
                    break L2021;
                }
                L2021 => {
                    if !a.eq("WEST") {
                        break L2023;
                    }
                    iwest += 1;
                    if iwest != 10 {
                        break L2023;
                    }
                    speak(17, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2023;
                }
                L2023 => {
                    for x in 1..=1000 {
                        i = x;
                        if *ktab.get(i) == -1 {
                            break 'goto L3000;
                        }
                        if *atab.get(i) == a {
                            break 'goto L2025;
                        }
                    }
                    pause("ERROR 6");
                    break L2025;
                }
                L2025 => {
                    k = *ktab.get(i) % 1000;
                    let kq = *ktab.get(i) / 1000 + 1;
                    if let Some(label) = goto(&[L5014, L5000, L2026, L2010], kq) {
                        break label;
                    }
                    pause("NO NO");
                    break L2026;
                }
                L2026 => {
                    jverb = k;
                    jspk = *jspkt.get(jverb);
                    if twowds != 0 {
                        break L2028;
                    }
                    if jobj == 0 {
                        break L2036;
                    }
                    break L2027;
                }
                L2027 => {
                    if let Some(label) = goto(
                        &[
                            L9000, L5066, L3000, L5031, L2009, L5031, L9404, L9406, L5081, L5200,
                            L5200, L5300, L5506, L5502, L5504, L5505,
                        ],
                        jverb,
                    ) {
                        break label;
                    }
                    pause("ERROR 5");
                    break L2028;
                }
                L2028 => {
                    a = wd2;
                    b = Word::new();
                    twowds = 0;
                    break L2023;
                }
                L3000 => {
                    jspk = 60;
                    if rand.gen() > 0.8 {
                        jspk = 61;
                    }
                    if rand.gen() > 0.8 {
                        jspk = 13;
                    }
                    speak(jspk, &rtext, &lline_text, &lline_len, &lline_cont);
                    ltrubl += 1;
                    if ltrubl != 3 {
                        break L2020;
                    }
                    if j != 13 || *iplace.get(7) != 13 || *iplace.get(5) != -1 {
                        break L2032;
                    }
                    yes(
                        18,
                        19,
                        54,
                        &mut yea,
                        &rtext,
                        &lline_text,
                        &lline_len,
                        &lline_cont,
                    );
                    break L2033;
                }
                L2032 => {
                    if j != 19 || *prop.get(11) != 0 || *iplace.get(7) == -1 {
                        break L2034;
                    }
                    yes(
                        20,
                        21,
                        54,
                        &mut yea,
                        &rtext,
                        &lline_text,
                        &lline_len,
                        &lline_cont,
                    );
                    break L2033;
                }
                L2034 => {
                    if j != 8 || *prop.get(grate) != 0 {
                        break L2035;
                    }
                    yes(
                        62,
                        63,
                        54,
                        &mut yea,
                        &rtext,
                        &lline_text,
                        &lline_len,
                        &lline_cont,
                    );
                    break L2033;
                }
                L2033 => {
                    if yea == 0 {
                        break L2011;
                    }
                    break L2020;
                }
                L2035 => {
                    if *iplace.get(5) != j && *iplace.get(5) != -1 {
                        break L2020;
                    }
                    if jobj != 5 {
                        break L2020;
                    }
                    speak(22, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2020;
                }
                L2036 => {
                    if let Some(label) = goto(
                        &[
                            L2037, L5062, L5062, L9403, L2009, L9403, L9404, L9406, L5062, L5062,
                            L5200, L5300, L5062, L5062, L5062, L5062,
                        ],
                        jverb,
                    ) {
                        break label;
                    }
                    pause("OOPS");
                    break L2037;
                }
                L2037 => {
                    if *iobj.get(j) == 0 || *ichain.get(*iobj.get(j)) != 0 {
                        break L5062;
                    }
                    for x in 1..=3 {
                        i = x;
                        if *dseen.get(i) != 0 {
                            break 'goto L5062;
                        }
                    }
                    jobj = *iobj.get(j);
                    break L2027;
                }
                L5062 => {
                    if !b.eq(" ") {
                        break L5333;
                    }
                    println!("  {} WHAT?", a);
                    break L2020;
                }
                L5333 => {
                    println!(" {}{} WHAT?", a, b);
                    break L2020;
                }
                L5014 => {
                    if idark == 0 {
                        break L8;
                    }
                    if rand.gen() > 0.25 {
                        break L8;
                    }
                    break L5017;
                }
                L5017 => {
                    speak(23, &rtext, &lline_text, &lline_len, &lline_cont);
                    pause("GAME IS OVER");
                    break L2011;
                }
                L5000 => {
                    jobj = k;
                    if twowds != 0 {
                        break L2028;
                    }
                    if j == *iplace.get(k) || *iplace.get(k) == -1 {
                        break L5004;
                    }
                    if k != grate {
                        break L502;
                    }
                    if j == 1 || j == 4 || j == 7 {
                        break L5098;
                    }
                    if j > 9 && j < 15 {
                        break L5097;
                    }
                    break L502;
                }
                L502 => {
                    if !b.eq(" ") {
                        break L5316;
                    }
                    println!(" I SEE NO {} HERE.", a);
                    break L2011;
                }
                L5316 => {
                    println!(" I SEE NO {}{} HERE.", a, b);
                    break L2011;
                }
                L5098 => {
                    k = 49;
                    break L5014;
                }
                L5097 => {
                    k = 50;
                    break L5014;
                }
                L5004 => {
                    jobj = k;
                    if jverb != 0 {
                        break L2027;
                    }
                    break L5064;
                }
                L5064 => {
                    if !b.eq(" ") {
                        break L5314;
                    }
                    println!(" WHAT DO YOU WANT TO DO WITH THE {}?", a);
                    break L2020;
                }
                L5314 => {
                    println!(" WHAT DO YOU WANT TO DO WITH THE {}{}?", a, b);
                    break L2020;
                }
                L9000 => {
                    if jobj == 18 {
                        break L2009;
                    }
                    if *iplace.get(jobj) != j {
                        break L5200;
                    }
                    break L9001;
                }
                L9001 => {
                    if *ifixed.get(jobj) == 0 {
                        break L9002;
                    }
                    speak(25, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L9002 => {
                    if jobj != bird {
                        break L9004;
                    }
                    if *iplace.get(rod) != -1 {
                        break L9003;
                    }
                    speak(26, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L9003 => {
                    if *iplace.get(4) == -1 || *iplace.get(4) == j {
                        break L9004;
                    }
                    speak(27, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L9004 => {
                    iplace.set(jobj, -1);
                    break L9005;
                }
                L9005 => {
                    if *iobj.get(j) != jobj {
                        break L9006;
                    }
                    iobj.set(j, *ichain.get(jobj));
                    break L2009;
                }
                L9006 => {
                    itemp = *iobj.get(j);
                    break L9007;
                }
                L9007 => {
                    if *ichain.get(itemp) == jobj {
                        break L9008;
                    }
                    itemp = *ichain.get(itemp);
                    break L9007;
                }
                L9008 => {
                    ichain.set(itemp, *ichain.get(jobj));
                    break L2009;
                }
                L9403 => {
                    if j == 8 || j == 9 {
                        break L5105;
                    }
                    break L5032;
                }
                L5032 => {
                    speak(28, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5105 => {
                    jobj = grate;
                    break L2027;
                }
                L5066 => {
                    if jobj == 18 {
                        break L2009;
                    }
                    if *iplace.get(jobj) != -1 {
                        break L5200;
                    }
                    break L5012;
                }
                L5012 => {
                    if jobj != bird || j != 19 || *prop.get(11) == 1 {
                        break L9401;
                    }
                    speak(30, &rtext, &lline_text, &lline_len, &lline_cont);
                    prop.set(11, 1);
                    break L5160;
                }
                L5160 => {
                    ichain.set(jobj, *iobj.get(j));
                    iobj.set(j, jobj);
                    iplace.set(jobj, j);
                    break L2011;
                }
                L9401 => {
                    speak(54, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L5160;
                }
                L5031 => {
                    if *iplace.get(keys) != -1 && *iplace.get(keys) != j {
                        break L5200;
                    }
                    if jobj != 4 {
                        break L5102;
                    }
                    speak(32, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5102 => {
                    if jobj != keys {
                        break L5104;
                    }
                    speak(55, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5104 => {
                    if jobj == grate {
                        break L5107;
                    }
                    speak(33, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5107 => {
                    if jverb == 4 {
                        break L5033;
                    }
                    if *prop.get(grate) != 0 {
                        break L5034;
                    }
                    speak(34, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5034 => {
                    speak(35, &rtext, &lline_text, &lline_len, &lline_cont);
                    prop.set(grate, 0);
                    prop.set(8, 0);
                    break L2011;
                }
                L5033 => {
                    if *prop.get(grate) == 0 {
                        break L5109;
                    }
                    speak(36, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5109 => {
                    speak(37, &rtext, &lline_text, &lline_len, &lline_cont);
                    prop.set(grate, 1);
                    prop.set(8, 1);
                    break L2011;
                }
                L9404 => {
                    if *iplace.get(2) != j && *iplace.get(2) != -1 {
                        break L5200;
                    }
                    prop.set(2, 1);
                    idark = 0;
                    speak(39, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L9406 => {
                    if *iplace.get(2) != j && *iplace.get(2) != -1 {
                        break L5200;
                    }
                    prop.set(2, 0);
                    speak(40, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5081 => {
                    if jobj != 12 {
                        break L5200;
                    }
                    prop.set(12, 1);
                    break L2003;
                }
                L5300 => {
                    for id in 1..=3 {
                        iid = id;
                        if *dseen.get(id) != 0 {
                            break 'goto L5307;
                        }
                    }
                    if jobj == 0 {
                        break L5062;
                    }
                    if jobj == snake {
                        break L5200;
                    }
                    if jobj == bird {
                        break L5302;
                    }
                    speak(44, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L2011;
                }
                L5302 => {
                    speak(45, &rtext, &lline_text, &lline_len, &lline_cont);
                    iplace.set(jobj, 300);
                    break L9005;
                }
                L5307 => {
                    if rand.gen() > 0.4 {
                        break L5309;
                    }
                    dseen.set(iid, 0);
                    odloc.set(iid, 0);
                    dloc.set(iid, 0);
                    speak(47, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L5311;
                }
                L5309 => {
                    speak(48, &rtext, &lline_text, &lline_len, &lline_cont);
                    break L5311;
                }
                L5311 => {
                    k = 21;
                    break L5014;
                }
                L5502 => {
                    if (*iplace.get(food) != j && *iplace.get(food) != -1)
                        || *prop.get(food) != 0
                        || jobj != food
                    {
                        break L5200;
                    }
                    prop.set(food, 1);
                    break L5501;
                }
                L5501 => {
                    jspk = 72;
                    break L5200;
                }
                L5504 => {
                    if (*iplace.get(water) != j && *iplace.get(water) != -1)
                        || *prop.get(water) != 0
                        || jobj != water
                    {
                        break L5200;
                    }
                    prop.set(water, 1);
                    jspk = 74;
                    break L5200;
                }
                L5505 => {
                    if jobj != lamp {
                        jspk = 76;
                    }
                    break L5200;
                }
                L5506 => {
                    if jobj != water {
                        jspk = 78;
                    }
                    prop.set(water, 1);
                    break L5200;
                }
            }
        }
    }
}

fn yes(
    x: i32,
    y: i32,
    z: i32,
    yea: &mut i32,
    rtext: &Array<i32>,
    lline_text: &Array2D<Char>,
    lline_len: &Array<i32>,
    lline_cont: &Array<i32>,
) {
    let mut junk = 0;
    let mut ia1 = Word::new();
    let mut ib1 = Word::new();
    let mut junk2 = Word::new();
    speak(x, rtext, lline_text, lline_len, lline_cont);
    getin(&mut junk, &mut ia1, &mut junk2, &mut ib1);
    if ia1.eq("NO") || ia1.eq("N") {
        *yea = 0;
        if z != 0 {
            speak(z, rtext, lline_text, lline_len, lline_cont);
        }
    } else {
        *yea = 1;
        if y != 0 {
            speak(y, rtext, lline_text, lline_len, lline_cont);
        }
    }
}

fn speak(
    it: i32,
    rtext: &Array<i32>,
    lline_text: &Array2D<Char>,
    lline_len: &Array<i32>,
    lline_cont: &Array<i32>,
) {
    let mut kkt = *rtext.get(it);
    if kkt == 0 {
        return;
    }
    loop {
        for jjt in 1..=*lline_len.get(kkt) {
            print!("{}", lline_text.get(kkt, jjt));
        }
        kkt += 1;
        if *lline_cont.get(kkt - 1) == 0 {
            break;
        }
    }
    println!();
}

fn getin(twow: &mut i32, b: &mut Word, c: &mut Word, d: &mut Word) {
    let mut a = Array::<Word>::new(5);
    let mut keyboard = Keyboard::open();
    let mut line = keyboard.read();
    for i in 1..=4 {
        line.read_word(a.get_mut(i));
    }
    *twow = 0;
    let mut s = 0;
    for j in 1..=4 {
        for k in 1..=5 {
            *a.get_mut(j).get_mut(k) = upcase(*a.get(j).get(k));
        }
    }
    *b = *a.get(1);
    'j: for j in 1..=4 {
        for k in 1..=5 {
            if *a.get(j).get(k) == BLANK {
                if s != 1 {
                    s = 1;
                    if j == 1 {
                        for l in k..=5 {
                            b.set(l, BLANK);
                        }
                    }
                }
            } else {
                if s != 0 {
                    *twow = 1;
                    for x in 1..=6 - k {
                        c.set(x, *a.get(j).get(k + x - 1));
                    }
                    if k != 1 {
                        for x in 6 - k + 1..=5 {
                            c.set(x, *a.get(j + 1).get(k - 1));
                        }
                    }
                    break 'j;
                }
            }
        }
    }
    *d = *a.get(2);
}

fn upcase(ch: Char) -> Char {
    let mut code = iachar(ch);
    if code >= 97 && code <= 122 {
        code -= 32;
    }
    achar(code)
}
