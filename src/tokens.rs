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
pub enum Others {
    Space,
    NewLine,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Akshara<
    Ayogavaha,
    Aytham,
    ConsonantsMain,
    Nukta,
    Numerals,
    Om,
    PersoArabic,
    Sinhala,
    South,
    Symbols,
    VowelMain,
    VowelModern,
    VowelSignMain,
    VowelSignModern,
    VowelSignSinhala,
    VowelSignSouth,
    VowelSignVirama,
    VowelSinhala,
    VowelSouth,
    Others,
> {
    Ayogavaha(Ayogavaha),
    Aytham(Aytham),
    ConsonantsMain(ConsonantsMain),
    Nukta(Nukta),
    Numerals(Numerals),
    Om(Om),
    PersoArabic(PersoArabic),
    Sinhala(Sinhala),
    South(South),
    Symbols(Symbols),
    VowelMain(VowelMain),
    VowelModern(VowelModern),
    VowelSignMain(VowelSignMain),
    VowelSignModern(VowelSignModern),
    VowelSignSinhala(VowelSignSinhala),
    VowelSignSouth(VowelSignSouth),
    VowelSignVirama(VowelSignVirama),
    VowelSinhala(VowelSinhala),
    VowelSouth(VowelSouth),
    Others(Others),
}

// #[derive(Debug)]
// pub struct Akshara {
//     enum_name: &'static str,
//     variant: Box<dyn std::fmt::Debug>,
// }
