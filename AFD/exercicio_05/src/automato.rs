pub enum Estado {
    INICIO,
    I,
    IF,
    E,
    EL,
    ELS,
    ELSE,
    W,
    WH,
    WHI,
    WHIL,
    WHILE,
    R,
    RE,
    RET,
    RETU,
    RETUR,
    RETURN,
    ERRO,
}

pub struct Automato {
    estado: Estado,
}

impl Automato {
    pub fn new () -> Self {
        Automato {
            estado: Estado::INICIO,
        }
    }

    pub fn processar(&mut self, letra: char) {
    }

    pub fn validar(&self) -> bool {
    }

    pub fn consumir(&mut self, fita: String) -> bool {
        for letra in fita.chars() {
            self.processar(letra);
        }

        self.validar()
    }
}
