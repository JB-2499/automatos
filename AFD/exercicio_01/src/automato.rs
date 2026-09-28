pub enum Estado {
    INICIO,
    VALIDO,
    ERRO,
}

pub struct Automato {
    estado: Estado,
}

impl Automato {
    pub fn new() -> Self {
        Self {
            estado: Estado::INICIO,
        }
    }

    pub fn processar(&mut self, letra: char) {
        match self.estado {
            Estado::INICIO => {
                if letra.is_ascii_lowercase() || letra == '_' {
                    self.estado = Estado::VALIDO;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::VALIDO => {
                if letra.is_ascii_lowercase()
                    || letra.is_ascii_digit()
                    || letra == '_' 
                {
                    self.estado = Estado::VALIDO;    
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::ERRO => {
            }
        }
    }

    pub fn validar(&self) -> bool {
        matches!(self.estado, Estado::VALIDO)
    }

    pub fn consumir(&mut self, entrada: &str) -> bool {
        for letra in entrada.chars() {
            self.processar(letra);
        }

        self.validar()
    }
}
