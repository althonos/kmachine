/// A Kappa input file.
pub struct File {
    declarations: Vec<Declaration>,
}

/// Any declaration in a Kappa input file.
pub enum Declaration {
    Rule(Rule),
    Var(Variable),
    Sig(Signature),
    Token(Token),
    Init(Initial),
    Plot(Plot),
    Obs(Observable),
    // Inter(Intervention),
    // Conf(Configuration),
}

// --- Section 2.1: Names and labels -----------------------------------------

/// A name for agents, sites, states and variables.
pub struct Name(String);

/// A label for rules and observables.
pub struct Label(String);

/// A binding stub, used in either patterns or declarations: `b.A`
pub struct Stub {
    site: Name,
    agent: Name,
}

// --- Section 2.2: Pattern expressions --------------------------------------

/// A pattern: `A(b[a.B], c[1], s{s1}), C(a[1])`
pub struct Pattern {
    agents: Vec<PatAgent>,
}

/// An agent in a pattern: `C(a[1])`.
pub struct PatAgent {
    name: Name,
    sites: Vec<PatSite>,
}

/// A site in a pattern.
pub enum PatSite {
    Counter(PatCounter),
    Link(PatLink),
}

/// A counter site in a pattern: `c{>=5}`.
pub struct PatCounter {
    name: Name,
    count: PatCount,
}

/// The state of a counter.
pub enum PatCount {
    EqInt(u64),
    GeInt(u64),
    EqVar(Name),
    ModPlus(u64),
    ModMinus(u64),
}

/// A link site in a pattern: `s{x}[.]`.
pub struct PatLink {
    state: Option<PatState>,
    link: Option<PatBinding>,
}

/// A binding state for link sites: `[.]`, `[#]`, ...
pub enum PatBinding {
    Bound,
    Free,
    Any,
    Numbered(u64),
    Named(Stub),  // TODO: better !
}

/// An internal state for link sites: `{x}` or `{#}`.
pub enum PatState {
    State(Name),
    Any,
}


// --- Section 2.3: Rules ----------------------------------------------------

/// A rule, either in edit or chemical notation.
pub enum Rule {
    Chem(RuleChem),
    Edit(RuleEdit),
}

/// A token in a rule: `-1E-6 ATP`.
pub struct RuleToken {
    token: Name,
    count: AlgExpr,
}

// --- Section 2.3.1: Arrow notation -----------------------------------------

/// A rule in chemical notation: `A(x[_]) -> A(x[.]) @ 0.001`.
pub enum RuleChem {
    Forward(ChemForward),
    Reverse(ChemRev),
    Ambi(ChemAmbi),
    AmbiRev(ChemAmbiRev)
}

/// A non-reversible rule in chemical notation.
pub struct ChemForward {
    label: Option<Label>,
    before: Pattern,
    after: Pattern,
    tokens: Vec<RuleToken>,
    rate: AlgExpr,
}

/// A reversible rule in chemical notation.
pub struct ChemRev {
    label: Option<Label>,
    before: Pattern,
    after: Pattern,
    tokens: Vec<RuleToken>,
    rate: AlgExpr,
    revrate: AlgExpr,
}

/// An ambiguous rule in chemical notation.
pub struct ChemAmbi {
    label: Option<Label>,
    before: Pattern,
    after: Pattern,
    tokens: Vec<RuleToken>,
    unirate: AlgExpr,
    birate: AlgExpr
}

/// An ambiguous reversible rule in chemical notation.
pub struct ChemAmbiRev {
    label: Option<Label>,
    before: Pattern,
    after: Pattern,
    tokens: Vec<RuleToken>,
    unirate: AlgExpr,
    birate: AlgExpr,
    revrate: AlgExpr,
}

// --- Section 2.3.2

/// A rule in edit notation: `A(x[_/.]) @ 0.001`.
pub struct RuleEdit; // TODO


// --- Section 2.4.1: Variables ----------------------------------------------

pub struct Variable {
    label: Label,
    value: AlgExpr, // TODO
}

pub struct Plot {
    name: Name,
}

