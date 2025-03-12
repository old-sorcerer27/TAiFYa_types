use std::fmt::{self, Display};
use rand::{rng, Rng};

#[derive(Clone)]
pub struct Grammar{
    pub terminals: Vec<Symbol>,
    pub nonterminals: Vec<Symbol>,
    pub rules: Vec<Rule>,
    pub initial_state: Symbol
}

impl fmt::Debug for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Terminals: {:?}\nNonterminals: {:?}\nRules: {:?}\nInitial State: {:?}",
            self.terminals,
            self.nonterminals,
            self.rules,
            self.initial_state
        )
    }
}

impl Grammar {
    pub fn generate_random() -> Self {
        let mut rng = rng();

        let terminals_count = rng.random_range(5..10);
        let nonterminals_count = rng.random_range(5..10);
        let rules_count = rng.random_range(20..25);


        let mut terminals = Vec::with_capacity(terminals_count as usize);
        let mut nonterminals = Vec::with_capacity(nonterminals_count as usize);
        let mut rules = Vec::with_capacity(rules_count as usize);

        for _ in 0..terminals_count {
            let terminal = Symbol::new(rng.random_range(97..122) as u8 as char,SymbolType::Terminal);
            terminals.push(terminal);
        }

        for _ in 0..nonterminals_count {
            let nonterminal = Symbol::new(rng.random_range(65..90) as u8 as char,SymbolType::Nonterminal);
            nonterminals.push(nonterminal);
        }

        for _ in 0..rules_count {
            let left = nonterminals[rng.random_range(0..nonterminals.len())].clone();
            let mut right = Chain::default();
            let empty_line = rng.random_bool(0.25);

            if empty_line{
                let symbol = Symbol{liter: 'ε', symbol_type:SymbolType::EmptyLine};
                right.add_symbol(symbol);
            } else {
                let num_symbols = rng.random_range(1..=4); 
                let mut has_nonterminal = true;   
                for _ in 0..num_symbols {
                    if  has_nonterminal  {
                        let random_nonterminal_index = rng.random_range(0..nonterminals.len());
                        let symbol = nonterminals[random_nonterminal_index].clone();
                        right.add_symbol(symbol);
                        has_nonterminal = false;
                    } else {
                        let random_terminal_index = rng.random_range(0..terminals.len());
                        let symbol = terminals[random_terminal_index].clone();
                        right.add_symbol(symbol);
                    }
                }
            }
            let rule = Rule { left, right };
            rules.push(rule);
        }

        let initial_state = nonterminals[0].clone();

        Grammar {
            terminals,
            nonterminals,
            rules,
            initial_state,
        }
    }

    pub fn get_terminals (&self) -> String {
        let mut term = String::new();
        for t in &self.terminals {
            term += &t.liter.to_string();
            term += " ";
        }
        return term
    }

    pub fn get_nonterminals (&self) ->  String {
        let mut nterm = String::new();
        for nt in &self.nonterminals {
            nterm += &nt.liter.to_string();
            nterm += " ";
        }
        return nterm
    }
    


    pub fn generate_line (&self) -> (Vec<Chain>, Vec<Rule>, String) {
        let mut cur_noterm = self.initial_state.clone();
        let mut chains = Vec::from(Chain::new(cur_noterm.clone()));
        let mut chain_string  = cur_noterm.to_string();
        let mut rules=  vec![Rule {
            left: self.initial_state.clone(), 
            right: Chain::new(self.initial_state.clone())
        }];
       
        for  _i in 0..10 {
                match self.get_random_nonterminal_rule(&cur_noterm) {
                    Ok(r) => {
                        chains.push(r.gen_chain(chains[chains.len() - 1].clone()));
                        rules.push(r.clone());
                        chain_string.push_str(&format!(" -> {}", chains[chains.len() - 1].clone().to_string()));
                        match r.right.get_nonterminal() {
                            Ok(nt) => {
                                cur_noterm = nt.0;
                            }
                            Err(_) => {
                                break;
                            },
                        }
                    },
                    Err(_) => {
                        break;
                    },
                }
            }
        
        return (chains, rules, chain_string);
    }

    
    pub fn get_random_nonterminal_rule (&self, nt:&Symbol ) -> Result<Rule, SymbolTypesError>{
        let mut rng = rng();
        let mut rules:Vec<Rule> = vec![];
        for r in &self.rules {
            if r.left.liter == nt.liter {
                rules.push(r.clone());
            }
        }
        if rules.len() > 0 {
            return Ok(rules[rng.random_range(0..rules.len())].clone()); 
        } else {
            return Err(SymbolTypesError::RightError);
        }
    }

    pub fn get_nonterminal_rule (&self, nt:&Symbol) -> Result<Rule, SymbolTypesError>{
        for r in &self.rules {
            if r.left.liter == nt.liter {
                return Ok(r.clone());
            }
        }
        return Err(SymbolTypesError::RightError);
    }
    

