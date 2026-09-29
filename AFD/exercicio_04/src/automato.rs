pub enum Estado {
    INICIO,
    SINAL,
    INTEIRO,
    PONTO,
    DECIMAL,
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
        match self.estado {
            Estado::INICIO => {
                if letra.is_ascii_digit() {
                    self.estado = Estado::INTEIRO;
                } else if letra == '+' || letra == '-' {
                    self.estado = Estado::SINAL;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::SINAL => {
                if letra.is_ascii_digit() {
                    self.estado = Estado::INTEIRO;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::INTEIRO => {
                if letra.is_ascii_digit() {
                    self.estado = Estado::INTEIRO;
                } else if letra == '.' {
                    self.estado = Estado::PONTO;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::PONTO => {
                if letra.is_ascii_digit() {
                    self.estado = Estado::DECIMAL;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::DECIMAL => {
                if letra.is_ascii_digit() {
                    self.estado = Estado::DECIMAL;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::ERRO => {
            }
        }
    }

    pub fn validar(&self) -> bool {
        matches!(self.estado, Estado::INTEIRO) || matches!(self.estado, Estado::DECIMAL)
    }

    pub fn consumir(&mut self, fita: String) -> bool {
        for letra in fita.chars() {
            self.processar(letra);
        }

        self.validar()
    }
}
