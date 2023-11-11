#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Sinhala {
    n_g,
    n_j,
    n_q,
    n_d,
    m_b,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum South {
    L,
    L0,
    r2,
    n2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Ayogavaha {
    C,
    M,
    H,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Aytham {
    K,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Symbols {
    A,
    D,
    DD,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Om {
    oM,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Nukta {
    Q,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelModern {
    e2,
    o2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSinhala {
    e4,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSouth {
    e1,
    o1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSignModern {
    e2,
    o2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSignSinhala {
    e4,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSignSouth {
    e1,
    o1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Unknown {
    unknown,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Others {
    Space,
    NewLine,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum VowelSignVirama {
    x,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Akshara<
    String,
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
    Unknown(String),
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
