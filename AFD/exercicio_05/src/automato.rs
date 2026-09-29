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

    pub fn igual(letra: char, carac: char) -> bool {
            return letra.to_ascii_lowercase() == carac.to_ascii_lowercase();
    }

    pub fn processar(&mut self, letra: char) {
        match self.estado {
            Estado::INICIO => {
                if Self::igual(letra, 'i') {
                    self.estado = Estado::I;
                } else if Self::igual(letra, 'e') {
                    self.estado = Estado::E;
                } else if Self::igual(letra, 'w') {
                    self.estado = Estado::W;
                } else if Self::igual(letra, 'r') {
                    self.estado = Estado::R;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::I => {
                if Self::igual(letra, 'f') {
                    self.estado = Estado::IF;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::IF => {
                self.estado = Estado::ERRO;
            }

            Estado::E => {
                if Self::igual(letra, 'l') {
                    self.estado = Estado::EL;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::EL => {
                if Self::igual(letra, 's') {
                    self.estado = Estado::ELS;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::ELS => {
                if Self::igual(letra, 'e') {
                    self.estado = Estado::ELSE;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::ELSE => {
                self.estado = Estado::ERRO;
            }

            Estado::W => {
                if Self::igual(letra, 'h') {
                    self.estado = Estado::WH;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::WH => {
                if Self::igual(letra, 'i') {
                    self.estado = Estado::WHI;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::WHI => {
                if Self::igual(letra, 'l') {
                    self.estado = Estado::WHIL;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::WHIL => {
                if Self::igual(letra, 'e') {
                    self.estado = Estado::WHILE;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::WHILE => {
                self.estado = Estado::ERRO;
            }

            Estado::R => {
               if Self::igual(letra, 'e') {
                   self.estado = Estado::RE;
               } else {
                   self.estado = Estado::ERRO;
               }
            }

            Estado::RE => {
                if Self::igual(letra, 't') {
                    self.estado = Estado::RET;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::RET => {
                if Self::igual(letra, 'u') {
                    self.estado = Estado::RETU;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::RETU => {
                if Self::igual(letra, 'r') {
                    self.estado = Estado::RETUR;
                } else {
                    self.estado = Estado::ERRO;
                }
            }

            Estado::RETUR => {
               if Self::igual(letra, 'n') {
                   self.estado = Estado::RETURN;
               } else {
                   self.estado = Estado::ERRO;
               }
            }

            Estado::RETURN => {
                self.estado = Estado::ERRO;
            }

            Estado::ERRO => {
            }
        }
    }

    pub fn validar(&self) -> bool {
        matches!(self.estado, Estado::IF) 
            || matches!(self.estado, Estado::ELSE) 
            || matches!(self.estado, Estado::WHILE)
            || matches!(self.estado, Estado::RETURN)
    }

    pub fn consumir(&mut self, fita: String) -> bool {
        for letra in fita.chars() {
            self.processar(letra);
        }

        self.validar()
    }
}