    pub fn get_nonterminal_rules (&self, nt:&Symbol) -> Result<Vec<Rule>,SymbolTypesError>{
        let mut rules:Vec<Rule> = vec![];
        for r in &self.rules {
            if r.left.liter == nt.liter {
                rules.push(r.clone());
            }
        }
        if rules.len() > 0 {
            return Ok(rules);
        } else {
            return Err(SymbolTypesError::RightError);
        }
    }

}

#[derive(Clone, Debug)]
pub struct Symbol{
    pub liter: char,
    pub symbol_type:SymbolType, 
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.liter)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.liter == other.liter && self.symbol_type == other.symbol_type
    }
}

impl From<Symbol> for Vec<Symbol> {
    fn from(symbol: Symbol) -> Vec<Symbol> {
        vec![symbol]
    }
}

impl Symbol {
    pub fn new(liter: char, symbol_type:SymbolType) -> Self {
        Self { liter, symbol_type }
    }
}

pub fn is_terminal(symbol: &Symbol) -> bool {
    match symbol.symbol_type {
        SymbolType::Terminal => true,
        _ => false,
    }
}

pub fn is_nonterminal(symbol: &Symbol) -> bool {
    match symbol.symbol_type {
        SymbolType::Nonterminal => true,
        _ => false,
    }
}

pub fn is_empty_line(symbol: &Symbol) -> bool {
    match symbol.symbol_type {
        SymbolType::EmptyLine => true,
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    Terminal,
    Nonterminal,
    EmptyLine 
}


#[derive(Debug)]
pub enum SymbolTypesError {
    LeftError,
    RightError,
    NoSymbolError
}

#[derive(Clone, Debug)]
pub struct Chain{
    pub string: Vec<Symbol>
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            string: Vec::new(), 
        }
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut st = String::new();
        for c in  self.string.clone() {
            st += &c.liter.to_string()
        }
        write!(f, "{}", st)
    }
}


impl Chain {
    pub fn new(s:Symbol) -> Self {
        Self {
            string: Vec::from(s),
        }
    }

    pub fn new_empty(s:Symbol) -> Self {
        Self {
            string: Vec::from(s),
        }
    }

    pub fn to_string (&self) -> String {
        let mut str = String::new();
        for s in &self.string {
            str += &s.liter.to_string();
        }
        return str;
    }

    pub fn get_nonterminal(&self) -> Result<(Symbol, usize),SymbolTypesError> {
        for (index, s) in self.string.iter().enumerate() {
            if let SymbolType::Nonterminal = s.symbol_type {
                return Ok((s.clone(), index));
            }
                
        }
        Err(SymbolTypesError::NoSymbolError)
    }
    

    pub fn add_symbol (&mut self, s:Symbol) {
        self.string.push(s);
    }

    pub fn add_chain (mut self, mut c:Chain) {
        self.string.append(&mut c.string);
    }
}

impl From<Chain> for Vec<Chain> {
    fn from(chain: Chain) -> Vec<Chain> {
        vec![chain]
    }
}


#[derive(Clone, Debug)]
pub struct Rule{
    pub left: Symbol,
    pub right: Chain
}

impl Rule {
    pub fn gen_chain(&self, chain:Chain) -> Chain{
        let mut new_chain = Chain::default();
        for s in chain.string{
            if s != self.left {
                new_chain.string.push(s);
            }
            else {
                for rs in self.right.string.clone() {
                    new_chain.string.push(rs);
                }
            }
        }
        return new_chain;
    }

    pub fn get_nonterminal(&self) ->  Result<Symbol, SymbolTypesError>{
        for s in self.right.string.clone() {
            if s.symbol_type.eq(&SymbolType::Nonterminal) {
                return Ok(s);
            }
        }
        return Err(SymbolTypesError::NoSymbolError)
    }
}

#[derive(Clone, Debug)]
pub struct ExRule{
    pub left: Chain,
    pub right: Chain
}

#[derive(Clone)]
pub struct Table{
    pub table: Vec<State>
}

impl Default for Table {
    fn default() -> Self {
        Self {
            table: Vec::new(), 
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for (index, r) in self.table.iter().enumerate() {
            str += &format!("{}\t{}",index, r);
        }
        writeln!(f, "{}", str)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for (index, r) in self.table.iter().enumerate() {
            str += &format!("{}\t{}",index, r);
        }
        writeln!(f, "{}", str)
    }
}

impl Table {
    pub fn new(start: State) -> Self {
        Self { table: Vec::from(start) }
    }
}

#[derive(Clone)]
pub struct State{
    pub input_row: Chain,
    pub magazine_state: Chain,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_row: Chain::default(), 
            magazine_state: Chain::default()
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "q\t {:?}\t {:?}\n",
            self.input_row,
            self.magazine_state
        )
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "q\t {}\t {}\n",
            self.input_row.to_string(),
            self.magazine_state.to_string()
        )
    }
}

impl From<State> for Vec<State> {
    fn from(row: State) -> Vec<State> {
        vec![row]
    }
}