pub struct Observable {
    label: Label,
    value: AlgExpr, // TODO
}


// --- Section 2.4.2: Signatures ---------------------------------------------

// ----- Algebraic expression

/// A comparison operator: `>`, `=` or `<`.
pub enum OpComp {
    Lt,
    Gt,
    Eq
}

/// A binary operator: `+`, `-`, `*`, `/`, `^` or `mod`.
pub enum OpBin {
    Plus,
    Minus,
    Times,
    Div,
    Pow,
    Mod,
}

/// A unary operator: `log`, `exp`, `sin`, `cos`, `tan`, `sqrt`.
pub enum OpUn {
    Log,
    Exp,
    Sin,
    Cos,
    Tan,
    Sqrt,
}

/// A defined algebraic constant: `pi`.
pub enum AlgConstant {
    Pi,
}

/// A reserved algebraic variable: `E`, `|X|`, ...
pub enum AlgReserved {
    /// Productive events since simulation start: `E`.
    ProductiveEvents,
    /// Number of null events: `E-`.
    NullEvents,
    /// Simulated physical time: `T`.
    PhysicalTime,
    /// CPU time since simulation start: `Tsim`.
    CpuTime,
    /// Concentration of token: `|X|`.
    Concentration(Name),
    // /// Occurrences of pattern: `|A(b[.])|`.
    // Occurrences(Box<Pattern>), // TODO
    /// Infinity: `inf`.
    Inf,
}

/// A boolean expression.
pub enum BoolExpr {
    Comparison(Box<AlgExpr>, OpComp, Box<AlgExpr>),
    Or(Box<BoolExpr>, Box<BoolExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Not(Box<BoolExpr>),
    Boolean(bool),
}

/// An algebraic expression.
pub enum AlgExpr {
    Float(f64),
    Constant(AlgConstant),
    Variable(Label),
    Reserved(AlgReserved),
    Binary(Box<AlgExpr>, OpBin, Box<AlgExpr>),
    Unary(OpUn, Box<AlgExpr>),
    Max(Box<AlgExpr>, Box<AlgExpr>),
    Min(Box<AlgExpr>, Box<AlgExpr>),
    Test(Box<BoolExpr>, Box<AlgExpr>, Box<AlgExpr>),
}

// ----- Agent signature

/// An agent declaration signature: `A(s{s1, s2}, b[b.A])`.
pub struct Signature {
    name: Name,
    sites: Vec<SigSite>,
}

/// An agent declaration signature site.
pub enum SigSite {
    Counter(SigCounter),
    Link(SigLink),
}

///
pub struct SigCounter {
    name: Name,
    init: u64,
    limit: u64,
}

pub struct SigLink {
    name: Name,
    states: Vec<SigState>,
    bindings: Vec<SigBinding>,
}

pub struct SigState {
    name: Name,
}

pub struct SigBinding {
    stub: Stub,
}

// --- Section 2.4.4: Parameters ---------------------------------------------

// --- Section 2.4.3: Initial conditions -------------------------------------

pub enum Initial {
    Mixture(AlgExpr, Pattern),
    Token(AlgExpr, Token),
}

// --- Section 2.4.5: Tokens -------------------------------------------------

pub struct Token {
    name: Name,
}

// --- Section 2.5: Intervention directives ----------------------------------

pub enum StringExpr {
    Alg(Box<AlgExpr>),
    String(String),
    Concat(Box<StringExpr>, Box<StringExpr>),
}

pub struct Intervention {
    alarm: Option<f64>,
    test: Option<BoolExpr>,
    effects: Vec<Effect>,
    repeat: Option<BoolExpr>,
}

pub enum Effect {
    Add(AlgExpr, Pattern),
    Del(AlgExpr, Pattern),
    Assign(Name, AlgExpr),
    Snapshot(StringExpr),
    Stop(StringExpr),
    Din(StringExpr, bool),
    Track(Label, bool),
    Update(Name, AlgExpr),
    PlotEntry,
    Print(StringExpr, StringExpr),
    SpeciesOff(StringExpr, Pattern, bool),
}
