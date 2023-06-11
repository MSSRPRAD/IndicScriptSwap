// use std::collections::HashMap;
// use std::fmt;

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum Vowels {
//     a,
//     ā,
//     i,
//     ī,
//     u,
//     ū,
//     ṛ,
//     ṝ,
//     ḷ,
//     ḹ,
//     è,
//     e,
//     ai,
//     ò,
//     o,
//     au,
// }
// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum VowelMarks {
//     a,
//     ā,
//     i,
//     ī,
//     u,
//     ū,
//     ṛ,
//     ṝ,
//     ḷ,
//     ḹ,
//     è,
//     e,
//     ai,
//     ò,
//     o,
//     au,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum Yogavāhas {
//     ṃ,
//     ḥ,
//     candrabindu,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum Virāma {
//     virāma,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum Accents {
//     udātta,
//     anudātta,
//     svarita,
// }

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum PersoArabic {
    k0,
    K0,
    g0,
    j0,
    q0,
    Q0,
    P0,
    Y0,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Sinhala {
    n_g,
    n_j,
    n_q,
    n_d,
    m_b,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum South {
    L,
    L0,
    r2,
    n2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Ayogavaha {
    C,
    M,
    H,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Aytham {
    K,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Symbols {
    A,
    D,
    DD,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Om {
    oM,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Nukta {
    Q,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelMain {
    a,
    A,
    i,
    I,
    u,
    U,
    f,
    F,
    x,
    X,
    e,
    E,
    o,
    O,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelModern {
    e2,
    o2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSinhala {
    e4,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSouth {
    e1,
    o1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSignMain {
    A,
    i,
    I,
    u,
    U,
    f,
    F,
    x,
    X,
    e,
    E,
    o,
    O,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSignModern {
    e2,
    o2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSignSinhala {
    e4,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSignSouth {
    e1,
    o1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Numerals {
    zero,
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Unknown {
    unknown,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum VowelSignVirama {
    x,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConsonantsMain {
    k,
    K,
    g,
    G,
    N,
    c,
    C,
    j,
    J,
    Y,
    w,
    W,
    q,
    Q,
    R,
    t,
    T,
    d,
    D,
    n,
    p,
    P,
    b,
    B,
    m,
    y,
    r,
    l,
    v,
    S,
    z,
    s,
    h,
}

// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq)]
// pub enum Symbols {
//     zero,
//     one,
//     two,
//     three,
//     four,
//     five,
//     six,
//     seven,
//     eight,
//     nine,
//     oṃ,
//     avagraha,
//     daṇḍa,
//     dvidaṇḍa,
//     whitespace,
//     newline,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Token<Vowels, VowelMarks, Accents, Consonants, Yogavāhas, Symbols, Virāma> {
//     Vowel(Vowels),
//     VowelMark(VowelMarks),
//     Accent(Accents),
//     Consonant(Consonants),
//     Yogavāha(Yogavāhas),
//     Symbol(Symbols),
//     Virāmam(Virāma),
//     Unk(char),
// }
