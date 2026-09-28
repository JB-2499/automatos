pub enum Estado {
    INICIO,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
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

    pub fn converter(estado: &Estado) -> u32 {
        match estado {
            Estado::A0 => 0,
            Estado::A1 => 1,
            Estado::A2 => 2,
            Estado::A3 => 3,
            Estado::A4 => 4,
            Estado::A5 => 5,
            Estado::A6 => 6,
            Estado::A7 => 7,
            Estado::A8 => 8,
            Estado::A9 => 9,
            _ => 10,
        }
    }

    pub fn match_letra(letra: &u32) -> Estado {
        match letra {
            0 => Estado::A0,
            1 => Estado::A1,
            2 => Estado::A2,
            3 => Estado::A3,
            4 => Estado::A4,
            5 => Estado::A5,
            6 => Estado::A6,
            7 => Estado::A7,
            8 => Estado::A8,
            9 => Estado::A9,
            _ => Estado::ERRO,
        }
    }

    pub fn processar(&mut self, letra: u32) {
        match self.estado {
            Estado::INICIO => {
               self.estado = Self::match_letra(&letra);
            }

            Estado::ERRO => {
            }

            _ => {
                if Self::converter(&Self::match_letra(&letra)) < Self::converter(&self.estado) {
                    self.estado = Estado::ERRO;
                } else {
                    self.estado = Self::match_letra(&letra);
                }
            }
        }
    }

    pub fn validar(&self) -> bool {
        match self.estado {
            Estado::ERRO => false,
            Estado::INICIO => false,
            _ => true,
        }
    }

    pub fn consumir(&mut self, fita: u32) -> bool {
        for letra in fita.to_string().chars() {
            self.processar(letra.to_digit(10).unwrap());
        }

        self.validar()
    }
}
